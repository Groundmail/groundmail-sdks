from __future__ import annotations

import io
import json
import time
import urllib.error
from types import SimpleNamespace
from unittest.mock import MagicMock, patch

import pytest

from groundmail import Groundmail
from groundmail.http import normalize_access_token


def _ok() -> MagicMock:
    response = MagicMock()
    response.read.return_value = json.dumps({"id": "msg_1"}).encode()
    response.__enter__.return_value = response
    return response


def _unauthorized() -> urllib.error.HTTPError:
    body = io.BytesIO(json.dumps({"error": {"code": "unauthorized", "message": "Invalid"}}).encode())
    return urllib.error.HTTPError("https://api.example.com/emails", 401, "Unauthorized", {}, body)  # type: ignore[arg-type]


def test_token_provider_sets_bearer() -> None:
    client = Groundmail(token_provider=lambda: "entra-token", base_url="https://api.example.com")
    with patch("urllib.request.urlopen", return_value=_ok()) as urlopen:
        client.emails.send(from_="a@example.com", to="b@example.com", subject="Hi", html="<p>Hi</p>")
    assert urlopen.call_args.args[0].get_header("Authorization") == "Bearer entra-token"


def test_access_token_shape_is_cached() -> None:
    calls = []

    def provider() -> SimpleNamespace:
        calls.append(1)
        return SimpleNamespace(token="tok", expires_on=int(time.time()) + 3600)

    client = Groundmail(token_provider=provider, base_url="https://api.example.com")
    with patch("urllib.request.urlopen", side_effect=lambda *_a, **_k: _ok()):
        client.request("GET", "/emails")
        client.request("GET", "/emails")
    assert len(calls) == 1


def test_refreshes_after_401() -> None:
    tokens = iter(["stale", "fresh"])
    client = Groundmail(token_provider=lambda: next(tokens), base_url="https://api.example.com")
    with patch("urllib.request.urlopen", side_effect=[_unauthorized(), _ok()]) as urlopen:
        assert client.request("GET", "/emails") == {"id": "msg_1"}
    assert urlopen.call_args.args[0].get_header("Authorization") == "Bearer fresh"


def test_api_key_401_is_not_retried() -> None:
    client = Groundmail("gm_test_key", base_url="https://api.example.com")
    with patch("urllib.request.urlopen", side_effect=[_unauthorized()]) as urlopen:
        with pytest.raises(Exception):
            client.request("GET", "/emails")
    assert urlopen.call_count == 1


def test_requires_key_or_provider() -> None:
    with pytest.raises(ValueError):
        Groundmail()


def test_normalize_access_token() -> None:
    assert normalize_access_token("opaque") == ("opaque", None)
    assert normalize_access_token(SimpleNamespace(token="t", expires_on=123)) == ("t", 123.0)
