---
schema: forkmesh-pull-v1
number: 9
title: blt-toasty: notify Slack when Toasty finishes triage (#35)
base: main
head: dahahanish:issue-35-blt-toasty-build-slack-notification-bot
status: open
ts: 1792830832
author: EqNbJ81Ib7tk86tkz55_EDo4gCk4z8HvrLf8ewzPc5c
authorName: dahahanish
sig: g6jlJgiIjuy4z05AFjnnNLAzZk2C2_HsEPcalmqBG31Dty25udlks_QpHEPWAEzsU0JoEcj_ysx2hrkpp1i5AA
aiModel: Claude Opus 5.5
headBaseOid: ecc1b2f7a9d91a5a31fae2181e26fbaefeb07a12
headOid: 34266776628c576163f34632021baf3af275ffee
revision: 5
creationBaseOid: ecc1b2f7a9d91a5a31fae2181e26fbaefeb07a12
creationHeadOid: f80c711ecb4e95feb59bbc38fbca770d4b9630f7
---

Closes #35.

WHAT CHANGED
Adds a Workers-native Slack notifier to blt-toasty (src/slack_notifier.py) that alerts maintainers when Toasty finishes triaging an issue or quarantines it.

- Runtime: uses js.fetch with an AbortController timeout and reads env.SLACK_WEBHOOK_URL. No urllib.request or os.getenv, which were the problems with blt-toasty!31.
- Wiring:
    - detect_duplicates (the last pipeline stage) notifies on both exit paths and includes the confirmed duplicate candidates.
    - quarantine_event notifies whenever it has an env.
    - redact() now passes env through so PII-gate quarantines are reported too.
- Message: issue number and link, CWE IDs and labels, a severity tier derived from the CWEs (highest tier wins), quarantine status with a fixed reason category, and possible duplicates with their similarity scores.
- Redaction: the title, report text, summary and raw quarantine reason are never sent, because the reason can include exception text.
- Best effort: if the webhook isn't set, nothing is sent. Slack errors and non-2xx responses are logged and ignored, so triage never fails because of Slack.
- Docs: SLACK_WEBHOOK_URL added to DEPLOYMENT.md (wrangler secret put) and .env.worker.example.

SETUP (MAINTAINERS)
The notifier stays off until a webhook is configured:
1. On https://api.slack.com/apps create an app, enable Incoming Webhooks, add it to a channel (e.g. #toasty-triage) and copy the webhook URL.
2. Store it on the Toasty worker: npx wrangler secret put SLACK_WEBHOOK_URL --env production
3. To turn it off, delete the secret.

Messages are one-way posts into that channel only. Nothing appears on the website.

Note: I couldn't find any code in blt-toasty that runs the triage stages in order (redaction -> summary -> duplicate detection); the webhook handler only stores the event. Notifications fire when those stages run, so they depend on that wiring, which looks like #34 (Cloudflare Queues).

TESTS
- New tests/test_slack_notifier.py (28 tests):
    - payload fields
    - a PII-in-title test checking the title never reaches Slack
    - quarantine reason categories
    - severity tiers
    - skipping when no webhook is set
    - fetch errors and non-2xx responses
    - wiring for duplicate detection, quarantine and redaction
- pip install --group dev && python -m pytest -q in blt-toasty (the scripts/ci/tests.tsv command, run by hand on Windows in a fresh venv): 524 passed (496 before).
- ruff check and bandit on the changed files: clean.
- Already broken on main (not changed here): tests/test_mcp_export.py fails 2 tests when run on its own because it depends on test order. It passes in the full suite.

AI DISCLOSURE
- Model: Claude Opus 5.5 (claude-opus-5-5) via Claude Code
- Thinking: reasoning effort medium

Generated with Claude Code (https://claude.com/claude-code)
