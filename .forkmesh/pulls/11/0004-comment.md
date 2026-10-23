---
type: comment
id: comment-web-1792786298-e0911b3f-34ff-41bc-9990-c5b35502ac4e
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
ts: 1792786298
sig: r4kKApa1i-bvTYrylehCze_mQW98tSz3bWGKutfpl6RODl8tDEkBC0nbngH_Fn2ZBZOwfOhj_nOthvpM5ajiCQ
---

Thank you @s3dfx-cyber for raising these concerns. Addressing each:

1. blt-cron's cron parser explicitly treats 0 and 7 as equivalent for the day-of-week field. This is stated in the code comment: "Standard 5-field cron in UTC: minute hour day-of-month month day-of-week (0 or 7 = Sunday)." The fix maps 7 to 0 post-parse so both resolve to the same weekday value.

2. Confirmed: parseField() fully expands the range into a Set before any normalization occurs. The relevant code is:

   const dow = parseField(fields[4], RANGES[4]);
   if (dow.has(7)) {
     dow.delete(7);
     dow.add(0);
   }

   parseField() is called first with the complete range string ("1-7", "5-7", "*/7"); the resulting Set is fully populated before the 7→0 substitution runs. There is no premature normalization of range endpoints prior to expansion.

   Separately, RANGES[4] was changed from [0, 6] to [0, 7] as part of this fix. Without this change, parseField() would reject any value of 7 as out-of-range before normalization could even apply — this was the root cause of the original bug.

3. I've pushed additional test assertions to establish the exact semantics rather than relying on a single successful match:

   - Negative assertions confirming cronMatches() returns false for 5-7 and */7 on a non-Sunday date, ruling out the possibility that these ranges match every day as a side effect of faulty expansion.
   - Direct assertions on the parsed weekday Set itself:
     assert.deepEqual(parseCron("0 0 * * 1-7").weekday, new Set([0, 1, 2, 3, 4, 5, 6]));
     assert.deepEqual(parseCron("0 0 * * 5-7").weekday, new Set([0, 5, 6]));

   These confirm that 0 and 7 collapse into a single weekday value (0) rather than persisting as two distinct set members, and that the resulting set contains exactly the expected days — not a superset or malformed range.

All tests pass locally via scripts/ci/test.sh blt-cron. Happy to add further edge cases if any remain unaddressed.
