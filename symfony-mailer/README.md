# groundmail/symfony-mailer

[Symfony Mailer](https://symfony.com/doc/current/mailer.html) transport for [Groundmail](https://groundmail.dev) transactional email (HTTP API).

## Install

```bash
composer require groundmail/symfony-mailer
```

```bash
export GROUNDMAIL_API_KEY=gm_your_key_here
```

## Usage

```php
use Groundmail\Client;
use Groundmail\Symfony\Transport\GroundmailTransport;
use Symfony\Component\Mailer\Mailer;
use Symfony\Component\Mime\Email;

$mailer = new Mailer(new GroundmailTransport(new Client(getenv('GROUNDMAIL_API_KEY'))));

$email = (new Email())
    ->from('noreply@yourdomain.com')
    ->to('user@example.com')
    ->subject('Welcome')
    ->html('<p>Welcome.</p>');

$mailer->send($email);
```

### DSN

```php
use Groundmail\Symfony\Transport\GroundmailTransportFactory;
use Symfony\Component\Mailer\Transport;

$transport = Transport::fromDsns(
    'groundmail+api://%s@default',
    null,
    null,
    null,
    [new GroundmailTransportFactory()],
);
// Or: groundmail://default?api_key=gm_…
```

In Symfony Framework, register `GroundmailTransportFactory` as a `mailer.transport_factory` service and set:

```yaml
# config/packages/mailer.yaml
framework:
  mailer:
    dsn: 'groundmail+api://%env(GROUNDMAIL_API_KEY)%@default'
```

## Headers

| Header | Purpose |
|--------|---------|
| `X-Groundmail-Tag` | `name=value` tag (repeatable) |
| `X-Groundmail-Idempotency-Key` | Idempotency key |
| `X-Groundmail-Scheduled-At` | ISO 8601 schedule |

## Docs

https://docs.groundmail.dev/frameworks/symfony

## License

MIT
