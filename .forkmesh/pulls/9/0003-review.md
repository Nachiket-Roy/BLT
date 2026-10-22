---
type: review
id: review-web-1792703999-c37324dc-ff60-4b2b-a6eb-30cab3a69437
author: YpN9eh6-B3zwn-kIgvInmuJ3rXCyfjNrjktHBQLT0pY
authorName: donnieblt
ts: 1792703999
state: changes_requested
sig: 2j_OKSceo9CyF-ywnViA_eGX4eSYMBU0HMALo9Iacl4HYUsMXV4dDwPDaNqrZOt2ys3MN4EZgXTezLLS_b9xBw
---

Thanks @dahahanish, this is a careful piece of work. The redaction approach is right: only metadata, a fixed quarantine category instead of the raw reason, no title or body. The notifier is best effort, the wiring avoids double notifications, the AI use is disclosed with a trailer, and the 524 blt-toasty tests pass on the branch merged with main. One thing to fix before merge:

1. **The 5 s timeout won't fire under Pyodide.** `setTimeout(lambda: controller.abort(), ...)` passes a Python lambda to a JS function. Pyodide destroys that borrowed proxy when `setTimeout` returns, so if Slack hangs, the timer throws instead of aborting and the fetch waits. That's the one case the timeout is there for. The tests don't catch it because they take the CPython path. Simplest fix: drop the controller and timer and pass `"signal": AbortSignal.timeout(SLACK_TIMEOUT_SECONDS * 1000)` (`from js import AbortSignal`), as `blt-vanish/cloudflare_workers/src/health.py` does. If you keep the timer, wrap the callback in `pyodide.ffi.create_once_callable`.

Smaller points:
- Please rebase onto main when you push the fix. It merges cleanly today, but it's 559 commits behind.
- `ISSUE_URL_TEMPLATE` hardcodes `https://git.owaspblt.org/BLT/issues/{iid}`. That's fine for now, but `mcp_export.py` builds the same URL, so a shared constant would keep them in step.
- In `DEPLOYMENT.md`, suggest a **private** channel. Messages name the issue and its CWE before a maintainer has looked at it.
- Thanks for flagging that nothing runs the stages in order yet (#34). Please also say in `DEPLOYMENT.md` that notifications start once that lands, so nobody sets the secret and waits for messages.
