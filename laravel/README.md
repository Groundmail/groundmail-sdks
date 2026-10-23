<p align="center">
  <a href="https://groundmail.dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://app.groundmail.dev/images/github/readme-logo-dark.png">
      <img alt="Groundmail" src="https://app.groundmail.dev/images/github/readme-logo-light.png" height="44">
    </picture>
  </a>
</p>

<h1 align="center">Groundmail for Laravel</h1>

<p align="center">
  A Laravel mail transport for the Groundmail transactional email API.<br>
  Keep <code>Mail::to()->send()</code>; deliver through Groundmail.
</p>

<p align="center">
  <a href="https://packagist.org/packages/groundmail/laravel"><img alt="Packagist" src="https://img.shields.io/packagist/v/groundmail/laravel?color=a3461f&label=packagist"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-b48f62"></a>
</p>

<p align="center">
  <a href="https://docs.groundmail.dev/frameworks/laravel">Docs</a> ·
  <a href="https://app.groundmail.dev/signup">Get a free account</a> ·
  <a href="https://groundmail.dev">groundmail.dev</a>
</p>

---

## Install

```bash
composer require groundmail/laravel
```

Set in `.env`:

```bash
GROUNDMAIL_API_KEY=gm_your_key_here
MAIL_MAILER=groundmail
```

Add to `config/mail.php`:

```php
'groundmail' => [
    'transport' => 'groundmail',
],
```

## Send as usual

```php
Mail::to($user)->send(new WelcomeMail($user));
```

Your existing Mailables, notifications and queued mail keep working.

## Call the API directly

```php
use Groundmail\Laravel\Facades\Groundmail;

Groundmail::client()->emails->send([
    'from' => 'you@yourdomain.com',
    'to' => 'user@example.com',
    'subject' => 'Hello',
    'html' => '<p>It works.</p>',
]);
```

## Contributing

Found a bug or want something added? [Open an issue](https://github.com/Groundmail/groundmail-sdks/issues). This code is published here on each release, so we port fixes rather than merging pull requests directly.

## Why Groundmail

- **Our own mail servers and IP space.** Your password resets and receipts don't share a pool with bulk marketing senders.
- **Transactional email only.** No newsletters or cold outreach on the platform, which keeps the sending reputation clean.
- **Start free.** Free covers 3,000 emails a month. After that it's pay as you go: $0.70 per 1,000, less as you grow. [Pricing](https://groundmail.dev/pricing/)
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

[Website](https://groundmail.dev) · [Docs](https://docs.groundmail.dev) · [Pricing](https://groundmail.dev/pricing/) · [Sign up free](https://app.groundmail.dev/signup)
