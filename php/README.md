<p align="center">
  <a href="https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://app.groundmail.dev/images/github/readme-logo-dark.png">
      <img alt="Groundmail" src="https://app.groundmail.dev/images/github/readme-logo-light.png" height="44">
    </picture>
  </a>
</p>

<h1 align="center">Groundmail PHP SDK</h1>

<p align="center">
  The official PHP client for the Groundmail transactional email API.<br>
  Send password resets, receipts and alerts in a few lines of PHP.
</p>

<p align="center">
  <a href="https://packagist.org/packages/groundmail/groundmail"><img alt="Packagist" src="https://img.shields.io/packagist/v/groundmail/groundmail?color=a3461f&label=packagist"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-b48f62"></a>
</p>

<p align="center">
  <a href="https://docs.groundmail.dev/sdks/php?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">Docs</a> ·
  <a href="https://app.groundmail.dev/signup?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">Get a free account</a> ·
  <a href="https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks">groundmail.dev</a>
</p>

---

## Install

```bash
composer require groundmail/groundmail
```

## Send an email

```php
<?php

use Groundmail\Client;

$tx = new Client(getenv('GROUNDMAIL_API_KEY'));

$result = $tx->emails->send([
    'from' => 'you@yourdomain.com',
    'to' => 'user@example.com',
    'subject' => 'Hello',
    'html' => '<p>It works.</p>',
]);

echo $result['id'], ' ', $result['status'], PHP_EOL;
```

Using a framework? There's a [Laravel package](https://github.com/Groundmail/groundmail-laravel) and a [Symfony Mailer transport](https://github.com/Groundmail/groundmail-symfony-mailer).

## Contributing

Found a bug or want something added? [Open an issue](https://github.com/Groundmail/groundmail-sdks/issues). This code is published here on each release, so we port fixes rather than merging pull requests directly.

## Why Groundmail

- **Our own mail servers and IP space.** Your password resets and receipts don't share a pool with bulk marketing senders.
- **Transactional email only.** No newsletters or cold outreach on the platform, which keeps the sending reputation clean.
- **Start free.** Free covers 3,000 emails a month. After that, pay as you go is $15 a month with 20,000 emails included, then from $0.70 per 1,000, less as you grow. [Pricing](https://groundmail.dev/pricing/?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks)
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
