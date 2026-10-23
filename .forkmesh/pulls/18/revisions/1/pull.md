---
schema: forkmesh-pull-v1
number: 18
title: blt-mcp: enforce strict plain-text output for draft_vision_issue
base: main
head: pr/vanshika2720/blt-mcp-vision-prompt
status: open
ts: 1792791561
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: 8wiYNECjExUD36YWqCth3IM_jAQxWrQPk1Ry-8NggTmbsb1TQYBLbgKISgsno-S4SFpiMXQjYCBuGUpiSfaLBQ
aiModel: None
headBaseOid: f7733a40ae9c1e2f0be04c4b49072476e1748f55
headOid: bb09e6d9e2cd106ce101a7e266a96eb47ee33562
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
