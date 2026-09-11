# Sprint 21 - Reliable shared builds and keyboard actions

Completed 2026-09-11: **16/16 accepted; no active sprint**.
Goal achieved: a reproducible shared build, shared legal arrow-key actions and
an observed terminal support baseline. Forecast: 841 / accepted 660 / remaining
181; 42 conditional points excluded. No token budget or subagents requested.

| Story | Points | State | Evidence |
|---|---:|---|---|
| REL-1 | 3 | Done | Deterministic restart observation regression and green four-platform CI |
| REL-2 | 2 | Done | 119-file manifest reconciliation and approved source branch handoff |
| UX-1 | 3 | Done, observed baseline | Six installed Windows shell journeys; exact Mac evidence explicitly unavailable |
| UX-2 | 5 | Done | Shared legal focus/amounts, Enter and shortcut submission, pending/stale/repeat guards |
| VIS-1 | 3 | Done | Twelve offline bitmap samples; retain text-table decision |

VIS-1 replaces unavailable REL-3 under the planned fallback. REL-3 remains open:
Ghostty worked well per owner; Warp/Codex app terminal issues need exact host
reproductions and a live second-device/VLAN journey. No Mac renderer fix claimed.

Local full gate: 332 passed / 0 failed / 4 existing ignored; fmt, strict Clippy,
release client/server builds, six installed journeys and two restoration probes
pass. Source 7a0271de1922fd8fd509ee03db4bb01a1e305782 passes Quality run
34567133724 on Linux, Windows, macOS Intel and Apple Silicon.

Final 10-page PDF, complete-hand ledger, screenshots and SHA-bound visual QA
are complete. See [review](rituals/2026-09-11-sprint-21-review.md),
[retro](rituals/2026-09-11-sprint-21-retrospective.md),
[investigation/refinement](rituals/2026-09-11-sprint-21-bitmap-spike.md) and
[inactive next recommendation](rituals/2026-09-11-sprint-22-recommendation.md).

Owner authorized branch commits/push/CI. The source and final documentation use
sprint21/reliability-and-actions in the owner fork. Original dirty work is
preserved. No merge/tag/deployment: the running server still needs the paired
all-in legality correction. See DELIVERY_CALIBRATION.md for timing limitations.

---
# Sprint 20 - Linux dedicated server and automatic LAN connection

Completed 2026-09-09: **26/26 accepted; no active sprint**.
Historical closure forecast 759 / accepted 644 / remaining 115. Direct verified TLS on
192.168.5.250:6969 and automatic installed Host/Join are deployed and verified.

Planning review 2026-09-10: revised forecast 841 / accepted 644 / remaining 197,
plus 42 conditional points. The next recommended 16-point candidate is inactive;
see BACKLOG.md and BACKLOG_REVIEW_2026-09-10.md. Historical sprint scope below is unchanged.

## Initial Linux milestone (historical)

Activated 2026-09-09 by the user's request for a Linux dedicated-server sprint.
Goal: build, install and validate on the supplied Fedora 44 x86_64 device, with
repeatable packaging and an operator runbook.

| Story | Points | Acceptance | State |
|---|---:|---|---|
| LX-1 | 3 | Locked native build, source manifest, full Linux gate and checksum | Done |
| LX-2 | 3 | User service package, graceful stop, durable paths, upgrade/rollback | Done |
| LX-3 | 2 | Windows-to-Linux SSH-forwarded game, lifetime/restart proof, inspected PDF | Done |

Eight new Linux deployment points refine newly available hardware work; existing
E12.10 client packaging remains 8. Activation: 741 forecast / 618 accepted / 123
remaining. No token budget requested. No agents requested.

Plan: inspect host; upload only build inputs; install user-local Rust; build/test;
package/install; verify lifecycle and Windows clients over SSH; capture one hand;
produce and inspect PDF; update backlog/status/risks and close this sprint only.

The supplied ThinkPadServer replaces the assumed unavailable Pi for this slice.
Transport stays loopback with SSH forwarding. Direct VLAN game access, public
release, active-hand crash durability, ARM/Pi and client packaging remain separate.
No firewall changes, commits or pushes. Preserve unrelated services/source changes.

Closure: 8/8 accepted; 741 forecast / 626 accepted / 115 remaining. Linux gate
312 passed / 0 failed / 3 existing ignored. Native build/service/restart proof
and Windows-to-Linux remote gameplay pass. User approved the scoped SELinux rule;
it is installed, TCP 7777 forwarding succeeds and unrelated TCP 22 remains denied.
SELinux enforcing; service enabled with lingering. Installed Windows lobby also
verified from outside repo. Final eight-page PDF visually inspected page by page.
No next sprint active. See review (local archive: `rituals/2026-09-09-sprint-20-review.md`).

## Completed extension - direct automatic LAN connection

Owner explicitly extended Sprint 20, clarified direct server listening (no SSH
for players), and supplied network-admin-approved TCP port 6969.

| Story | Points | Acceptance | State |
|---|---:|---|---|
| LX-4 | 8 | Verified TLS listener on 6969, private host keys, bounded admission, trust-negative tests | Done |
| LX-5 | 5 | Automatic TUI Host/Join, migration of old tunnel default, responsive cancel/retry | Done |
| LX-6 | 5 | Paired deployment, native/Windows gates, installed three-shell flow, revised inspected report | Done |

Extension closure: 18/18 accepted, total 26/26. Linux 318 pass / 3 ignored;
Windows 320 pass / 4 ignored; zero failures. Direct TLS journey, installed three
shells, cancellation/retry, service restart and final eight-page visual review pass.
No next sprint active. Final 759 / 644 / 115.

Initial 8-point acceptance remains historical. Extension adds 18 direct private
LAN points; total Sprint 20 scope 26. Activation: 759 forecast / 626 accepted /
133 remaining. Existing public hardening allocation remains for internet trust,
identity, operations and release; no public reach accepted by this LAN slice.
No token budget or subagents requested. ADR 0022 governs direct TLS and trust.
Plan: implement transport + UI; trust/abuse/cancel regression checks; full native
and Windows gates; deploy server/client; verify port 6969 without tunnels; review
one complete hand and installed shell flows; update artifacts and close extension.

## Historical Sprint 19 closure

Activated and completed 2026-09-09 by explicit user request for a full lobby sprint.
21/21 points accepted; no active sprint.
Goal: Join Game opens a game list on a remembered dedicated server. Open and
password-protected games are discoverable; server enforces passwords, lobby never
contains them. Host chooses game name and optional password. Preserve existing
hidden legacy games and authoritatively reject closed/full/stale admission.

| Story | Points | Acceptance | State |
|---|---:|---|---|
| LB-1 | 5 | Visibility/access contract, protected listing/admission and checkpoint privacy | Done |
| LB-2 | 8 | Remember server, browse/refresh/select, masked password, host setup and cancellation | Done |
| LB-3 | 8 | Process scenarios, installed journeys, full gates, hand ledger and inspected PDF | Done |

21 newly refined local-lobby points; 733 forecast / 597 accepted / 136 remaining
at activation. Existing public online Join/discovery 5 points remain distinct;
this sprint does not close internet transport, identity or public deployment.
No token budget requested. Active-clock forecast 60-120 minutes, including PDF.
No agents requested. Plan: core policy/version boundaries, UI, integration tests,
full gate, installed three-shell journey, screenshot/PDF review and closure.

Loopback-only dedicated server retained. No Pi purchase/deployment, LAN exposure,
Custom Practice, public service, or multi-table tournament movement. No commits,
pushes or external publication inferred. Close this sprint only.

Closure: 733 forecast / 618 accepted / 115 remaining. Full gate: 314 passed,
zero failed, four existing ignored; fmt/Clippy and optimized paired builds pass.
Installed three-shell tournaments, cancellation/password/error paths and compact
lobby exercised. Eight-page PDF rendered and every page inspected. See
review record (local archive: `rituals/2026-09-09-sprint-19-review.md`) and
PDF (local archive: `../../output/pdf/sprint-19-review-report.pdf`). No next sprint activated.

Post-review correction: waiting-host polling/close_notify defect fixed and
deployed with delayed-join regression. No new sprint or points. Evidence and
current build identities: output/sprint20-wait-fix/ and STATUS.md.
