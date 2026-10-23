---
schema: forkmesh-pull-v1
number: 18
title: blt-mcp: enforce strict plain-text output for draft_vision_issue
base: main
head: pr/vanshika2720/blt-mcp-vision-prompt
status: open
ts: 1792791576
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: laoysf-mDuS-qybZUi6mNtQPvlN700ykfvtkFzwA7acWt1dlDNGqN0aeUQD9KcGMvG3KXMQiktfC4RlZ24j0Cw
aiModel: None
headBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
headOid: 72dc51fe405724ca658e4e23bf19f0d299155a48
revision: 2
creationBaseOid: adcea056cc55f7789374ed5c1dbead26c85725bd
creationHeadOid: b5a3e67e7b208b978f1b29803a837236915abfcf
---

## Summary

Fixes #24 by improving the Vision AI prompt and validating model output before it reaches the TUI.

The previous prompt loosely requested `TITLE:` and `DESCRIPTION:` sections, allowing Markdown-heavy, inconsistent, or malformed responses to populate the issue submission form.

## Changes

- **Add `blt_mcp/vision.py`:** Introduces a strict plain-text prompt and `parse_vision_response()` helper.
- **Enforce structured output:** Requires exactly one non-empty `TITLE:` followed by one non-empty `DESCRIPTION:`.
- **Reject malformed responses:** Handles missing, duplicate, or reversed markers, empty fields, introductory text, multiline titles, and invalid output types.
- **Allow limited normalization:** Tolerates wrapping code fences and Markdown decoration around the required markers without broadly rewriting model output.
- **Handle invalid output safely:** Updates `draft_vision_issue` in `blt_mcp/server.py` to reject unusable responses without exposing provider error details or filling the form with malformed content.
- **Add regression tests:** Introduces `tests/test_vision.py` covering the prompt contract, valid and malformed responses, the no-issue fallback, tool integration, and TUI parsing compatibility.

When no clear issue is visible, the model is instructed to return:

```text
TITLE: No clear issue identified
DESCRIPTION: The screenshot does not provide sufficient visual evidence to identify a specific bug.
```

## Testing

- `uv run pytest tests/test_vision.py` — 29 passed
- `uv run pytest` — 58 passed
- `ruff check blt_mcp tests` — passed

## Compatibility

The `draft_vision_issue` tool name, arguments, and string return type are unchanged. Invalid model output now produces a controlled failure instead of being passed through to the form.

## Limitations

- Responses containing introductory text before `TITLE:` are rejected rather than automatically repaired.
- The no-issue fallback is displayed in the title field; filtering it would require a separate TUI change.
- TUI parser compatibility is tested using a Python copy of the Rust parser, not the Rust implementation itself.
- The README still describes the tool as drafting Markdown issue reports and may need a separate documentation update.
