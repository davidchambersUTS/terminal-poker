# ADR 0023 - Dedicated game lifecycle and local administration

Accepted by the owner on 2026-09-10 through the request to implement the proposed
cleanup rules. This is a bounded follow-up, not a newly activated sprint.

The dedicated registry expires waiting games after ten minutes with no connected
players, running games after fifteen minutes with no connected players, and
finished/cancelled games five minutes after completion is observed. A remaining
connected player retains a running or waiting game; the creator has no special
lifetime authority. Presence includes registered players and waiters, not browsers.
A reconnect resets the empty-game timer. Finished-game retention never resets.
The server observes presence and deadlines once per second using monotonic time.
After restart, retained games receive a full grace period; timers are not added
to the checkpoint format. Existing active-hand crash-recovery limits still apply.

The default reconnect credential TTL is fifteen minutes. Authenticated connected
sessions retain their credentials, and socket departure starts a fresh credential
grace period. This extends the advertised issuance expiry while connected; clients
continue presenting the same token, which rotates on successful reconnect. Explicit
operator TTL overrides still apply. Browsers cannot refresh a player's credential.
Deletion revokes all credentials scoped to the removed table.

Removal first atomically publishes the existing version-4 checkpoint excluding the
tables, routes and credential records being deleted. Only after success does it
release live runtimes, waiting entries, cached terminal updates and capacity. A
publication error leaves live authority intact. A removed game cannot return on
restart. Stable table/hand counters do not regress or reuse identities. Safe hand
history is retained separately; abandonment does not manufacture a winner or award.

A private Unix socket provides operator list, clear-inactive, and remove commands.
Its parent must be private (0700) and the socket is 0600. The interface is local to
Linux/macOS and has bounded requests/timeouts; it is never exposed on the TLS game
port. Safe cleanup removes finished games and empty waiting lobbies. Removing a
running game or a waiting game with connected players requires selecting its table
ID and an explicit --force. Bulk force deletion is unavailable. Listings contain
only table ID, name, coarse lifecycle state and connected-player count.

The initial production cleanup uses clear-inactive. The existing running game is
preserved and subsequently follows the ordinary all-disconnected grace period.
