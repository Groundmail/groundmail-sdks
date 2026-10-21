from __future__ import annotations

import base64
import json
import time
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime
from typing import Any, Callable, Optional

from groundmail.errors import GroundmailError

# A token provider returns a token string, or an object with `.token` and
# `.expires_on` (seconds since epoch) such as azure-identity's AccessToken.
TokenProvider = Callable[[], Any]

# Refresh this long before a token expires, so it never expires in flight.
_TOKEN_REFRESH_SKEW_SECONDS = 120
# Tokens with no known expiry are reused this long at most.
_TOKEN_DEFAULT_TTL_SECONDS = 300


def _jwt_expiry(token: str) -> Optional[float]:
    """`exp` from a JWT (not verified; only used to know when to refresh)."""
    parts = token.split(".")
    if len(parts) < 2:
        return None
    try:
        padded = parts[1] + "=" * (-len(parts[1]) % 4)
        claims = json.loads(base64.urlsafe_b64decode(padded.encode("ascii")))
        exp = claims.get("exp")
        return float(exp) if isinstance(exp, (int, float)) else None
    except Exception:
        return None


def normalize_access_token(value: Any) -> tuple[str, Optional[float]]:
    """A token provider's result -> (token, expiry in seconds since epoch, if known)."""
    if not value:
        return "", None
    if isinstance(value, str):
        return value, _jwt_expiry(value)
    token = getattr(value, "token", None)
    if not isinstance(token, str):
        return "", None
    expires_on = getattr(value, "expires_on", None)
    if isinstance(expires_on, datetime):
        expiry: Optional[float] = expires_on.timestamp()
    elif isinstance(expires_on, (int, float)):
        expiry = float(expires_on)
    else:
        expiry = _jwt_expiry(token)
    return token, expiry


class HttpClient:
    def __init__(
        self,
        api_key: Optional[str] = None,
        *,
        base_url: str = "https://api.groundmail.dev",
        token_provider: Optional[TokenProvider] = None,
    ) -> None:
        if token_provider is not None:
            if not callable(token_provider):
                raise ValueError("token_provider must be a callable that returns a token")
            self.api_key: Optional[str] = None
        else:
            if not api_key or not api_key.startswith("gm_"):
                raise ValueError("Groundmail API key must start with gm_")
            self.api_key = api_key
        self.token_provider = token_provider
        self._cached_token: Optional[str] = None
        self._refresh_at = 0.0
        self.base_url = base_url.rstrip("/")

    def _bearer(self, force_refresh: bool = False) -> str:
        if self.api_key:
            return self.api_key
        if not force_refresh and self._cached_token and time.time() < self._refresh_at:
            return self._cached_token
        token, expiry = normalize_access_token(self.token_provider())  # type: ignore[misc]
        if not token:
            raise ValueError("Groundmail: token_provider returned no token")
        self._cached_token = token
        self._refresh_at = (
            expiry - _TOKEN_REFRESH_SKEW_SECONDS if expiry else time.time() + _TOKEN_DEFAULT_TTL_SECONDS
        )
        return token

    def request(
        self,
        method: str,
        path: str,
        *,
        body: Any = None,
        headers: dict[str, str] | None = None,
    ) -> Any:
        try:
            return self._send(method, path, body=body, headers=headers, force_refresh=False)
        except GroundmailError as error:
            # With a token provider, a 401 may be an expired token: refresh once.
            if self.token_provider is not None and getattr(error, "status", None) == 401:
                self._cached_token = None
                return self._send(method, path, body=body, headers=headers, force_refresh=True)
            raise

    def _send(
        self,
        method: str,
        path: str,
        *,
        body: Any,
        headers: dict[str, str] | None,
        force_refresh: bool,
    ) -> Any:
        url = f"{self.base_url}{path}"
        payload = None
        req_headers = {
            "Authorization": f"Bearer {self._bearer(force_refresh)}",
            "Content-Type": "application/json",
            **(headers or {}),
        }
        if body is not None:
            payload = json.dumps(body).encode("utf-8")
        request = urllib.request.Request(url, data=payload, headers=req_headers, method=method)
        try:
            with urllib.request.urlopen(request) as response:
                raw = response.read().decode("utf-8")
                if not raw:
                    return {}
                return json.loads(raw)
        except urllib.error.HTTPError as error:
            raw = error.read().decode("utf-8")
            try:
                parsed = json.loads(raw)
            except json.JSONDecodeError:
                parsed = {}
            raise GroundmailError.from_response(error.code, parsed) from error

    @staticmethod
    def query(params: dict[str, Any]) -> str:
        filtered = {key: value for key, value in params.items() if value is not None}
        if not filtered:
            return ""
        return "?" + urllib.parse.urlencode(filtered)
