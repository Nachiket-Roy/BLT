---
schema: forkmesh-pull-v1
number: 14
title: Fix /health endpoint to build endpoint map dynamically from router
base: main
head: navodit:blt-api/fix-health-endpoint
status: open
ts: 1792934044
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
sig: jc299Fps8EVe83hCe69V5yqNBClY_0NsEHvOt-N3fClRPPNEk0-bPU4ZA2Yy0yUn68O7YoAdnOYNxJgEMtCjDg
aiModel: Claude Sonnet 5
headBaseOid: 3658a48760208ca297bc8dae8a5b935dc11331c7
headOid: 510615e8bb1a8b64d43ba1049126a40764e13596
revision: 2
creationBaseOid: 3658a48760208ca297bc8dae8a5b935dc11331c7
creationHeadOid: a02b8d0bf355785f7064d27adaa20a61a8005b6b
---

Fixes #47

- Pointed `documentation` at `/dashboard` (it previously returned `/docs`, which has no route).
- Converted `handle_health` to a factory (`make_health_handler(router)`, same pattern as `make_routes_handler`), so the endpoint map is built dynamically from `router.get_route_list()` instead of being a hardcoded dict. This way every top-level route the health response lists is guaranteed to be a real, registered route — including any added in the future, with no need to update health.py by hand.
- Filtered out `/v2`-prefixed routes (duplicates of the unversioned routes) and dynamic-segment-only paths when building the map.
- Added `tests/test_health.py` with two tests: one on the basic response shape, and one that asserts every path in the `endpoints` map is an actual registered route on the router (the acceptance criteria from the issue).

Tested locally with `scripts/ci/test.sh blt-api` — all 435 tests pass.
