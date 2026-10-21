---
schema: forkmesh-pull-v1
number: 6
title: blt-legacy: add tests for ossh_catalog view
base: main
head: psamuelvijay:test/ossh-catalog-coverage
status: merged
ts: 1792613987
author: gKYk725y-DlKjRB7i6nng7fXWReWZOGMBHLdNYPIxRI
authorName: psamuelvijay
sig: bDHyxkYMcA6x4ekSjODXx0stShIim8Iqasyhujq1nE-qF_VTpN-wMd2VVIlvRld4D1PDtbgM02p2OAO4CMNoDQ
aiModel: Kiro with Claude Sonnet 4.5
headBaseOid: 76452b475b084d33b2ff1d79395dcf2eb49a0dff
headOid: c353858415bcbde86d97074c78f223b376d6f8ce
mergeBase: 5ac8ff6a8e679756de9a3aeef7ab12c8eefd085f
mergeHead: c353858415bcbde86d97074c78f223b376d6f8ce
mergedAt: 1792622477861
---

Fixes #44

Adds `blt-legacy/website/tests/test_ossh_catalog.py`, covering the
`ossh_catalog` view (`GET /api/ossh/catalog/`), which previously had no
tests — `test_ossh_recommender.py` only covers `/api/ossh/github/`.

### What's covered
- **Response shape**: top-level keys (`repos`, `communities`,
  `discussion_channels`, `articles`) and the per-item fields for each of
  the four models, seeded with tags.
- **Empty database**: the endpoint returns all four keys with empty lists.
- **Method restriction**: `POST` returns `405`.
- **No N+1**: an `assertNumQueries` check confirms the query count stays
  constant (8 = one model query + one `tags` prefetch per model, across
  four models) whether there is 1 row or 3 of each kind. This guards the
  `prefetch_related("tags")` behaviour the view's docstring promises.

### How I tested
```
cd blt-legacy && uv run python manage.py test website.tests.test_ossh_catalog
```
Result: 8 tests, all passing.
