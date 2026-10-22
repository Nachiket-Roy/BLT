---
schema: forkmesh-pull-v1
number: 7
title: blt-docs: use pull request instead of merge request and add blt-cron to project map
base: main
head: hrinkar01:docs/pr-terminology-blt-cron
status: open
ts: 1792649680
author: RstIDDJG6C6U1Pai9khxpJmAfEPyL4aIsmU-ctc0kJM
authorName: hrinkar01
sig: snVUa5VY4IlPmvf3AbO-X9xsF660Ivkdp0ydZEsaPhCbnelJvT2j9EJIlp7qhDdZaqROvlLcBytBoSkJrl4vDA
aiModel: Gemini
headBaseOid: 067cb2ac4e41075de7759cf1b9a843c63ff98a46
headOid: dc3d9545caa6c6dd702b6bca5a18638d5703724b
---

Fixes #186

Changes: Replaced 18 occurrences of merge request with pull request across 14 documentation files in blt-docs to reflect current git.owaspblt.org terminology, and added blt-cron to the project registry table in docs/getting-started/projects.md in alphabetical order.

Files updated:

README.md
docs/index.md
docs/getting-started/setup.md
docs/getting-started/projects.md
docs/features/overview.md
docs/developer-guide/contributing.md
docs/developer-guide/docs-maintenance.md
docs/developer-guide/platform-architecture.md
docs/developer-guide/agentic-development.md
docs/developer-guide/bot-setup.md
docs/repositories/blt-api.md
docs/repositories/blt-jobs.md
docs/repositories/blt-next.md
docs/repositories/blt-sizzle.md

Verification:
Verified grep for merge request in blt-docs returns 0 occurrences outside the internal plan.
Verified strict MkDocs build passes locally with zero errors.

AI Disclosure: Used iGemin to assist in scanning and locating all occurrences across the repository. All changes and builds were verified locally.
