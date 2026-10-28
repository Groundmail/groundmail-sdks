# Groundmail Python SDK

Official Python client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) transactional email API.

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

## Azure managed identity (no API key)

On Azure, pass a token provider instead of a key. The client caches the token until shortly before it expires:

```python
from azure.identity import DefaultAzureCredential
from groundmail import Groundmail

credential = DefaultAzureCredential()
client = Groundmail(token_provider=lambda: credential.get_token("https://api.groundmail.dev/.default"))
```

Add the Groundmail app to your Entra tenant once (`az ad sp create --id 0146ad5c-4051-4703-aece-e4edf9805d8e`) and register the identity in the dashboard first: https://docs.groundmail.dev/authentication/azure-managed-identity?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks

## Django

```bash
pip install 'groundmail[django]'
```

```python
EMAIL_BACKEND = "groundmail.django.EmailBackend"
# GROUNDMAIL_API_KEY in the environment
```

Docs: https://docs.groundmail.dev/sdks/python?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
