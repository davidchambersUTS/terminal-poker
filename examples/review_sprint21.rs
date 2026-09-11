//! Executable evidence: authorized Practice hand through the production selector.
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};
use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};
use terminal_poker::{
    local_practice::PracticeSession,
    ui::{
        action_selection::ActionSelection,
        multiway_review::{MultiwayReviewView, ShowdownStage},
        render::{render_practice_view_with_actions, RaiseSizingView},
    },
};

fn capture(
    root: &Path,
    name: &str,
    view: &MultiwayReviewView,
    focus: &ActionSelection,
    width: u16,
    height: u16,
    terminal: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut display = Terminal::new(TestBackend::new(width, height))?;
    let raise = view
        .legal_actions
        .as_ref()
        .and_then(|legal| legal.min_bet_to.or(legal.min_raise_to))
        .map(|target| RaiseSizingView {
            target,
            minimum: target,
            maximum: view.legal_actions.as_ref().unwrap().all_in_to - 1,
            preset_index: None,
        });
    let started = Instant::now();
    display.draw(|frame| {
        render_practice_view_with_actions(
            frame,
            view,
            raise,
            0,
            terminal.then_some(ShowdownStage::Award),
            focus,
        )
    })?;
    let render_us = started.elapsed().as_micros();
    let cells: Vec<Value> = display
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| {
            json!({
                "symbol": cell.symbol(), "foreground": format!("{:?}", cell.fg),
                "background": format!("{:?}", cell.bg), "modifiers": cell.modifier.bits(),
            })
        })
        .collect();
    let data = json!({"renderer":"render_practice_view_with_actions", "backend":"ratatui::backend::TestBackend",
        "build_id":"sprint21-candidate", "hand_id":view.hand_id, "checkpoint":name,
        "width":width,"height":height,"cells":cells,"render_us":render_us});
    fs::write(
        root.join(format!("{name}.json")),
        serde_json::to_vec_pretty(&data)?,
    )?;
    // At award the renderer retains the settled pot label; stacks already include it.
    let total: u32 = view.seats.iter().map(|seat| seat.stack).sum::<u32>()
        + if terminal { 0 } else { view.pot_total };
    assert_eq!(total, 900);
    if !terminal {
        assert!(view
            .seats
            .iter()
            .filter(|seat| seat.seat != view.local_seat)
            .all(|seat| !seat.cards_visible));
    }
    Ok(
        json!({"checkpoint":name,"hand_id":view.hand_id,"phase":view.phase.name(),
        "revision":view.protocol.as_ref().map(|p| p.revision),
        "board":view.board.iter().map(|card| format!("{card}")).collect::<Vec<_>>(),
        "pot":view.pot_total,"total":total,"selected":focus.selected(),
        "seats":view.seats.iter().map(|seat| json!({"seat":seat.seat.as_u8(),"stack":seat.stack,
            "contribution":seat.contribution,"awarded":seat.awarded})).collect::<Vec<_>>() }),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "output/sprint21/captures".into());
    let root = Path::new(&root_arg);
    fs::create_dir_all(root)?;
    let mut session = PracticeSession::nine_handed_seeded_for_review(100, 21001)?;
    let mut focus = ActionSelection::default();
    let mut frames = vec![capture(
        root,
        "01-deal",
        &session.view(),
        &focus,
        80,
        30,
        false,
    )?];
    let mut actions = Vec::new();
    let mut first = true;
    let mut last_phase = session.view().phase;
    for _ in 0..256 {
        session.current_mut().apply_updates()?;
        if session.current().app().is_terminal() {
            break;
        }
        let view = session.view();
        let actor = view
            .seats
            .iter()
            .find(|seat| seat.to_act)
            .map(|seat| seat.seat.as_u8());
        if session.current().app().client().controls_enabled() {
            let minimum = view
                .legal_actions
                .as_ref()
                .and_then(|legal| legal.min_raise_to.or(legal.min_bet_to));
            focus.sync(&view, minimum, true);
            if first {
                frames.push(capture(
                    root,
                    "02-select-call",
                    &view,
                    &focus,
                    80,
                    30,
                    false,
                )?);
                assert!(focus
                    .handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE))
                    .is_none());
                assert_eq!(focus.selected(), Some(2));
                frames.push(capture(
                    root,
                    "03-select-raise",
                    &view,
                    &focus,
                    80,
                    30,
                    false,
                )?);
                for (width, height) in [(56, 40), (64, 36), (72, 32), (120, 40), (40, 20)] {
                    capture(
                        root,
                        &format!("viewport-{width}x{height}"),
                        &view,
                        &focus,
                        width,
                        height,
                        false,
                    )?;
                }
                first = false;
            }
            let action = focus
                .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
                .ok_or("expected an actionable selection")?;
            actions.push(json!({"actor":actor,"action":format!("{action:?}"),"revision_before":view.protocol.as_ref().map(|p|p.revision)}));
            session.current_mut().submit_local(action)?;
            assert!(focus
                .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
                .is_none());
        } else {
            session.current_mut().step_bot()?;
        }
        session.current_mut().apply_updates()?;
        let view = session.view();
        if view.phase != last_phase && !session.current().app().is_terminal() {
            focus.sync(&view, None, true);
            frames.push(capture(
                root,
                &format!("phase-{}", view.phase.name()),
                &view,
                &focus,
                80,
                30,
                false,
            )?);
            last_phase = view.phase;
        }
    }
    assert!(session.current().app().is_terminal());
    focus.sync(&session.view(), None, true);
    frames.push(capture(
        root,
        "99-award",
        &session.view(),
        &focus,
        80,
        30,
        true,
    )?);
    let history = session.current().safe_history()?;
    let evidence = json!({"build_id":"sprint21-candidate","seed":21001,"observer":0,
        "table_id":history.table_id.0,"hand_id":history.hand_id.0,
        "mapping":"S0 keyboard selector; S1-S8 deterministic passive controllers",
        "frames":frames,"local_actions":actions,"safe_history":history,"privacy_pass":true});
    fs::write(
        root.join("evidence.json"),
        serde_json::to_vec_pretty(&evidence)?,
    )?;
    println!("SPRINT21_REVIEW_PASS frames={} chips=900", frames.len());
    Ok(())
}
