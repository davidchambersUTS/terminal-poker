# Full backlog review - 2026-09-10

Planning only. No sprint activation, runtime changes, commits, deployment or new
acceptance points follow from this review. The owner reports a working Mac build
and substantial terminal-to-terminal variation. Exact emulator/version coverage
and a complete cross-VLAN player journey remain to be recorded.

## Recommendation

Improve the existing player's experience before expanding server topology or
public reach: reliable shared source, usable action selection, measured terminal
support, then visual design and distribution. Keep a single action/state model
so future text, image or window presentation does not duplicate poker rules.
Build Study as a product with authored content and a grading contract, not as a
menu over the existing training harness. Independent table windows, concurrent
independent server games and D2 tournament movement are three separate features.

The recommended next sprint is 16 points: CI reliability 3, lifecycle source
handoff 2, cross-device rollout 3, terminal matrix 3, arrow-action controls 5.
No sprint is active. If the external rollout check is unavailable, use the
3-point bitmap spike as the substitute and retain the unverified rollout gate.

## What changed in the estimates

Historical acceptance remains 644. Completed LAN TLS, server deployment, lobby
access, source PR and lifecycle implementation are not re-estimated as unfinished
feature development. REL-2 estimates only remaining source/artifact reconciliation
and review, not another implementation of lifecycle cleanup.

| Prior remaining allocation | Previous | Revised | Change / mapping |
|---|---:|---:|---|
| Custom Practice | 8 | 13 | CP-1/2/3; explicit bot scope and previously unmapped save/pause/resume |
| Study and Learn placeholder | 13 | 47 | RV 13 retained; structured teaching ST 34 added |
| Client packaging | 8 | 13 | PKG-1/2/3; real desktop artifacts, maintenance, diagnostics |
| Final UX | 5 | 24 | UX-1/2/3/4/5; terminal diagnosis, controls, refresh, validation and shot-clock bar |
| Range Explorer | 8 | 8 | RG-1/2; content gate retained |
| Public Join | 5 | 5 | Existing LAN directory is not public internet discovery |
| D2 tournaments | 34 | 34 | Six remaining stories retained, provisional |
| Public hardening | 34 | 34 | Existing TLS reused; public trust/identity/operations remain |
| New decision spikes | 0 | 6 | VIS-1 and WIN-1 |
| Reliability, rollout and policy gaps | 0 | 13 | REL-1/2/3/4 and POL-1 |
| **Remaining** | **115** | **197** | **+82** |

Base planning forecast is 841 (644 + 197). Conditional bitmap implementation 21
and separate-table-window implementation 21 are excluded. If both are selected,
remaining becomes 239 and total forecast 883. Selecting a graphical client would
require a replacement estimate, not stacking it invisibly onto the bitmap option.

Use Fibonacci estimates as relative size including implementation, focused tests,
integration and documentation. Do not convert them directly to dates or tokens.
The Study estimate is low confidence until ST-1; D2/public estimates are retained
planning allowances and must be split/re-estimated at activation. Investigations
are bounded by their question and evidence, not by pretending their result is known.
No estimates for unlimited content production, a solver or learned-policy research
are hidden inside the first-course allowance.

## Left/Right action selection

UX-2 is a 5-point functional slice shared by Practice and network play. Left/Right
moves visible focus through legal Fold, Check/Call, Bet/Raise and All-in choices;
Enter commits the selected legal action. Show the actual call or total raise
amount. Check replaces Call when nothing is owed; Bet replaces Raise where
appropriate. Disabled choices remain understandable and cannot be submitted.
Existing direct hotkeys can remain as shortcuts under the same validation path.
Up/Down sizing and numeric/preset editing must not conflict with action navigation.

Navigation, focus acquisition and resize never submit an action. Ignore key
release; repeated Enter cannot submit twice. Clear stale selection on a new hand,
turn or changed legal-action set, and disable submission while disconnected or
awaiting acknowledgement. Do not auto-select/submit all-in on focus. Verify short
all-in calls, minimum raises, unavailable raise rights and tiny terminals. Keep
one unified style and automatic capability fallback, not new appearance options.

UX-1 records terminals separately from shells: CMD, PowerShell and Git Bash are
shells; Windows Terminal/classic console and macOS Terminal/iTerm2 are rendering
hosts. Capture exact versions, font/cell dimensions, colour detection, key events,
resize and image support. Treat this as a proposed test matrix, not a claim all
those combinations are already supported. Add a Linux terminal where relevant to
player support; a headless dedicated server needs no graphical terminal.

## Bitmap table and cards

Recommendation: investigate, but do not assume bitmaps solve terminal variability.
Kitty specifies a graphics protocol and iTerm2 documents its image mechanisms and
capability detection. This establishes a feasible rendering route, not consistent
support across every emulator/version. See [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
and [iTerm2 image documentation](https://iterm2.com/documentation-images.html).

VIS-1 (3) compares the current renderer with bitmap cards/table on the same
recorded authorized view: standard/compact sizes, resize, font scaling, redraw,
flicker, retained image cleanup, input latency, memory and unsupported-terminal
fallback. Reuse existing image detection before adding dependencies. Use original
assets and keep readable textual action/amount labels. Concealed cards remain
concealed in every backend and image cache.

Decision choices: retain a consistent text baseline; add automatic bitmap
presentation on a named support matrix; or separately propose an application-owned
graphics window if consistent pixels across arbitrary terminals is the real goal.
My inference is that the third offers greater rendering control but changes product
scope and packaging substantially. None is approved merely by listing the options.
VIS-2/3/4 reserve 21 conditional points for terminal bitmaps, not a desktop rewrite.
UX-3 remains one projection/layout design; image work must consume it rather than
build a second independent UI with different poker semantics.

## Lobby in one terminal, games in other terminals

The interaction pattern is reasonable: keep discovery available while a table is
open, and allow returning to the lobby without ending a game. PokerStars documents
separate table-window layouts (tile/cascade/stack) in its [table settings](https://www.pokerstars.com/help/articles/multi-table-tips/).
That is evidence for the window interaction pattern, not evidence it uses terminal
processes or that OS focus/launch behavior will transfer to this application.

Recommend WIN-1 (3) before implementation. Test one child table on Windows and Mac:
which supported emulator is launched, how it gets a new terminal session, whether
keys work without a click, how launch failure rolls back, and how either window
can close independently. A table must keep working if its lobby closes; the server
remains independent of both. Closing a table means that player's departure under
existing reconnect rules, never termination of the dedicated server.

Use a secure local one-use handoff or private IPC for credentials; never put a
password/reconnect token in shell command strings or visible process arguments.
Do not join twice while parent and child race. Keep diagnostics and profile writes
safe across processes. Decide whether the parent has zero/one/many open tables;
a single new window does not itself authorize simultaneous multi-tabling.

WIN-2/3/5 cover the first independent table-window experience (16 conditional
points). WIN-4 adds five conditional points for several table windows and a clear
attention/session policy. This is unrelated to balancing players across a D2
tournament. Retain the existing in-terminal path until the chosen launch matrix
is reliable; do not scatter OS-specific launch code through game rules.

## Study scope and sequence

Replace the old single 13-point blob with two related capabilities: review tools
(RV-1/2/3, 13) and structured learning (ST-1 through ST-7, 34). Study is currently
disabled in the shell. Existing safe histories and practice authority are reusable
foundations; the research harness is not learner-facing curriculum.

Proposed first course: six lessons and 18 challenges covering legal actions/hand
ranking, position/preflop concepts, pot odds/outs, sizing, board texture, and
short-stack/tournament fundamentals. ST-1 approves exact objectives and difficulty
before content work. ST-6 delivers the first two lessons/six challenges; ST-7
completes four lessons/twelve challenges using the proven format. Its estimate
assumes reuse of that template and reviewed first-course scope; re-estimate if
expert review or content complexity exceeds it.

Each lesson has a stated objective, concise explanation, worked example,
interactive challenge, explanation of the result, and retry/next step. Grade
rules/math questions exactly. Strategy questions must declare assumptions and
accept justified alternatives; do not label one unvalidated action universally
optimal. Provide content provenance and review. Keep only authorized seat views;
future cards and other players' hidden holdings are not a teaching shortcut.

First vertical teaching slice: ST-1/2/3/6 = 21 points. It can ship before RV review
features or full mastery/hint systems. ST-3 contains basic immediate feedback;
ST-4 adds worked hints and targeted remediation. ST-5 adds persistent progress,
attempt history and curriculum migrations. Offline/local progress is sufficient;
accounts, cloud sync, subscriptions, a content CMS, adaptive AI tutors, solvers,
unlimited courses and learned bots are excluded. Custom Practice shares the
scenario/authority boundary but the course does not require learned policies.

## Full disposition of older and unscoped work

| Item | Disposition |
|---|---|
| E0-E4 rules / E5 authority / E6 networking / E7 registry and ring lifecycle | Preserve accepted scope; regressions accompany every new slice, no re-awarded points |
| E8 credentials/history/recovery | Foundations delivered; REL-4 makes active-hand durability explicit; RV exposes authorized history to users |
| E9 D1 tournament | Preserve accepted 55; POL-1 covers the still-present requested stack-floor change |
| E9 D2 | Retain 34 after recovery decision and a demonstrated need for one tournament across tables |
| E10/E11 private hardening | Delivered evidence retained; flaky CI is REL-1, fresh source handoff REL-2, rollout REL-3 |
| E12 player experience | Old remaining rows map to UX, PKG, CP, RV/ST, RG and retained public Join; no duplicate totals |
| DS / LB / LX | Dedicated ownership, local lobby and Linux/LAN slices delivered; stop listing a Pi purchase or player SSH as prerequisites |
| Lifecycle cleanup and earlier UI/showdown fixes | Implemented outside sprints, no retrospective points; retain human retest/regressions and lifecycle source handoff |
| DISC-001 multi-tabling | WIN-1 decides workflow; WIN-4 conditional implementation |
| DISC-002 and distinct bot styles | CP-1/2 for local practice; public bot participation still a policy gate |
| DISC-010 live human-table HUD | Parked, default off; no implementation estimate without fairness/product decision |
| DISC-013 learned policy / mathematical oracle | Separate research programme, inactive and unestimated; needs explicit research objective/evaluation before sizing |
| DISC-016 20-BB floor | POL-1 (2); actual source still enforces the floor |
| Durable Practice pause/save/end | CP-3 (5); online game pause not implied |
| Active-hand crash recovery | REL-4 (3) investigation; implementation not estimated until RPO/storage decision |
| Public account/password recovery | Identity allowance E10.2b (8) is its current envelope; detailed UX/recovery policy to refine there |
| Spectator, chat/moderation, rebuy/re-entry | Icebox; no accepted behavior/scale, unestimated and excluded |
| Additional regions / provable-fairness scheme | Icebox; no requirement for current private LAN milestone, unestimated |
| Native web/mobile / broad GUI rewrite | Parked; VIS-1 may recommend separately sizing a desktop alternative only |
| Real-money product | Outside this product scope |

The next priority change should follow evidence from the first support/control
slice, not a full visual rewrite, new account service, or D2 simply because those
features appear on mature poker clients. No policy research or public release is
required for friends to play reliably on this LAN.

## Shot-clock addition

UX-5 adds 3 P1 points for an authoritative countdown bar at the bottom of the
acting player's panel, including seconds, compact layout and deadline/reconnect
handling. It changes presentation, not server timeout policy. UI becomes 24 points
and base remaining becomes 197; the proposed next 16-point sprint is unchanged.
