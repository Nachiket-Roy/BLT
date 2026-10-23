---
schema: forkmesh-pull-v1
number: 16
title: blt-cli: use cycle-able selectors for Severity and Bug Type
base: main
head: pr/vanshika2720/blt-cli-selectors
status: open
ts: 1792786085
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: Rj3vjTuGB7Dio5zAtpWLECxp919KdwQRZMmOp25pOM0dJYodAIdneo5eLiri-FRkXKPPH05-iS39u7-cI2P2Dw
aiModel: None
headBaseOid: 1ba8c8d06c8c40b2fdd1479bd4685c635fc01b45
headOid: dbf421d5d3d11fabd3c7549c2b2861124a0848a5
syncedHeadOid: 64e9eee853f9f405c48dd82bac967d42d6680356
revision: 2
creationBaseOid: 1ba8c8d06c8c40b2fdd1479bd4685c635fc01b45
creationHeadOid: 64e9eee853f9f405c48dd82bac967d42d6680356
---

## Summary

Replace the free-text Severity and Bug Type fields in the BLT CLI vulnerability submission form with keyboard-driven selectors.

This improves usability, prevents invalid or mistyped values, and makes vulnerability reporting faster and more consistent.

## Changes Made

- **Severity selector:** Cycles through `Critical`, `High`, `Medium`, and `Low`.
- **Bug Type selector:** Cycles through `XSS`, `SQL Injection`, `CSRF`, `IDOR`, `Authentication Bypass`, `Information Disclosure`, and `Other`.
- **Keyboard navigation:** Supports Up/Down and Left/Right arrow keys to move between options, with wraparound in both directions.
- **UI feedback:** Displays the current selection and a hint explaining the available keyboard controls.
- **Form integration:** Preserves existing focus navigation, submission behavior, and API payload compatibility.
- **Regression tests:** Covers selector ordering, navigation, wraparound, focus handling, defaults, and submission values.

## Motivation

Previously, users had to manually type severity levels and bug categories, making the form error-prone and slower to use. Predefined selectors ensure consistent values and provide a smoother terminal UI experience.

## Related Issue

Closes #6
