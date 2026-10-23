"""
Confirmation tests for every write tool, run through the registered tool dispatch (`mcp.call_tool`).

For each tool whose schema has a `confirmed` argument:
- `confirmed` omitted and `confirmed=False`: the call is rejected with the confirmation error and no HTTP
  request is made.
- `confirmed=True`: exactly one HTTP request is made.
- A malformed `confirmed` value is rejected by schema validation and no HTTP request is made.

`test_write_tool_set_is_explicit` fails when a tool gains or loses `confirmed`, so a new tool has to be
classified here on purpose.
"""

from unittest.mock import AsyncMock, MagicMock

import pytest

import blt_mcp.server  # noqa: F401  (registers the tools)
from blt_mcp import server_base, v2_tools
from blt_mcp.server_base import mcp

WRITE_TOOLS = {
    "submit_issue",
    "add_comment",
    "auth_signup",
    "create_bug",
    "update_bug",
    "create_user",
    "create_job",
    "update_job",
    "record_job_view",
    "create_hackathon",
    "update_hackathon",
    "register_hackathon_participant",
}
NO_CONFIRMATION_TOOLS = {"auth_signin", "summarize_issue_context", "triage_vulnerability", "draft_vision_issue"}
CONFIRMATION_ERROR = "Write operation requires confirmation"


def _minimal_args(schema: dict) -> dict:
    return {name: "1" for name in schema.get("required", [])}


@pytest.fixture
def http(monkeypatch):
    """Replace the HTTP client used by the v1 and v2 tools and record every request."""
    response = MagicMock()
    response.raise_for_status = MagicMock()
    response.json = MagicMock(return_value={"id": 1})
    response.text = "{}"
    client = MagicMock()
    for method in ("get", "post", "patch", "put", "delete", "request"):
        setattr(client, method, AsyncMock(return_value=response))
    monkeypatch.setattr(server_base, "http_client", client)
    # v2_tools binds the client at import time, so it has to be replaced separately.
    monkeypatch.setattr(v2_tools, "http_client", client)
    return client


def _requests_made(client) -> int:
    return sum(getattr(client, m).await_count for m in ("get", "post", "patch", "put", "delete", "request"))


async def _schemas():
    return {t.name: t.inputSchema for t in await mcp.list_tools()}


async def test_write_tool_set_is_explicit():
    schemas = await _schemas()
    with_confirmed = {name for name, schema in schemas.items() if "confirmed" in schema.get("properties", {})}
    assert with_confirmed == WRITE_TOOLS
    assert set(schemas) == WRITE_TOOLS | NO_CONFIRMATION_TOOLS
    for name in WRITE_TOOLS:
        assert schemas[name]["properties"]["confirmed"]["default"] is False
        assert "confirmed" not in schemas[name].get("required", [])


@pytest.mark.parametrize("tool", sorted(WRITE_TOOLS))
async def test_confirmation_omitted_is_rejected(tool, http):
    args = _minimal_args((await _schemas())[tool])
    with pytest.raises(Exception, match=CONFIRMATION_ERROR):
        await mcp.call_tool(tool, args)
    assert _requests_made(http) == 0


@pytest.mark.parametrize("tool", sorted(WRITE_TOOLS))
async def test_confirmation_false_is_rejected(tool, http):
    args = {**_minimal_args((await _schemas())[tool]), "confirmed": False}
    with pytest.raises(Exception, match=CONFIRMATION_ERROR):
        await mcp.call_tool(tool, args)
    assert _requests_made(http) == 0


@pytest.mark.parametrize("tool", sorted(WRITE_TOOLS))
async def test_confirmation_true_performs_the_write(tool, http):
    args = {**_minimal_args((await _schemas())[tool]), "confirmed": True}
    await mcp.call_tool(tool, args)
    assert _requests_made(http) == 1
    method = next(m for m in ("post", "patch", "put", "delete") if getattr(http, m).await_count)
    assert method in {"post", "patch"}


@pytest.mark.parametrize("tool", sorted(WRITE_TOOLS))
async def test_malformed_confirmation_is_rejected(tool, http):
    args = {**_minimal_args((await _schemas())[tool]), "confirmed": "maybe"}
    with pytest.raises(Exception, match="confirmed"):
        await mcp.call_tool(tool, args)
    assert _requests_made(http) == 0
