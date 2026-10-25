---
schema: forkmesh-pull-v1
number: 20
title: blt-netguardian: add unit tests for payload_redact
base: main
head: owasp-marquina:blt-netguardian/add-payload-redact-tests
status: open
ts: 1792920175
author: SDGRl2pWA4LQfxCX-son7AShrtzI9j9fDXQRG0Duig4
authorName: owasp-marquina
sig: XePT2iBVolkN0zpeg8y7m87lxsGU78-e2MuXv4SEo4YBc3PvKQwU3yZAC5TgLGpOMe8JmDe-367UUVZOJzuIAw
aiModel: Claude Opus 5.5
headBaseOid: 99654b6df54e1ff1941089b0a897c2d2ed382c12
headOid: 85848888d7717656932a95675ad5a6aece727fe5
---

Closes #74

## What
Adds `blt-netguardian/tests/test_payload_redact.py` with 9 passing tests and 2 strict xfail tests. No source code changed.

## Why
`payload_redact.py` hides sensitive values in finding snippets returned by the findings API and PDF reports, but nothing under `tests/` covered it.

## Covered
- top-level sensitive keys
- case-insensitive key matching (original key spelling kept)
- nested dicts and lists of dicts
- sensitive key holding a dict (whole value redacted)
- non-dict pass-through
- input dict is not modified

## Known gaps (xfail, not fixed here)
Only exact key names are matched, so `access_token` and `api-key` are not redacted today. Recorded as `xfail(strict=True)` so these tests start failing once redaction is extended.

## How tested
On macOS, Python 3.14.6:
- `scripts/ci/test.sh blt-netguardian`: PASS
- New file: 9 passed, 2 xfailed
- Full NetGuardian suite:  371 passed, 2 xfailed
## AI use
Test cases drafted with Claude Opus 5.5 (claude.ai). I reviewed each test, ran them locally and can explain them.
