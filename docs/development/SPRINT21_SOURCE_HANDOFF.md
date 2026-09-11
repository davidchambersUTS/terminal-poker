# Sprint 21 source handoff

Candidate changes are prepared in an isolated checkout based on fork main
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

The candidate is local until publication is authorized. Four-platform CI must
run on its exact source revision before REL-1 can be accepted. Publication of the
source branch is separate from merging, tagging or deploying it. The currently
installed server/client are not automatically updated by this handoff.

## Validation and deployment implication

See the Sprint 21 review and `output/sprint21/` for exact final check results and
source/binary identities. The previous local baseline was 328 passing tests;
new selector and rule regressions extend it. Do not treat that old count as the
final candidate result.

The all-in correction changes executable behavior on both client and server.
Deploy the validated paired candidate through the normal upgrade/rollback
runbook when authorized. Until then, the deployed server retains the old all-in
reopening defect; a source/CI acceptance does not claim that deployment fixed it.
