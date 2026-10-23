---
schema: forkmesh-pull-v1
number: 14
title: Fix /health endpoint to build endpoint map dynamically from router
base: main
head: navodit:blt-api/fix-health-endpoint
status: open
ts: 1792750965
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: G8hp3ucMbgZ0tuRk0v-LTbmIn4UMQ1EWzJ68W5Hgw5wmabteSOsS4OSe1tlA9nvSiD6qtIGGXj0_S6QpyJWqBw
aiModel: Claude Sonnet 5
headBaseOid: 10a4595af5997228e6d16711e2e23a1e6b776957
headOid: 484be0fb3aed51aaa1169d85364468b51e5ed46a
---

Fixes #47

- Pointed `documentation` at `/dashboard` (it previously returned `/docs`, which has no route).
- Converted `handle_health` to a factory (`make_health_handler(router)`, same pattern as `make_routes_handler`), so the endpoint map is built dynamically from `router.get_route_list()` instead of being a hardcoded dict. This way every top-level route the health response lists is guaranteed to be a real, registered route — including any added in the future, with no need to update health.py by hand.
- Filtered out `/v2`-prefixed routes (duplicates of the unversioned routes) and dynamic-segment-only paths when building the map.
- Added `tests/test_health.py` with two tests: one on the basic response shape, and one that asserts every path in the `endpoints` map is an actual registered route on the router (the acceptance criteria from the issue).

Tested locally with `scripts/ci/test.sh blt-api` — all 435 tests pass.
