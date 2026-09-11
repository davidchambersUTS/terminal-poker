# Changelog

## Unreleased

- Share legal Left/Right action selection and Enter submission between Practice
  and network tables, with visible focus/amounts and stale/repeated-input guards.
- Reject all-in raises when a short raise has not reopened action; retain legal
  all-in calls and align client validation and training action masks.
- Make restart process coverage tolerate valid late-client observations while
  retaining deterministic fresh-authority and chip-conservation regressions.
- Document the tested terminal baseline, lifecycle source handoff and completed
  Sprint 21 review. The paired candidate is not yet deployed.

- Fix heart suit glyph rendering as outlined/hollow in Ghostty terminal. Switched from U+2665 (BLACK HEART SUIT, missing from Menlo) to U+2764 (HEAVY BLACK HEART, present in Menlo) to avoid Nerd Font fallback providing an incorrect glyph.
