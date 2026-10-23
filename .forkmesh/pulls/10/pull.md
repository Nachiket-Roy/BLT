---
schema: forkmesh-pull-v1
number: 10
title: Fix day-of-week range parsing to allow values ending in 7
base: main
head: navodit:blt-cron/fix-day-of-week-range
status: open
ts: 1792738561
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: E-K99swWRwLCppapWXaSFFQ6omo8m-EYrtLVPakAJhU54zQ7jytES5-kXA5-lU0_Ai3bNZwOQ1YKQvx2sdvzAQ
aiModel: Claude Sonnet 5
headBaseOid: b1658f146dfe69b6d692c1fffdb283c96060b8b7
headOid: 4d2a2f1c756f067a0b7c2536e57170cd5dc1495f
---

Fixes #52

- Changed the day-of-week range to [0,7] (instead of pre-replacing "7" with "0" in the raw string), so ranges like `1-7` and `5-7` parse correctly instead of becoming invalid ranges like `1-0`.
- Map 7 to 0 (Sunday) in the resulting parsed set, after parsing instead of before.
- Added test cases for `1-7`, `5-7`, and `*/7` in the "cron matching" test.

Bonus fix: noticed `wrangler.toml` has CRLF line endings, which made two existing regex assertions in the same test file fail locally on Windows. Updated those regexes to accept both LF and CRLF so the full test suite passes cleanly.

Tested locally with `scripts/ci/test.sh blt-cron` — all 11 tests pass.

AI assistance: I identified the root cause (the string-replace approach breaking ranges ending in 7) and the fix approach myself, using Claude to help debug the wrangler.toml line-ending issue. I reviewed and tested every change before committing.
