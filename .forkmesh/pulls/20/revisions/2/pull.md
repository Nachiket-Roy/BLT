---
schema: forkmesh-pull-v1
number: 20
title: blt-netguardian: add unit tests for payload_redact
base: main
head: owasp-marquina:blt-netguardian/add-payload-redact-tests
status: open
ts: 1792926559
author: SDGRl2pWA4LQfxCX-son7AShrtzI9j9fDXQRG0Duig4
authorName: owasp-marquina
sig: y0MWwBRGqOSvvJx5-pU-x7b0ndmyWUli2B7oTXxT4Lm36A5VsssXFzveQAKZbhSjFeKXgOGsIa2mLMWFVIDFBA
aiModel: Claude Opus 5.5
headBaseOid: a00adc54291ae3dea5245f2ef323368a1d0d367c
headOid: 25fe105f355755ac20d046cafa772faa0a4e1412
revision: 2
creationBaseOid: a00adc54291ae3dea5245f2ef323368a1d0d367c
creationHeadOid: 1b1c5b3395bc25eb9ee1084986d972cc121c77eb
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
