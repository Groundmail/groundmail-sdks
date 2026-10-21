from __future__ import annotations

from typing import Any

from groundmail.errors import GroundmailError
from groundmail.http import HttpClient, TokenProvider
from groundmail.resources import (
    DomainsResource,
    EmailsResource,
    SuppressionsResource,
    TemplatesResource,
    WebhooksResource,
)

DEFAULT_API_BASE_URL = "https://api.groundmail.dev"


class Groundmail:
    """Thin HTTP client for the Groundmail REST API."""

    def __init__(
        self,
        api_key: str | None = None,
        *,
        base_url: str = DEFAULT_API_BASE_URL,
        token_provider: TokenProvider | None = None,
    ) -> None:
        """Authenticate with an API key (``gm_...``), or keyless with ``token_provider``.

        ``token_provider`` is called for a Bearer token (for example an Azure managed
        identity's Microsoft Entra token: ``lambda: credential.get_token("https://api.groundmail.dev/.default")``).
        The client caches it until shortly before it expires and asks again after a 401.
        """
        self._http = HttpClient(api_key, base_url=base_url, token_provider=token_provider)
        self.emails = EmailsResource(self._http)
        self.domains = DomainsResource(self._http)
        self.webhooks = WebhooksResource(self._http)
        self.templates = TemplatesResource(self._http)
        self.suppressions = SuppressionsResource(self._http)

    def request(self, method: str, path: str, *, body: Any = None, headers: dict[str, str] | None = None) -> Any:
        return self._http.request(method, path, body=body, headers=headers)
