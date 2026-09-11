# Sprint 21 bitmap investigation and refinement

VIS-1 (3) replaces REL-3 (3) under the fallback agreed in Sprint 21 planning.
The owner reports Ghostty works well; Warp and the Codex app terminal have
issues. Versions, fonts and live second-device/VLAN access are unavailable.
REL-3 remains three points of release-gate work; no live rollout success is claimed.

## Experiment and decision

`examples/sprint21_bitmap_spike.rs` encoded the same authorized table capture with
the existing ratatui-image dependency using Halfblocks, Sixel, Kitty and iTerm2.
Twelve samples cover 80x30, 56x40 and 120x40. Assumed cells are 10x20 pixels;
these are offline encoding dimensions, not measured Mac font metrics.

| Protocol, 80x30 | Encode microseconds | Cached draw microseconds | Encoded cell bytes |
|---|---:|---:|---:|
| Halfblocks | 2072 | 17 | 4938 |
| Sixel | 19222 | 3 | 97634 |
| Kitty | 2373 | 16 | 13200 |
| iTerm2 | 1221 | 4 | 113050 |

The source RGBA buffer occupies 1,920,000 bytes. Encoded cell bytes are measured
buffer output, not total process memory. Changing the target size re-encodes the
frame; repeated draws use the cached protocol. These single-run timings are not
input latency or terminal-visible redraw benchmarks. Raw results are retained
in `output/sprint21/bitmap-benchmark.json`.

Decision: retain the shared text table and automatic menu fallback. Encoding
success cannot establish flicker, cleanup, image retention, focus or font scaling
in the reported problem terminals. A terminal-table bitmap implementation is
rejected for this increment because the evidence does not justify that cost.
No new renderer, artwork, appearance settings or child terminal is activated.
VIS-2/3/4 remain conditional, as do the separate-window implementation stories.

## Terminal baseline and refinement

Six installed ConPTY journeys pass through CMD, Windows PowerShell and Git Bash
at 80x30 and 56x40, launched from a temporary directory outside the repository.
They cover arrows, Enter, resize, disabled undersize input, Home, basic palette,
normal exit and Ctrl-C restoration. This establishes the available Windows
baseline, not a certification of every GUI terminal hosting those shells.
Two local fault-injection probes compile the exact production restoration
functions and pass error-unwind and panic cleanup; they are not faults injected
into a live player's installed binary. See `docs/TERMINAL_SUPPORT.md`.

UX-1's unavailable named-host measurements are explicitly retained as rollout
follow-up under REL-3 and final supported-emulator validation under UX-4. The
goal-preserving acceptance boundary is an observed, honest support baseline;
there is no claim that Warp or the Codex app terminal has been repaired.

UX-2 exposed an existing high-severity authority defect: an all-in raise could
bypass closed raising rights. Corrected under existing P-004/ADR 0007, including
client validation, training mask and invariant generator. No policy, wire or
persistence change. Regression proves rejected commands leave state unchanged.
The deployed version retains that defect until an authorized paired upgrade.

No additional forecast points are awarded for this in-slice correctness fix.
Scope stays REL-1/REL-2/UX-1/UX-2/VIS-1 = 16, with 42 conditional points excluded.
