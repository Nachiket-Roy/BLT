---
schema: forkmesh-pull-v1
number: 13
title: blt-vanish: rename remaining SelfErase references to BLT Vanish
base: main
head: pr/vanshika2720/vanish-selferase-rename
status: merged
ts: 1792786592
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: 7pDQRuqAW6KC26YhJN3wUwurjJszRcQVB9Z3MYtpbplCDhBu9eAPJOmWfqdPINYX3FK1AGlSmWN4TTxJSTHFAg
aiModel: None
headBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
headOid: fa480e8ce8b13de76cb7ff3de20dfce5607a11f0
syncedHeadOid: fa480e8ce8b13de76cb7ff3de20dfce5607a11f0
revision: 3
creationBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
creationHeadOid: 511322f9a8a6aa88844b490293be4fd6c9989c33
mergeBase: 09120547eee9ede7e2ab79ff1a80504c86c4e211
mergeHead: fa480e8ce8b13de76cb7ff3de20dfce5607a11f0
mergedAt: 1792793081098
---

## Summary

This PR completes the remaining branding rename from **SelfErase** to **BLT Vanish**, ensuring the application and its documentation consistently use the current product name.

## Changes made

- Updated `blt-vanish/README.md` to reflect the BLT Vanish branding.
- Updated `blt-vanish/data/schema.json` to remove outdated user-facing branding while preserving the schema structure.
- Renamed the Flutter application class from `SelfEraseApp` to `BLTVanishApp` and updated the relevant references.
- Reviewed related project files for remaining outdated branding and updated applicable references.

## Testing

- Checked the repository for remaining `SelfErase` references.
- Verified relevant imports and application entry-point references.
- Ran the applicable formatting, analysis, and test checks where available.

## Scope

This is a branding and consistency cleanup. It does not intentionally change application functionality or introduce new features.

The changes address the remaining rename work described in issue #36.

**Related issue:** #36 - Follow-up from "feat: add macOS platform build configuration"
