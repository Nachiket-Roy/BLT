---
schema: forkmesh-pull-v1
number: 11
title: Fix day-of-week range parsing to allow values ending in 7
base: main
head: navodit:blt-cron/fix-day-of-week-range
status: open
ts: 1792785937
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: gDm90m9ezU1yYhLnIsU48bFsqk5CrtakuBcJqPtu5vjNblhW2Jr9T_bP5gUy6FjdYbJRJMfym1LjB-5TzhOLDw
aiModel: Claude Sonnet 5
headBaseOid: a8e09d5480a36498976b4b3676529bd97f398187
headOid: 0a05462934dd9f0ced27e86671edeaaba01abcdb
revision: 2
creationBaseOid: a8e09d5480a36498976b4b3676529bd97f398187
creationHeadOid: a6cd1f1fb4d05468785d0e98a8e24487915f5360
---

Fixes #52

- Changed the day-of-week range to [0,7] (instead of pre-replacing "7" with "0" in the raw string), so ranges like `1-7` and `5-7` parse correctly instead of becoming invalid ranges like `1-0`.
- Map 7 to 0 (Sunday) in the resulting parsed set, after parsing instead of before.
- Added test cases for `1-7`, `5-7`, and `*/7` in the "cron matching" test.

Bonus fix: noticed `wrangler.toml` has CRLF line endings, which made two existing regex assertions in the same test file fail locally on Windows. Updated those regexes to accept both LF and CRLF so the full test suite passes cleanly.

Tested locally with `scripts/ci/test.sh blt-cron` — all 11 tests pass.
