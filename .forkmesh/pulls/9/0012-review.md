---
type: review
id: review-web-1792781889-55cf4373-84e2-49c2-baa9-d99cc1b6238c
author: gxltjPfmdiXKA78VbPYy3nwR8CSvKWou6mFsIjm4CWs
authorName: s3dfx-cyber
ts: 1792781889
state: changes_requested
sig: Uq4L1gVsziq_WHqhzUrL6f7V1-uCH4sP_kC5cDn5U9kHPY_8p9ecoJ5xlAWRZogz4ZwLCIH0nvhYeMLHvN45DA
---

Appreciate for the changes , I have some changes needed
The new hard-coded CWE-to-severity table assigns severity tiers to individual CWEs, but the mapping is a heuristic rather than a reliable assessment of the specific finding's impact. The message may present a heuristic tier as authoritative.
Slack message content needs explicit trust-boundary tests — blt-toasty/src/slack_notifier.py:133–159. Issue references and duplicate event identifiers are interpolated into Slack mrkdwn fields. Although raw report text is excluded, the screenshots do not establish that every interpolated identifier is constrained to its expected format. Slack markup can interpret crafted text as links or formatting.
The mock identifies queries by searching their SQL text. This is workable for unit tests but can become brittle if query formatting changes. Prefer assertions around explicit query behavior where practical.
