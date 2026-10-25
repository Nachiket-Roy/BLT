---
schema: forkmesh-pull-v1
number: 11
title: Fix day-of-week range parsing to allow values ending in 7
base: main
head: navodit:blt-cron/fix-day-of-week-range
status: open
ts: 1792934758
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: wudOPrT2U6u9wAFJIpPt2y61ISiIVPpn6RwTOsHo5CJ37P5Kp0qGH0fWVw9kqRmP4S59979FwphdOMN7GPyXCg
aiModel: Claude Sonnet 5
headBaseOid: 3658a48760208ca297bc8dae8a5b935dc11331c7
headOid: f152da8c3f9d154ebcf4cc0818870c86179d4a8e
revision: 4
creationBaseOid: 3658a48760208ca297bc8dae8a5b935dc11331c7
creationHeadOid: f8c98f7c3a6a565f0c8b249b14c1f2358922be8d
---

Fixes #52

- Changed the day-of-week range to [0,7] (instead of pre-replacing "7" with "0" in the raw string), so ranges like `1-7` and `5-7` parse correctly instead of becoming invalid ranges like `1-0`.
- Map 7 to 0 (Sunday) in the resulting parsed set, after parsing instead of before.
- Added test cases for `1-7`, `5-7`, and `*/7` in the "cron matching" test.

Bonus fix: noticed `wrangler.toml` has CRLF line endings, which made two existing regex assertions in the same test file fail locally on Windows. Updated those regexes to accept both LF and CRLF so the full test suite passes cleanly.

Tested locally with `scripts/ci/test.sh blt-cron` — all 11 tests pass.
