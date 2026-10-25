---
schema: forkmesh-pull-v1
number: 19
title: blt-leaf: keep review decisions when a reviewer comments
base: main
head: arnavghai19:fix/blt-leaf-review-status-pr
status: merged
ts: 1792905932
author: VJAr5lCdBD4mHhE9xI32zT-lIIkXDt6C3dZdu7JLqZ0
authorName: arnavghai19
sig: Q_lJDJsDr_pHsF_9AHBK-aTJzzzp-8N_lekXEIp0iCN3WZUj7JfCsB_u34_Jh4HZZ4xGXhNVWxoLQjqZpP7bAQ
aiModel: Claude Opus 5.5
headBaseOid: 1fde75d66fb37f7bd630b089d69b480d3a7c0654
headOid: 31645e9c756a6b2fae30bdc0f1865b3fc4056dd1
mergeBase: 8e48bab6852a0c602c52499a711bd007cdcd23e6
mergeHead: 31645e9c756a6b2fae30bdc0f1865b3fc4056dd1
mergedAt: 1792907567261
coAuthors: noreply@anthropic.com
---

## Summary

Fixes #221.

`calculate_review_status()` previously allowed a reviewer's later `COMMENTED` review to overwrite their earlier `APPROVED` or `CHANGES_REQUESTED` state. This could produce an incorrect overall review status.

This change skips comment-only reviews when recording each reviewer's review state. `DISMISSED` reviews continue to clear the dismissed review state.

## Tests

- Added four regression tests covering comments after approvals, comments after change requests, mixed review states, and dismissed change requests.
- Focused review-status tests: 4 passed.
- Full Python test suite: 11 passed.

## AI disclosure

- **Model:** Claude Opus 5.5 via Claude Code.
- **AI assistance:** Claude Code implemented the fix and assisted with verification. The regression tests were written collaboratively with AI assistance.
