# Sprint 21 activation and first reliability slice - 2026-09-11

Historical planning record; superseded by the completed Sprint 21 review.

The owner requested commencement of the recommended next sprint. Activated
REL-1 (3), REL-2 (2), REL-3 (3), UX-1 (3), UX-2 (5): 16 points.
No token budget or parallel agents requested. Start: 05:12 UTC; initial forecast
90-180 active minutes excluding external participant and approval waits.
Acceptance remains 644; remaining remains 197 until review evidence is complete.

## REL-1 diagnosis

Read-only inspection of fork Quality run 34427624828 (merge commit
28ba1bade1ae4e0e79144fa56c4d9c5744c9fd0f) identifies the Apple Silicon failure at
tests/network_process_test.rs:188: initial_revision was 1, expected 0.
The preceding run 34427531513 passed on the source branch.

The first snapshot observed by a reconnecting process is not necessarily the
initial snapshot of the restored authority: another client can already act.
The registry test retains the pre-action revision-zero/fresh-awards assertions.
A new deterministic test executes one legal action before the other observer's
snapshot and proves same new hand, revision one, no awards and chip conservation.
The process test now checks initial_revision < terminal_revision and preserves
new hand identity, table identity, chip total, empty initial awards and no errors.
No production recovery code or checkpoint contract changes.

Initial regression development caught a missing reconnect in the fixture;
corrected before observing the fresh snapshot. The focused regression now passes.
Full local validation results are recorded in output/sprint21/.
Four-platform CI on this patch remains pending; this is not a green remote claim.

## REL-2 initial reconciliation

Compared all 119 files in output/game-lifecycle/source-manifest.json to the current
workspace by SHA-256. 117 match exactly. The only two differences are the files
containing the Sprint 21 regression changes: src/table_registry.rs and
tests/network_process_test.rs. No missing files. Retained machine-readable result:
output/sprint21/lifecycle-reconciliation.json. This establishes local source
correspondence with the recorded deployment manifest, not a fresh remote audit.

Lifecycle source remains uncommitted, including src/server_admin.rs; preserve it
with the existing explicit PR file list. No commit, push, deployment or rerun of
remote workflows was performed. Rollout, terminal matrix and arrow-action work
remain pending. No acceptance points or sprint closure claimed.

Local gate completed: formatting, strict Clippy and all-target/all-feature tests
pass: 328 passed, zero failed, four existing ignored. git diff --check passes.
REL-1 remains Review pending remote CI and sprint review; no points accepted.

## Final planning reconciliation

Owner subsequently required full sprint/ritual closure and authorized branch
commits, push and CI. The planned REL-3-to-VIS-1 substitution was applied because
live Mac/VLAN access was unavailable. UX-1 records the available baseline and
retains missing named-host measurements under REL-3/UX-4. Final acceptance is
16/16, local tests 332/0/4, four-platform source CI green, all ten PDF pages passed.
See 2026-09-11-sprint-21-review.md for final evidence and residual release gates.
