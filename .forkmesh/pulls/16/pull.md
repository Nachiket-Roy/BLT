---
schema: forkmesh-pull-v1
number: 16
title: blt-cli: use cycle-able selectors for Severity and Bug Type
base: main
head: pr/vanshika2720/blt-cli-selectors
status: open
ts: 1792776491
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: p_drxJbD2vS_OvK3UEg4g3zwoUi49tggXR06dD7A8l2Q24A_7djPBB-7Bv_qPWiTuCGH3ToAQwrOCjsS2XpgBQ
aiModel: None
headBaseOid: 812b48c4185ff39c78568134a8902fa42c332e67
headOid: cd55535ec5c57cb067f86de3848571a8c6e88bfe
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
