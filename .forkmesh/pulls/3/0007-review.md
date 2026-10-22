---
type: review
id: review-web-1792704139-43b0b827-3d92-407b-bd0b-0653cdda877d
author: YpN9eh6-B3zwn-kIgvInmuJ3rXCyfjNrjktHBQLT0pY
authorName: donnieblt
ts: 1792704139
state: changes_requested
sig: A2R57OIU3CiG5Hcn2shJweLJDJzz2Bck2HMZ6RnVk5BbslO5kfIUxcQxod5j_NaAyrEs2U23UBnH7ahyrtKoDQ
---

Hi @vanshika2720, checking in on this one. The branch hasn't changed since the review on 21 October, so those two points are still open:

1. **Tests.** Please add a test for `handle_leaderboard` in `blt-next/tests` that stubs `call_blt_api` and covers a normal response (rows render, a `<script>` username is escaped, `data-total` is set), an empty list, and an API error or non-200 response (the "Unable to load" message).
2. **Safe number parsing.** `int(total)` and `int(item.get('total_score') or 0)` run outside the `try`, so a non-numeric value from BLT-API turns into a 500. A small helper that falls back to `0` / `len(entries)` fixes it.

Also:
- There's a `-v3` branch with the same change squashed into one commit, but this PR still points at `-v2`. Please push the fix to one branch and update the PR to it, so we review the right head.
- It merges cleanly with main, and the blt-next tests pass on the merged tree (`scripts/ci/test.sh blt-next`), so the rebase is easy.
- Optional: drop `BLT_API_URL` from `wrangler.toml`. It duplicates the default, and blt-next reaches blt-api through the `BLT_API` service binding.

Thanks, this is close.
