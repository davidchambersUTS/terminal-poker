# Sprint 21 review - 2026-09-11

Result: **Done, 16/16 accepted**. Goal achieved: a reproducible shared build with
reliable arrow-key actions and an observed terminal support baseline. Review is
based on executable evidence and final PDF inspection, not a live owner demo.

| Story | Points | Accepted evidence |
|---|---:|---|
| REL-1 | 3 | Deterministic late-observer regression; corrected process assertion; all four CI platforms pass |
| REL-2 | 2 | All 119 manifest entries reconciled; preserved lifecycle source published on the approved fork branch |
| UX-1 | 3 | Six installed Windows ConPTY journeys; documented host/version evidence and explicit unavailable Mac measurements |
| UX-2 | 5 | Shared Practice/network selector, legal focus/amounts, Press-only submission and stale/pending/repeat guards |
| VIS-1 | 3 | Twelve bitmap encoding/cache/resize samples and a documented decision to retain text tables |

REL-3 (3) moved back to rollout work under the planned VIS-1 substitution.
Ghostty is owner-reported working; Warp/Codex issues are not reproduced or fixed.
The refined UX-1 baseline and remaining named-host checks are recorded in the
[investigation](2026-09-11-sprint-21-bitmap-spike.md). No acceptance of live Mac/VLAN
rollout or bitmap/child-window implementations is implied.

## Demonstration and quality

Left/Right moves the visible `>` focus without submitting; Enter sends the legal
action through ordinary authority. A real network tournament uses this selector,
suppresses duplicate/pending commands and reaches a winner with 2,000 chips.
The deterministic review hand runs the production selector and renderer through
42 accepted actions: initial pot 3, call focus, raise-to-4 focus, all streets,
36-chip split pot, and final stacks totaling 900. Other private cards remain
concealed until legitimate showdown. Seven captured frames share one identity.

Local full gate: **332 passed, 0 failed, 4 existing ignored** with locked
all-target/all-feature tests. Formatting, strict Clippy and optimized paired
client/server builds pass. Six installed CMD/PowerShell/Git Bash journeys at
80x30 and 56x40 pass from outside the repository, including normal exit and Ctrl-C.
Two fault probes of the exact production cleanup code pass error/panic restoration.
They are separate probes, not fault injection into the deployed app.

Source commit: `7a0271de1922fd8fd509ee03db4bb01a1e305782`.
[Quality run 34567133724](https://github.com/davidchambersUTS/terminal-poker/actions/runs/34567133724)
passes Linux, Windows, macOS Intel and macOS Apple Silicon: format, lint, tests,
optimized player/server builds and player command checks.
The final documentation publication retains these Rust build inputs; its own CI
receipt is checked before the final handoff. No merge, tag or deployment is made.

## Inspected report

Local deliverable: `output/pdf/sprint-21-review-report.pdf` (10 landscape A4 pages).
SHA-256: `3b47216891445e2c40911ca496816d4a00090d496a3c87a25a33ad0019796696`.
Every final page was rendered at 150 dpi and visually passed after margin
correction. Reopened metadata and text extraction pass. Retained local evidence:
`docs/agile/reports/sprint-21/` contains screenshots, report source, the complete
hand ledger and SHA-bound visual QA. Generated evidence remains local under the
existing source-publication exclusions; this review publishes its audit summary.

## Decisions and release impact

The discovered all-in raise bypass now enforces existing P-004/ADR 0007 in
authority, client validation and training masks. This is a correction, not a
new poker rule; rejected actions are mutation-free. The deployed server still
contains the old defect until a separately authorized paired upgrade.
Between-hand recovery remains the supported contract. Public release is not ready.

Accounting: **841 forecast = 660 accepted + 181 remaining**, with 42 conditional
points excluded. The sprint awards no retrospective implementation points for
previously deployed lifecycle work. The two-point handoff is the accepted work.
The [retrospective](2026-09-11-sprint-21-retrospective.md) contains two owned actions.
The [next recommendation](2026-09-11-sprint-22-recommendation.md) remains inactive.
