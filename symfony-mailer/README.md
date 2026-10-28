<p align="center">
  <a href="https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://app.groundmail.dev/images/github/readme-logo-dark.png">
      <img alt="Groundmail" src="https://app.groundmail.dev/images/github/readme-logo-light.png" height="44">
    </picture>
  </a>
</p>

<h1 align="center">Groundmail for Symfony Mailer</h1>

<p align="center">
  A <a href="https://symfony.com/doc/current/mailer.html">Symfony Mailer</a> transport for the Groundmail transactional email API.<br>
  Send over the HTTP API with a DSN, no SMTP needed.
</p>

<p align="center">
  <a href="https://packagist.org/packages/groundmail/symfony-mailer"><img alt="Packagist" src="https://img.shields.io/packagist/v/groundmail/symfony-mailer?color=a3461f&label=packagist"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-b48f62"></a>
</p>

<p align="center">
  <a href="https://docs.groundmail.dev/frameworks/symfony?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">Docs</a> ·
  <a href="https://app.groundmail.dev/signup?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">Get a free account</a> ·
  <a href="https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">groundmail.dev</a>
</p>

---

## Install

```bash
composer require groundmail/symfony-mailer
```

```bash
export GROUNDMAIL_API_KEY=gm_your_key_here
```

## Symfony Framework

Register `GroundmailTransportFactory` as a `mailer.transport_factory` service and set the DSN:

```yaml
# config/packages/mailer.yaml
framework:
  mailer:
    dsn: 'groundmail+api://%env(GROUNDMAIL_API_KEY)%@default'
```

## Standalone

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

Or build it from a DSN:

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

## Headers

| Header | Purpose |
|--------|---------|
| `X-Groundmail-Tag` | `name=value` tag (repeatable) |
| `X-Groundmail-Idempotency-Key` | Send once, even when your code retries |
| `X-Groundmail-Scheduled-At` | ISO 8601 time to send later |

## Contributing

Found a bug or want something added? [Open an issue](https://github.com/Groundmail/groundmail-sdks/issues). This code is published here on each release, so we port fixes rather than merging pull requests directly.

## Why Groundmail

- **Our own mail servers and IP space.** Your password resets and receipts don't share a pool with bulk marketing senders.
- **Transactional email only.** No newsletters or cold outreach on the platform, which keeps the sending reputation clean.
- **Start free.** Free covers 3,000 emails a month. After that it's pay as you go: $0.70 per 1,000, less as you grow. [Pricing](https://groundmail.dev/pricing/?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks)
- **Built for code and for agents.** REST API, SMTP, SDKs for Node, Python, Go, PHP and Ruby, a CLI, and an MCP server for Claude, ChatGPT and Cursor.

## All Groundmail libraries

| Use | Package |
|-----|---------|
| Node.js / TypeScript | [`groundmail`](https://www.npmjs.com/package/groundmail) on npm |
| Python (Django), Go, Ruby (Rails Action Mailer) | [Groundmail/groundmail-sdks](https://github.com/Groundmail/groundmail-sdks) |
| PHP | [Groundmail/groundmail-php](https://github.com/Groundmail/groundmail-php) |
| Laravel | [Groundmail/groundmail-laravel](https://github.com/Groundmail/groundmail-laravel) |
| Symfony Mailer | [Groundmail/groundmail-symfony-mailer](https://github.com/Groundmail/groundmail-symfony-mailer) |
| Nodemailer | [Groundmail/groundmail-nodemailer](https://github.com/Groundmail/groundmail-nodemailer) |
| NestJS | [Groundmail/nestjs-groundmail](https://github.com/Groundmail/nestjs-groundmail) |
| Strapi | [Groundmail/strapi-provider-email-groundmail](https://github.com/Groundmail/strapi-provider-email-groundmail) |
| Medusa | [Groundmail/medusa-notification-groundmail](https://github.com/Groundmail/medusa-notification-groundmail) |
| AdonisJS | [Groundmail/adonisjs-groundmail](https://github.com/Groundmail/adonisjs-groundmail) |
| Phoenix (Swoosh) | [Groundmail/swoosh_groundmail](https://github.com/Groundmail/swoosh_groundmail) |
| CLI | [`groundmail-cli`](https://www.npmjs.com/package/groundmail-cli) on npm |
| MCP server (Claude, ChatGPT, Cursor) | [Groundmail/groundmail-mcp](https://github.com/Groundmail/groundmail-mcp) |
| Agent skills and Claude Code plugin | [Groundmail/groundmail-skills](https://github.com/Groundmail/groundmail-skills) |
| Email templates (React Email, MJML, HTML) | [Groundmail/email-templates](https://github.com/Groundmail/email-templates) |

[Website](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) · [Docs](https://docs.groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) · [Pricing](https://groundmail.dev/pricing/?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) · [Sign up free](https://app.groundmail.dev/signup?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks)
