---
schema: forkmesh-pull-v1
number: 3
title: blt-next: serve leaderboard from BLT-API instead of hardcoded data
base: main
head: pr/vanshika2720/blt-next-leaderboard-real-data-v2
status: open
ts: 1792568803
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: laTEnyOGpDy2qw2BuXgP8CNTazsWinhXa9K4I0wNqJ6J_Tykv_SO9gekt3tk63vwTzDXK652KL7AW1s5-FYpCw
aiModel: None
headBaseOid: 105c83d9159b7c0ab875f22b87f8440ebf5a3778
headOid: db40c95b08d238cdfc0fc599dfed85f8ddee6538
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
