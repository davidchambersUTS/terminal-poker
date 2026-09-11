//! Presentation-only action focus shared by Practice and network play.
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::multiway_review::MultiwayReviewView;
use crate::game::actions::Action;
use crate::game::multiway::MultiwayLegalActions;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Context {
    hand: String,
    revision: Option<u64>,
    legal: MultiwayLegalActions,
    wager: u32,
    contribution: u32,
}

/// Button order is Fold, Check/Call, Bet/Raise, All-in.
#[derive(Debug, Default)]
pub struct ActionSelection {
    context: Option<Context>,
    actions: [Option<Action>; 4],
    selected: Option<usize>,
    submitted: bool,
}

impl ActionSelection {
    pub fn sync(&mut self, view: &MultiwayReviewView, raise_to: Option<u32>, viewport_ok: bool) {
        let context = view
            .legal_actions
            .as_ref()
            .filter(|_| {
                viewport_ok
                    && view
                        .client
                        .as_ref()
                        .is_none_or(|client| client.controls == "ENABLED")
            })
            .map(|legal| Context {
                hand: view.hand_id.clone(),
                revision: view.protocol.as_ref().map(|protocol| protocol.revision),
                legal: legal.clone(),
                wager: view.current_wager,
                contribution: view
                    .seats
                    .iter()
                    .find(|seat| seat.seat == view.local_seat)
                    .map_or(0, |seat| seat.contribution),
            });
        if self.context != context {
            self.selected = None;
            self.submitted = false;
        }
        self.context = context;
        self.actions = [None; 4];
        let Some(context) = &self.context else {
            return;
        };
        let legal = &context.legal;
        self.actions[0] = legal.can_fold.then_some(Action::Fold);
        self.actions[1] = if legal.can_check {
            Some(Action::Check)
        } else if let Some(amount) = legal.call_amount {
            Some(Action::Call(amount))
        } else if legal.all_in_to > context.contribution && legal.all_in_to <= context.wager {
            Some(Action::AllIn(legal.all_in_to))
        } else {
            None
        };
        self.actions[2] = raise_to.and_then(|amount| {
            if amount >= legal.all_in_to {
                return None;
            }
            if legal.min_bet_to.is_some_and(|minimum| amount >= minimum) {
                Some(Action::Bet(amount))
            } else if legal.raise_reopened
                && legal.min_raise_to.is_some_and(|minimum| amount >= minimum)
            {
                Some(Action::Raise(amount))
            } else {
                None
            }
        });
        self.actions[3] = (legal.all_in_to > context.contribution
            && (legal.all_in_to <= context.wager || legal.raise_reopened))
            .then_some(Action::AllIn(legal.all_in_to));
        if self
            .selected
            .is_none_or(|index| self.actions[index].is_none())
        {
            // A new turn never focuses the separate All-in button automatically.
            self.selected = [1, 0, 2]
                .into_iter()
                .find(|&index| self.actions[index].is_some());
        }
    }

    pub fn selected(&self) -> Option<usize> {
        if self.submitted {
            None
        } else {
            self.selected
        }
    }

    pub fn enabled(&self) -> [bool; 4] {
        self.actions
            .map(|action| action.is_some() && !self.submitted)
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        if key.kind == KeyEventKind::Release || self.submitted {
            return None;
        }
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                let current = self.selected?;
                let direction = if key.code == KeyCode::Right { 1 } else { 3 };
                self.selected = (1..=4)
                    .map(|step| (current + step * direction) % 4)
                    .find(|&index| self.actions[index].is_some());
                None
            }
            KeyCode::Enter | KeyCode::Char('f' | 'F' | 'c' | 'C' | 'r' | 'R' | 'a' | 'A')
                if key.kind == KeyEventKind::Press =>
            {
                let index = match key.code {
                    KeyCode::Char('f' | 'F') => 0,
                    KeyCode::Char('c' | 'C') => 1,
                    KeyCode::Char('r' | 'R') => 2,
                    KeyCode::Char('a' | 'A') => 3,
                    _ => self.selected?,
                };
                let action = self.actions[index]?;
                self.submitted = true;
                Some(action)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::local_practice::PracticeSession;
    use crossterm::event::KeyModifiers;

    fn view() -> MultiwayReviewView {
        let mut session = PracticeSession::nine_handed_seeded_for_review(100, 21001).unwrap();
        while !session.current().app().client().controls_enabled() {
            assert!(session.current_mut().step_bot().unwrap());
            session.current_mut().apply_updates().unwrap();
        }
        session.view()
    }
    fn key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind)
    }

    #[test]
    fn navigation_never_submits_and_enter_is_consumed_once_per_context() {
        let view = view();
        let mut focus = ActionSelection::default();
        focus.sync(&view, Some(10), true);
        assert_eq!(focus.selected(), Some(1));
        for kind in [KeyEventKind::Release, KeyEventKind::Repeat] {
            assert_eq!(focus.handle_key(key(KeyCode::Enter, kind)), None);
        }
        assert_eq!(
            focus.handle_key(key(KeyCode::Left, KeyEventKind::Press)),
            None
        );
        assert_eq!(focus.selected(), Some(0));
        assert_eq!(
            focus.handle_key(key(KeyCode::Enter, KeyEventKind::Press)),
            Some(Action::Fold)
        );
        focus.sync(&view, Some(12), true);
        assert_eq!(
            focus.handle_key(key(KeyCode::Enter, KeyEventKind::Press)),
            None
        );
    }

    #[test]
    fn stale_focus_clears_on_new_hand_legal_change_disconnect_or_tiny_viewport() {
        let mut view = view();
        let mut focus = ActionSelection::default();
        focus.sync(&view, Some(10), true);
        focus.handle_key(key(KeyCode::Left, KeyEventKind::Press));
        view.hand_id.push_str("-next");
        focus.sync(&view, Some(10), true);
        assert_eq!(focus.selected(), Some(1));
        focus.sync(&view, Some(10), false);
        assert_eq!(focus.selected(), None);
        assert_eq!(
            focus.handle_key(key(KeyCode::Enter, KeyEventKind::Press)),
            None
        );
        view.client.as_mut().unwrap().controls = "DISABLED".into();
        focus.sync(&view, Some(10), true);
        assert_eq!(focus.enabled(), [false; 4]);
    }

    #[test]
    fn short_all_in_call_and_closed_raise_rights_have_only_legal_choices() {
        let mut view = view();
        let mut focus = ActionSelection::default();
        view.current_wager = 20;
        let legal = view.legal_actions.as_mut().unwrap();
        legal.can_check = false;
        legal.can_fold = true;
        legal.call_amount = None;
        legal.all_in_to = 15;
        legal.min_bet_to = None;
        legal.min_raise_to = None;
        legal.raise_reopened = false;
        focus.sync(&view, None, true);
        assert_eq!(
            focus.handle_key(key(KeyCode::Enter, KeyEventKind::Press)),
            Some(Action::AllIn(15))
        );
        let legal = view.legal_actions.as_mut().unwrap();
        legal.all_in_to = 100;
        legal.call_amount = Some(20);
        focus.sync(&view, Some(40), true);
        assert_eq!(focus.enabled(), [true, true, false, false]);
        focus.handle_key(key(KeyCode::Right, KeyEventKind::Press));
        assert_eq!(focus.selected(), Some(0));
    }

    #[test]
    fn focus_and_amounts_remain_visible_across_supported_viewports_and_basic_color() {
        use crate::ui::{
            platform::{apply_terminal_palette, ColorDepth, ThemeMode},
            render::{render_practice_view_with_actions, RaiseSizingView},
        };
        use ratatui::{backend::TestBackend, Terminal};
        let view = view();
        for (width, height) in [(80, 30), (72, 32), (64, 36), (56, 40), (120, 40)] {
            let mut focus = ActionSelection::default();
            focus.sync(&view, Some(10), true);
            focus.handle_key(key(KeyCode::Right, KeyEventKind::Press));
            assert_eq!(focus.selected(), Some(2));
            let mut display = Terminal::new(TestBackend::new(width, height)).unwrap();
            display
                .draw(|frame| {
                    render_practice_view_with_actions(
                        frame,
                        &view,
                        Some(RaiseSizingView {
                            target: 10,
                            minimum: 4,
                            maximum: 99,
                            preset_index: None,
                        }),
                        0,
                        None,
                        &focus,
                    );
                    let area = frame.area();
                    apply_terminal_palette(
                        frame.buffer_mut(),
                        area,
                        ThemeMode::Ash,
                        ColorDepth::Basic,
                    );
                })
                .unwrap();
            let text: String = display
                .backend()
                .buffer()
                .content
                .iter()
                .map(|cell| cell.symbol())
                .collect();
            assert!(text.contains("> R"), "missing focus at {width}x{height}");
            assert!(
                text.contains("Enter act"),
                "missing input hint at {width}x{height}"
            );
            assert!(text.contains(if width < 72 { "> R 10" } else { "> R RAISE 10" }));
        }
    }
}
