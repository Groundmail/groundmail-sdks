"""Groundmail Python SDK — transactional email API client."""

from groundmail.client import Groundmail
from groundmail.errors import GroundmailError

__all__ = ["Groundmail", "GroundmailError"]
DEFAULT_API_BASE_URL = "https://api.groundmail.dev"
