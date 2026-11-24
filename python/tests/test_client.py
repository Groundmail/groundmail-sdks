from __future__ import annotations

import json
import re
from unittest.mock import MagicMock, patch

import pytest

from groundmail import Groundmail
from groundmail.errors import GroundmailError


@pytest.fixture
def client() -> Groundmail:
    return Groundmail("gm_test_key", base_url="https://api.example.com")


def _response(body: bytes = b"{}") -> MagicMock:
    response = MagicMock()
    response.read.return_value = body
    response.__enter__.return_value = response
    return response


def test_sends_its_user_agent(client: Groundmail) -> None:
    with patch("urllib.request.urlopen", return_value=_response()) as urlopen:
        client.request("GET", "/domains")
    assert re.fullmatch(r"groundmail-python/\S+ python/\S+", urlopen.call_args[0][0].get_header("User-agent"))


def test_a_callers_user_agent_wins(client: Groundmail) -> None:
    with patch("urllib.request.urlopen", return_value=_response()) as urlopen:
        client.request("GET", "/domains", headers={"User-Agent": "my-app/1.0"})
    assert urlopen.call_args[0][0].get_header("User-agent") == "my-app/1.0"


def test_requires_gm_prefix() -> None:
    with pytest.raises(ValueError, match="gm_"):
        Groundmail("bad")


def test_emails_send(client: Groundmail) -> None:
    response = MagicMock()
    response.read.return_value = json.dumps({"id": "msg_1", "status": "sent"}).encode()
    response.__enter__.return_value = response

    with patch("urllib.request.urlopen", return_value=response) as urlopen:
        result = client.emails.send(
            from_="a@example.com",
            to="b@example.com",
            subject="Hi",
            html="<p>Hi</p>",
        )

    assert result == {"id": "msg_1", "status": "sent"}
    request = urlopen.call_args.args[0]
    assert request.full_url == "https://api.example.com/emails"
    assert request.method == "POST"
    assert request.get_header("Authorization") == "Bearer gm_test_key"


def test_emails_send_passes_category_and_tenant(client: Groundmail) -> None:
    response = MagicMock()
    response.read.return_value = json.dumps({"id": "msg_1", "status": "sent"}).encode()
    response.__enter__.return_value = response

    with patch("urllib.request.urlopen", return_value=response) as urlopen:
        client.emails.send(
            from_="a@example.com",
            to="b@example.com",
            subject="Hi",
            text="Hi",
            category="product",
            tenant="cust_a",
        )

    body = json.loads(urlopen.call_args.args[0].data)
    assert body["category"] == "product"
    assert body["tenant"] == "cust_a"


def test_http_error_raises_super_send_tx_error(client: Groundmail) -> None:
    import urllib.error

    payload = json.dumps({"error": {"message": "Invalid API key"}}).encode()
    http_error = urllib.error.HTTPError(
        url="https://api.example.com/emails",
        code=401,
        msg="Unauthorized",
        hdrs=None,
        fp=MagicMock(read=MagicMock(return_value=payload)),
    )

    with patch("urllib.request.urlopen", side_effect=http_error):
        with pytest.raises(GroundmailError) as exc:
            client.emails.send(
                from_="a@example.com",
                to="b@example.com",
                subject="Hi",
                html="<p>Hi</p>",
            )

    assert exc.value.status == 401
    assert exc.value.message == "Invalid API key"


def test_validate(client: Groundmail) -> None:
    body = {"email": "ada@gmial.com", "verdict": "invalid", "reasons": ["no_mail_server", "possible_typo"]}
    with patch("urllib.request.urlopen", return_value=_response(json.dumps(body).encode())) as urlopen:
        result = client.validate("ada@gmial.com")

    assert result == body
    request = urlopen.call_args.args[0]
    assert request.full_url == "https://api.example.com/validate"
    assert request.method == "POST"
    assert json.loads(request.data) == {"email": "ada@gmial.com"}


def test_validate_with_mailbox_check_and_tenant(client: Groundmail) -> None:
    with patch("urllib.request.urlopen", return_value=_response()) as urlopen:
        client.validate("ada@acme.com", mailbox_check=True, tenant="cust_1")
    assert json.loads(urlopen.call_args.args[0].data) == {"email": "ada@acme.com", "mailbox_check": True, "tenant": "cust_1"}


def test_validate_batch(client: Groundmail) -> None:
    with patch("urllib.request.urlopen", return_value=_response(b'{"data": []}')) as urlopen:
        result = client.validate_batch(["a@acme.com", "b@acme.com"], mailbox_check=True)

    assert result == {"data": []}
    request = urlopen.call_args.args[0]
    assert request.full_url == "https://api.example.com/validate/batch"
    assert json.loads(request.data) == {"emails": ["a@acme.com", "b@acme.com"], "mailbox_check": True}


def test_validate_error_raises(client: Groundmail) -> None:
    import io
    import urllib.error

    error = urllib.error.HTTPError(
        "https://api.example.com/validate", 401, "Unauthorized", {}, io.BytesIO(b'{"error": {"message": "Invalid API key", "code": "unauthorized"}}')
    )
    with patch("urllib.request.urlopen", side_effect=error), pytest.raises(GroundmailError):
        client.validate("a@b.com")
