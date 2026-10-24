---
type: comment
id: comment-web-1792831214-8d6d101f-a421-4e2c-bd8e-e89b251999f8
author: EqNbJ81Ib7tk86tkz55_EDo4gCk4z8HvrLf8ewzPc5c
authorName: dahahanish
ts: 1792831214
sig: 2s9_8qWRs8h5okcVpXY1LNwDimsucEzRo_PAR6QRYjLh37DlDbW-uWS24gCpVxrG1iCI1Qb9eHnmxCvd2IdGAQ
---

Hey @s3dfx-cyber , I've pushed the changes we agreed on over slack (rebased onto main):

- Severity: the field now reads "Est. severity (from CWE type)" with a note that it is estimated from the CWE type, not an assessment of the report. CWEs the table doesn't know, CWE-UNKNOWN, or no CWE show "unrated, needs review" instead of a guessed tier.
- notify_triage/build_payload take an optional assessed_severity; a valid tier is shown as "Severity (assessed)" and replaces the estimate, so a future assessment stage is picked up without notifier changes.
- Trust boundary: every interpolated value is validated before use (issue number = positive int, event IDs = [A-Za-z0-9_:-]{1,64}, CWE IDs = CWE-<digits>/CWE-UNKNOWN, scores = finite 0-1). Anything else becomes "unknown issue" / "[invalid id]" / "similarity n/a" or is dropped, and all interpolated text is escaped (&, <, >). Crafted-input tests cover links, <!channel>, @-mentions, backticks, formatting, long strings and newlines in each field, including crafted values stored in D1.
- Tests: replaced the SQL-text-matching mock with a SQLite-backed D1 fixture (conftest.sqlite_d1) that applies the real migrations and runs the actual queries; tests assert on query results and table contents.

blt-toasty: 630 passed, ruff and bandit clean.

I'll open the separate issue for hybrid severity assessment shortly, once this PR is merged.

@donnieblt  Please review :)
