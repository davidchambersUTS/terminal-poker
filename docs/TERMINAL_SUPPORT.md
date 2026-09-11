# Terminal support - Sprint 21

Use Left/Right to select a legal action, then Enter to submit it. The selected
button has a `>` marker as well as a highlighted border. Up/Down changes the
bet/raise amount; 1-5 choose pot-size presets. F/C/R/A remain shortcuts through
the same selector and authoritative validation. Navigation and resize do not
submit actions. Release/repeat Enter events do not submit; an accepted selection
is consumed until authoritative context changes. Reconnect, pending commands and
unsupported viewport sizes disable submission.

The call button shows chips added. Bet/raise shows the total street commitment.
A short all-in call is available on Call as well as All-in. When action has not
reopened, an all-in above the current wager is unavailable and rejected by the
server. This implements the existing P-004 rule, not a new poker policy.

## Observed matrix

| Host | Shell | Evidence | Status |
|---|---|---|---|
| Windows ConPTY on this Windows machine | CMD, Windows PowerShell, Git Bash | Installed candidate launched outside source directory; 80x30 and 56x40, basic-color fallback, navigation, Enter, resize, Home and clean exit | Sprint 21 executable matrix; see review evidence |
| Ghostty on owner's Mac | Not specified | Owner reported that it worked well, 2026-09-11 | Reported working; version/font not supplied, not a fresh certified journey |
| Warp | Not specified | Owner reported issues, 2026-09-11 | Unverified; retain text baseline and investigate exact rendering/input failure |
| Codex app terminal | Embedded shell unspecified | Owner reported issues, 2026-09-11 | Unverified; a tool-controlled ConPTY is not proof of the app's visible renderer |
| Windows Terminal 1.24.11911.0 | Any | Installation version inspected | Installed, not independently certified by the ConPTY harness |

Shells and terminal hosts are different. Passing in CMD does not certify every
application that can host CMD. The headless Fedora server needs no graphical
terminal. No version, font, pixel size, graphics protocol or live cross-VLAN
success is inferred from the owner's report.

## Size and fallback

Table minimums: 80x30, 72x32, 64x36 or 56x40. Larger viewports use the same
composition. Home fits 40x20; Settings and Help require 80x24. Below a table
minimum, the app explains the required size and disables action submission.
`NO_COLOR` selects the basic palette; focus remains visible through the `>` cue.

The production menu queries image capabilities once before normal event reads.
Unsupported graphics use the ordinary text menu. Sprint 21 retains text tables:
bitmap encoding success cannot prove visible image retention, cleanup, flicker,
font scaling or keyboard behavior in Warp, Ghostty or the Codex app terminal.
Collect host/version, font/cell dimensions, viewport, a reproducer and an
authorized screenshot when reporting a discrepancy; omit credentials/private cards.

## Reproduction

The local review harness is `scripts/sprint21_terminal_smoke.py`. It uses pywinpty
and pyte installed under `output/sprint21/python-deps`, creates an isolated
candidate install directory, and records each real ConPTY transcript. Its screen
parser is an observation aid, not a replacement for the named emulator's renderer.
`examples/review_sprint21.rs` separately exercises one complete authorized hand
through the production selector/renderer and records the continuity ledger.
