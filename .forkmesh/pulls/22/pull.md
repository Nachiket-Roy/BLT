---
schema: forkmesh-pull-v1
number: 22
title: blt-docs: exclude superpowers from build and update repo metadata (#185)
base: main
head: hrinkar01:fix/exclude-internal-docs-issue-185
status: open
ts: 1792947138
author: RstIDDJG6C6U1Pai9khxpJmAfEPyL4aIsmU-ctc0kJM
authorName: hrinkar01
sig: FVRk1ypb0lR8Y5uq6NST-BMSAxm422Lg8xPH7UXQp6-9-JGoubby4PjX1L-PZO8UkVYjllKaxuIP7I3HLDEDBQ
aiModel: None
headBaseOid: c1356c7aef815b2387aac924149e1b4838712582
headOid: 97d02140edce8b6ba82c92093aa5afdb89a20aad
---

Fixes #185

Context:
docs/superpowers/plans/2026-08-20-blt-documentation-modernization.md was not in navigation but was still getting compiled into site/superpowers/ during site builds. Additionally, mkdocs.yml had an outdated repo name and copyright year.

Changes made:
- Added exclude_docs: superpowers/ to mkdocs.yml so internal planning documents are excluded from the build.
- Updated repo name from owasp-blt/blt-docs to BLT/blt-docs.
- Updated copyright year to 2026.

Testing and Verification:
- Ran uvx --with-requirements requirements.txt mkdocs build --strict locally.
- Confirmed that site/superpowers/ is no longer generated.
- Strict build passes cleanly with zero warnings or errors.
