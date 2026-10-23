---
schema: forkmesh-pull-v1
number: 3
title: blt-next: serve leaderboard from BLT-API instead of hardcoded data
base: main
head: pr/vanshika2720/blt-next-leaderboard-real-data-v2
status: open
ts: 1792786979
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: 86zaZ09MehrfgOJCRMJ9y0lTiZ7GW4etXbvU2xIgFl4Ebo2lT-yBjDOrIoH_MvdeFDBHHKe8PneyqLKNE8prCQ
aiModel: None
headBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
headOid: ea5c104a2e4f9afdc794a3a9e9e02fe06ffdff0b
syncedHeadOid: 01850a6397015ee5285f4f0ba3d4df01a2f0bd94
revision: 6
creationBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
creationHeadOid: 01850a6397015ee5285f4f0ba3d4df01a2f0bd94
---

Summary

This PR updates the BLT-Next leaderboard to retrieve leaderboard data from BLT-API instead of relying on hardcoded data.

Changes made

Replaces hardcoded leaderboard data with data fetched from BLT-API.

Updates the leaderboard data handling to use the API response.

Displays a dash (—) for bug counts and rewards when those fields are not provided by the API.

Updates the relevant components and supporting logic to accommodate the API-backed leaderboard.

Implementation notes

The changes include updates to blt-next/workers/main.py. This file was manually reconciled with the upstream changes, but I could not run the worker or its tests in my local environment. Reviewers should pay particular attention to this file and verify its behavior.

Testing

The changes have been committed and pushed to the proposed branch.

The worker and its tests could not be executed locally.

Further verification of the API integration and worker behavior is recommended during review.

Expected outcome

The leaderboard will use live data from BLT-API rather than hardcoded entries, making it easier to keep leaderboard information current. Fields not yet available through the API will display a dash instead of misleading or fabricated values.

Review focus

Verify the BLT-API endpoint, response structure, and error handling.

Confirm that leaderboard entries are mapped and rendered correctly.

Review blt-next/workers/main.py for compatibility with upstream changes.

Run the relevant tests and verify the leaderboard in an appropriate environment.
