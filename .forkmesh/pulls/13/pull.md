---
schema: forkmesh-pull-v1
number: 13
title: blt-vanish: rename remaining SelfErase references to BLT Vanish
base: main
head: pr/vanshika2720/vanish-selferase-rename
status: open
ts: 1792746122
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: gm3MbUJG3mVawbCuLQKupMM87s6Ep3V6v-6VsCBmZpjPvvpj1U6H5PS87s6Q7QRW4xmgVEVX9PpLXuHUtn01AQ
aiModel: None
headBaseOid: 0088329439f17f7e6aa6b3d6a00a4a56e9eaa70e
headOid: 75da549209e75ea4e75903f4f3fca9e3a5440d65
syncedHeadOid: 75da549209e75ea4e75903f4f3fca9e3a5440d65
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
