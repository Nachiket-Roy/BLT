---
schema: forkmesh-pull-v1
number: 15
title: blt-mcp: sync AGENT.md with tool definitions
base: main
head: pr/vanshika2720/blt-mcp-sync-agent-md
status: open
ts: 1792752076
author: Cu6HYOAI_KXF_0n4il24bXIniH5dBGJ0ELffhPIkLLQ
authorName: vanshika2720
sig: qLb57qbyPp4Ylbd4_ww4SIi2PchEEBbXLx3Rzg1vxW-RtnDISupUzPnX34eyZEJc9-69a_LFDIWrcS_PZPRlBw
aiModel: None
headBaseOid: 92a6484fa139ad82b25a3e603b5c60b70cc0a58f
headOid: 0d6d4bde0812af29e8ca6b0c410ab46ceee962ed
syncedHeadOid: 0d6d4bde0812af29e8ca6b0c410ab46ceee962ed
---

## Summary

Update `blt-mcp/AGENT.md` to align the documentation with the currently registered tools and their actual implementations.

The existing documentation had drifted from the tool definitions, including outdated arguments, missing confirmation requirements, and undocumented tools.

## Changes Made

- **`add_comment`**: Documented `issue_id`, `comment_text`, and `confirmed=False`, including the requirement to set `confirmed=true` before the operation can proceed.
- **`submit_issue`**: Corrected the required `title` argument, removed the undocumented `screenshot` argument, and documented `labels`, `assignee`, and `confirmed`.
- **`summarize_issue_context`**: Documented its `issue_id` argument, read-only behavior, and string-based error responses.
- **`draft_vision_issue`**: Documented `image_path` and optional `context`, supported image formats, file size and path restrictions, required Cloudflare credentials, and draft-only behavior.
- **V2 write tools**: Added documentation for the previously undocumented tools and a dedicated section explaining write-operation confirmation requirements.
- **`triage_vulnerability`**: Added its Cloudflare credential requirements.

## Validation

- Verified tool arguments and behavior against `server.py`, `tools.py`, and `v2_tools.py`.
- Ran `scripts/ci/test.sh blt-mcp` successfully.
- Ran Prettier on `AGENT.md`.
- Limited changes to documentation; no runtime code was modified.

**Note:** Prettier also reformatted a few existing bullets and removed trailing whitespace. The live server was not run, and tool behavior was verified through source-code inspection.

## Motivation

Keep `AGENT.md` accurate and reliable so agents can discover available tools, use the correct arguments, understand confirmation requirements, and avoid unintended write operations.
