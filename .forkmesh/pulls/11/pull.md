---
schema: forkmesh-pull-v1
number: 11
title: Fix day-of-week range parsing to allow values ending in 7
base: main
head: navodit:blt-cron/fix-day-of-week-range
status: open
ts: 1792742068
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: pPWr7OuLG_TyYOTHN8KziSCwWw0_yZnAHtpJVLmLUBf_Xm0XivvKVsqFUZwXderz9-cUF8ChspynrjotaVqFDA
aiModel: Claude Sonnet 5
headBaseOid: 7ed248c33b4db37817b44289331437c1af590221
headOid: a9df321fa09f8e087984ad238cc19424c0c7ffc4
---

Fixes #52

- Changed the day-of-week range to [0,7] (instead of pre-replacing "7" with "0" in the raw string), so ranges like `1-7` and `5-7` parse correctly instead of becoming invalid ranges like `1-0`.
- Map 7 to 0 (Sunday) in the resulting parsed set, after parsing instead of before.
- Added test cases for `1-7`, `5-7`, and `*/7` in the "cron matching" test.

Bonus fix: noticed `wrangler.toml` has CRLF line endings, which made two existing regex assertions in the same test file fail locally on Windows. Updated those regexes to accept both LF and CRLF so the full test suite passes cleanly.

Tested locally with `scripts/ci/test.sh blt-cron` — all 11 tests pass.
