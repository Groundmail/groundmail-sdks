# Groundmail Python SDK

Official Python client for the [Groundmail](https://groundmail.dev) transactional email API.

```bash
pip install groundmail
```

```python
from groundmail import Groundmail

tx = Groundmail("gm_your_key_here")

result = tx.emails.send(
    from_="you@yourdomain.com",
    to="user@example.com",
    subject="Hello",
    html="<p>It works.</p>",
)

print(result["id"], result["status"])
```

## Django

```bash
pip install 'groundmail[django]'
```

```python
EMAIL_BACKEND = "groundmail.django.EmailBackend"
# GROUNDMAIL_API_KEY in the environment
```

Docs: https://docs.groundmail.dev/sdks/python
