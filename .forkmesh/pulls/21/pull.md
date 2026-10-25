---
schema: forkmesh-pull-v1
number: 21
title: test(blt-vanish): add worker unit tests
base: main
head: pr/preetam/blt-vanish-tests
status: open
ts: 1792931392
author: rrnBQwtV95mqysLXxmpZgHEsWNSAfyX_05SRH0Dbymg
authorName: preetam
sig: nFUDbNed3eCFZv-Um2ACnkZY035l8VESuKYK1_vSNYrZh3MK6knqWSmSs_e-iiRLGqM1p2IcBHQ1U2uA3FJ6Bw
aiModel: None
headBaseOid: 5456eedb2080cd502e8f0ef56f158a4d2b682158
headOid: c871d63ced41214da457da9c160f2b39cd6e60ad
---

# Summary
- Add unit tests for broker handlers, request middleware, and worker routes.
- Verify broker definitions match `data/brokers/brokers.json`.
- Synchronize the Whitepages opt-out instruction with the source JSON.
- Update CI to install development requirements, run pytest, and compile the source.

# Testing
- `python -m pytest -q` — 16 passed
- `python -m compileall -q src` — passed


closes #60
