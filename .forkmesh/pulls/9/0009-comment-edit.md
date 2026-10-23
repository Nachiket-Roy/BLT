---
type: comment-edit
id: comment-edit-web-1792777612-ff6c1d50-0ab9-48bc-9855-c7b0db7e4620
author: EqNbJ81Ib7tk86tkz55_EDo4gCk4z8HvrLf8ewzPc5c
authorName: dahahanish
ts: 1792777612
target: comment-web-1792777499-ee94e4f3-2f07-4509-8b57-31603d250284
sig: BljDwn8VFZKR7CcJtKSh58L232443Fhhg7LhzHbJjG9JtlLtKHRE6p93AGi5_uEStDFddFWVs6bGgynYrbvEDw
---

Thanks for the review @donnieblt ! The update went through on this PR after I synced my fork's main (it was behind upstream, which made the mirror compare time out), so no fresh PR needed. All points addressed:

1. Timeout: replaced the AbortController + setTimeout lambda with AbortSignal.timeout(SLACK_TIMEOUT_SECONDS * 1000), as in blt-vanish's health.py. Added a test that drives the Pyodide path with a fake js module and checks the signal comes from AbortSignal.timeout and setTimeout is never called (it fails on the old code).
2. Rebased onto main (2 commits on top of main).
3. Moved the issue URL to src/forge_links.py; slack_notifier and mcp_export both use it. This also fixes mcp_export's source_ref, which was missing the "/" before the issue number.
4. DEPLOYMENT.md now recommends a private channel and notes that notifications start once #34 lands.

blt-toasty: 526 passed (2 new tests), ruff and bandit clean.
