# Sprint 21 source handoff

Candidate changes are published in an isolated checkout based on fork main
`28ba1bade1ae4e0e79144fa56c4d9c5744c9fd0f`, on
`sprint21/reliability-and-actions`. The user's original working tree and its
uncommitted lifecycle changes remain intact.

## Review scope

- Preserve and share the previously deployed lifecycle cleanup and private
  operator socket, including its new source file and ADR 0023.
- Correct the restart process test's initial-revision timing assumption and add
  a deterministic late-observer test. Fresh authority still starts at zero.
- Share one action selector between Practice and network play. Add visible focus,
  Enter/shortcut submission, bounds, short all-in calls and stale-context guards.
- Enforce the existing no-reopening rule for all-in raises in authority, projected
  client validation, the invariant campaign and training action mask. An all-in
  call remains legal; no wire or persistence format changes.
- Update Help, terminal support, sprint tracking and review documentation.

Local source reconciliation compared all 119 entries in the previously deployed
lifecycle manifest. Before the UI changes, 117 matched exactly; the only two
differences were the Sprint 21 regression-test files. The retained manifest
comparison is evidence of correspondence with the recorded deployment, not a
fresh inspection of the running host.

## Boundaries

The explicit source file list excludes `.env`, SSH known-host state, generated
evidence, private deployment keys, runtime profiles, credentials and saved games.
The checked-in TLS fixture key is deliberately public test data. The public LAN
CA remains required build input. No production credential is part of the patch.

The owner explicitly approved branch commits, push and four-platform CI on
2026-09-11. Source commit 7a0271de1922fd8fd509ee03db4bb01a1e305782 is published
on the named branch. Quality run 34567133724 passes on all four platforms.
Final documentation publication uses the same Rust build inputs; its CI receipt
is checked before final handoff. No PR, merge, tag or deployment is authorized.

## Validation and deployment implication

Local full gate: 332 passed / 0 failed / 4 existing ignored; formatting, strict
Clippy and optimized client/server builds pass. Six installed ConPTY journeys
and two restoration fault probes pass. The final ten-page PDF is visually
inspected and SHA-bound; see the Sprint 21 review for the retained local bundle.

The all-in correction changes executable behavior on both client and server.
Deploy the validated paired candidate through the normal upgrade/rollback
runbook when authorized. Until then, the deployed server retains the old all-in
reopening defect; a source/CI acceptance does not claim that deployment fixed it.
