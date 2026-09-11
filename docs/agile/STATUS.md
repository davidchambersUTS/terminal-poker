# Delivery status - 2026-09-11

Sprint 21 is complete: **16/16 accepted**, including planning, scope refinement,
review, retrospective, final PDF visual QA and inactive next recommendation.
Completed REL-1, REL-2, UX-1, UX-2 and VIS-1. REL-3 live Mac/VLAN rollout remains
open under the planned substitution; Warp/Codex issues remain unverified.

Practice/network share legal Left/Right focus and Enter submission. The source
also corrects an existing all-in bypass of closed raising rights. Local gates:
332 passed / 0 failed / 4 existing ignored, fmt and strict Clippy pass; six
installed shell journeys and two restoration probes pass. Source 7a0271de1922
passes all four platforms in Quality run 34567133724.

**841 forecast / 660 accepted / 181 remaining**, plus 42 conditional points.
No next sprint activated. Recommended: finish named-host/rollout checks and
WIN-1 workflow decision before unified visual design. See the Sprint 21 review.
Branch publication is approved; no merge, tag or deployment. The running service
retains the old all-in defect pending an authorized paired upgrade.

## Previously deployed runtime and historical evidence

The installed client connects Host/Join directly to the managed Linux server at
192.168.5.250:6969 with verified TLS. Fresh profiles use that endpoint; old tunnel
defaults migrate. Players need no operator SSH access. The server owns multiple
independent single-table tournaments, open/password lobby access and game state.

The waiting-host correction separates status polling from admission budgets and
sends clean TLS closure after final responses. Two installed Windows clients
waited more than 30 seconds, joined and completed a tournament successfully.

Current runtime baseline: Linux release 5b33d7945d9a0be2; server SHA-256
`93a3a0a5626866bcdc59b17338b9ecca50c5fe22c48411b2ba2aba265571a731`.
Installed Windows client SHA-256
`651b03135ab4ae38ad7d7fe91f6090c498cc0381d3c2aadf7f93862921f166a3`.
Prior complete tests: Windows 321 passed / 0 failed / 4 existing ignored;
Linux 319 / 0 / 3. Strict Clippy and optimized builds passed.

## Source handoff

The owner authorized a fork PR following the [tracking audit](../development/PR_TRACKING_AUDIT.md).
The [explicit file list](../development/PR_FILE_LIST.txt) excludes private/generated
state and historical research/design archives. An isolated source-only snapshot
passed all 321 Windows tests. Portable onboarding and a four-platform Quality
matrix are included. Fork PR #1 was merged; its earlier four-platform checks
passed, but the post-merge Apple Silicon restart test failed intermittently. That
separate reliability follow-up remains open. The lifecycle changes below have
not yet been committed or pushed. The owner confirms the Mac build worked, with
substantial differences between terminal emulators. Exact support coverage and
a complete cross-VLAN player journey remain to be recorded.

Historical sprint reviews and screenshots are retained locally rather than
included in this public checkout. References to those artifacts in older planning
records are labeled local archive references. No release tag/publication is part
of the PR. Public deployment, active-hand crash durability, ARM Linux validation
and multi-table tournament movement remain outside accepted scope.

Current work: Sprint 21, the activated 16-point reliability, rollout, terminal matrix and arrow-action slice. Then rendering/window decisions, unified visual
refresh, packaging, Custom Practice and structured Study. See the full
[2026-09-10 review](BACKLOG_REVIEW_2026-09-10.md).
See [Linux operations](../LINUX_SERVER.md), [ADR 0022](../adr/0022-automatic-lan-tls.md)
and [backlog](BACKLOG.md).

## Dedicated game lifecycle follow-up

Implemented and deployed 2026-09-10 outside a sprint; no new points. Waiting games
expire after 10 empty minutes, running games after 15 all-disconnected minutes,
and finished games after 5 minutes. Reconnection/registration resets the empty
timer, including visits between sweeps. Authenticated sockets preserve their
reconnect grants; the default credential grace after departure is 15 minutes.
Removed games and their routes/credentials are excluded from the durable registry
before live authority is released. Cleanup write errors retain games and retry.
The existing lobby refreshes automatically every two seconds.

Local operator list/remove/clear-inactive runs through a private 0600 Unix socket.
Running/connected games require an explicit per-game force override. Initial live
cleanup removed five finished games, retaining game 11 (Sneaky Freezeout DC1).
The hand-history SHA-256 was unchanged by cleanup. The final deployment restart
confirmed removed games stayed absent. Game 11 follows normal abandonment expiry.
Private saved-state backups and the previous release remain on the host.

Final fmt/strict Clippy/full tests: Windows 327 passed / 0 failed / 4 existing
ignored; Linux 326 / 0 / 3. Native optimized build/package passed. Tests cover
expiry boundaries, reconnect/brief-visit resets, active-game protection, credential
revocation, failed persistence, recovery and the real local operator socket.
The deployed TLS journey passed delayed two-player joining, password rejection,
cancellation, reconnect and a completed 2,000-chip hand; temporary games were
removed through the operator interface. No player binary update is needed.

Source manifest: `f7ef0d77de50c21bc6be279e5c2270f0a78b46b6d13fea2769a6c0abb36b3062`.
Local evidence is retained under `output/game-lifecycle/` (intentionally untracked).
See [ADR 0023](../adr/0023-game-lifecycle-cleanup.md) and the Linux runbook.
Human retest and source commit/PR remain pending; tracked in active Sprint 21.

## UI and Study planning review

2026-09-11 learning design: [detailed specification](../study/LEARNING_SPECIFICATION.md)
now covers 60 lessons, 180 challenge briefs and ten capstones using the ordinary
table interface. Lesson coverage, local links and selected arithmetic were checked.
The foundations-only first course is a proposed revision for ST-1 review; later
courses remain unestimated. Fixtures, strategy review and learner evaluation are
pending. This documentation work changes no Sprint 21 scope or story acceptance.

Added the owner's Left/Right action selection, bitmap-table/cards investigation,
lobby plus separate table-terminal investigation, and structured lessons/challenges.
Study is split into 13 review-tool points and 34 first-course points. Bitmap/window
implementation is conditional, not an approved rewrite. No sprint started and no
runtime changes made by this planning review.

Shot-clock backlog addition: UX-5 (3, P1), a shrinking countdown bar at the
bottom of the acting player's panel using existing authoritative deadlines.
UI allocation is now 24; no implementation or sprint activation in this update.
