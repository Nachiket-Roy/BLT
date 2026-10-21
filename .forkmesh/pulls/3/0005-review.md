---
type: review
id: review-web-1792622470-74de929f-2cfa-4c5d-8d90-969538064fa2
author: JZwDCCTQsQtXm3k2fURwage7qDlqcLaEgtnGR1_BFZk
authorName: donnieblt
ts: 1792622470
state: changes_requested
sig: O1iyf8PRMtQomn93gr8hxjVVR7msHYqOnGL3kGUaKUKJ6CFDYQNmL44nc0z3P0LHm7lOsvE-JKbiTKtYSTR9Dw
---

Thanks @vanshika2720 — this is close. Moving the leaderboard to BLT-API through call_blt_api is the right approach, usernames/initials are escaped, the failure and empty states are handled, and the existing blt-next tests pass. Two small things before we merge:

1. Tests: please add a test for handle_leaderboard in blt-next/tests that stubs call_blt_api and covers (a) a normal response (rows rendered, a username with <script> is escaped, data-total set), (b) an empty list, and (c) an API error / non-200 (the "Unable to load" message).
2. Defensive parsing: int(total) and int(item.get('total_score') or 0) raise if BLT-API ever returns a non-numeric value, which would turn the response into a 500 outside your try block. Please parse them safely (e.g. a small helper that falls back to 0 / len(entries)).

Optional: BLT_API_URL in wrangler.toml duplicates the code default and blt-next already reaches blt-api through the BLT_API service binding, so that line can go.
