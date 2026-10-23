---
schema: forkmesh-pull-v1
number: 12
title: blt-toasty: notify BLT Chat about terminal triage outcomes
base: main
head: himaneeesh:toasty-chat-notifier
status: open
ts: 1792743869
author: s3hhVb22t037OCG2vjJx291ljjPDh7-mX6PliP4yDAI
authorName: himaneeesh
sig: MNT-yvHHlfbPuvI79sjGlUb5PxWn8HfOA_a7sQGn1YZtslTeH0dQdp3JC0vYd_0EWPMxBYwxZBCZgBmXT52bDA
aiModel: OpenAI GPT-5 (Codex)
headBaseOid: 55c8485acdb38ab8df74f1104a6a5828f04a9c54
headOid: 6009e9dc2232e65261925c6298bbb41f961f721e
---

Relates to #35.

## Scope

This implementation sends notifications to **BLT Chat**, not Slack. It uses a BLT Chat Incoming Webhook configured through `BLT_CHAT_WEBHOOK_URL`.

The issue originally described Slack notifications; this PR deliberately targets BLT Chat as the project’s chat destination. Maintainer confirmation is needed if Slack compatibility is still a hard requirement.

## What changed

- Adds `src/notifier.py`, a Workers-native BLT Chat formatter and delivery layer.
- Sends an exported-triage alert after an event successfully transitions to `exported`.
- Sends a quarantine alert after an event successfully transitions to `quarantined`.
- Passes the Worker environment through the redaction quarantine path so PII-gate failures can notify BLT Chat.
- Adds tests for safe notification contents, successful-state-only delivery, error containment, and the native Worker timeout.
- Adds local-only webhook proof and diagnostic helpers.
- Documents secret setup without committing a webhook URL.
- Makes staging Worker bindings and Durable Object SQLite migrations explicit in `wrangler.toml`.

## Architecture

The D1 state machine is the source of truth.

1. Toasty completes a terminal state transition in D1.
2. Only after that successful state update does the state machine invoke the notifier.
3. The notifier reads only the minimal metadata required for the alert and posts it to BLT Chat.
4. Delivery is best effort: a missing secret, timeout, transport failure, or non-success response is logged and never rolls back or changes the completed D1 state.

Quarantine state updates and audit-log writes remain atomic. The notification is intentionally outside that database operation so an external chat outage cannot weaken Toasty’s fail-closed redaction path.

## Cloudflare / Workers design

- Uses the Workers JavaScript `fetch` bridge rather than `urllib` or `os.getenv`.
- Uses `AbortSignal.timeout(...)` for a native Worker timeout; it does not pass a Python callback to JavaScript `setTimeout`.
- Reads `BLT_CHAT_WEBHOOK_URL` from the Worker environment. The secret is not stored in source control.
- Keeps the webhook optional: Toasty operates normally when the secret is not configured.
- Staging configuration explicitly declares its AI binding, D1 binding, Durable Object SQLite migrations, and related variables.

## Privacy and security assumptions

- The BLT Chat webhook points to a private maintainer-controlled channel.
- D1 event state is authoritative; chat delivery is observability only.
- Alerts contain safe operational metadata only: event/issue identifiers, CWE hints, a severity placeholder, and confirmed duplicate metadata.
- Quarantine alerts use a fixed, generalized reason category.
- Alerts never contain issue titles, report bodies, summaries, raw PII, raw exception text, or the raw quarantine reason.
- Severity is shown as `unknown (not assessed)` because Toasty does not currently persist a severity score for this notification path.
- Notifications begin when existing pipeline stages call the exported/quarantined state transitions; they are not sent merely when an inbound webhook is stored.

## Validation

- Added focused unit coverage for formatting, redaction safety, confirmed-duplicate lookup, post-transition notification timing, timeout configuration, and contained delivery failures.
- `git diff --check` passes.
- The local environment used for preparation lacks `pytest-asyncio`, so the async test suite should be run in the project development environment before merge.

## AI disclosure

AI assistance was used during implementation and review.
