<p align="center">
  <a href="https://groundmail.dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://app.groundmail.dev/images/github/readme-logo-dark.png">
      <img alt="Groundmail" src="https://app.groundmail.dev/images/github/readme-logo-light.png" height="44">
    </picture>
  </a>
</p>

<h1 align="center">Groundmail SDKs</h1>

<p align="center">
  Official clients for the Groundmail transactional email API:<br>
  Python, Go, Ruby, PHP, Laravel and Symfony Mailer. Node.js is on npm as <a href="https://www.npmjs.com/package/groundmail"><code>groundmail</code></a>.
</p>

<p align="center">
  <a href="https://pypi.org/project/groundmail/"><img alt="PyPI" src="https://img.shields.io/pypi/v/groundmail?color=a3461f&label=pypi"></a>
  <a href="https://rubygems.org/gems/groundmail"><img alt="RubyGems" src="https://img.shields.io/gem/v/groundmail?color=a3461f&label=gem"></a>
  <a href="https://packagist.org/packages/groundmail/groundmail"><img alt="Packagist" src="https://img.shields.io/packagist/v/groundmail/groundmail?color=a3461f&label=packagist"></a>
  <a href="https://pkg.go.dev/github.com/Groundmail/groundmail-sdks/go"><img alt="Go reference" src="https://img.shields.io/badge/go-reference-241c15"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-b48f62"></a>
</p>

<p align="center">
  <a href="https://docs.groundmail.dev">Docs</a> ·
  <a href="https://app.groundmail.dev/signup">Get a free account</a> ·
  <a href="https://groundmail.dev">groundmail.dev</a>
</p>

---

## Install

| Language | Install | Source |
|----------|---------|--------|
| Python (Django backend included) | `pip install groundmail` | [`python/`](./python) |
| Go | `go get github.com/Groundmail/groundmail-sdks/go` | [`go/`](./go) |
| Ruby | `gem install groundmail` | [`ruby/`](./ruby) |
| PHP | `composer require groundmail/groundmail` | [`php/`](./php) · [groundmail-php](https://github.com/Groundmail/groundmail-php) |
| Laravel | `composer require groundmail/laravel` | [`laravel/`](./laravel) · [groundmail-laravel](https://github.com/Groundmail/groundmail-laravel) |
| Symfony Mailer | `composer require groundmail/symfony-mailer` | [`symfony-mailer/`](./symfony-mailer) · [groundmail-symfony-mailer](https://github.com/Groundmail/groundmail-symfony-mailer) |
| Node.js / TypeScript | `npm install groundmail` | [npm](https://www.npmjs.com/package/groundmail) |

## Send an email

**Python**

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

**Go**

```go
client, err := groundmail.New("gm_your_key_here")
if err != nil {
	log.Fatal(err)
}

result, err := client.Emails.Send(map[string]any{
	"from":    "you@yourdomain.com",
	"to":      "user@example.com",
	"subject": "Hello",
	"html":    "<p>It works.</p>",
})
```

**Ruby**

```ruby
require "groundmail"

tx = Groundmail::Client.new("gm_your_key_here")

result = tx.emails.send(
  from: "you@yourdomain.com",
  to: "user@example.com",
  subject: "Hello",
  html: "<p>It works.</p>"
)
```

Each folder has its own README with framework setup, errors and retries.

## Contributing

Found a bug or want something added? [Open an issue](https://github.com/Groundmail/groundmail-sdks/issues). These SDKs are published here on each release, so we port fixes rather than merging pull requests directly.

## Why Groundmail

- **Our own mail servers and IP space.** Your password resets and receipts don't share a pool with bulk marketing senders.
- **Transactional email only.** No newsletters or cold outreach on the platform, which keeps the sending reputation clean.
- **Start free.** 3,000 emails a month on the Free plan; Pro is 50,000 emails for $20/month. Dedicated IPs on Scale.
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

[Website](https://groundmail.dev) · [Docs](https://docs.groundmail.dev) · [Pricing](https://groundmail.dev/pricing) · [Sign up free](https://app.groundmail.dev/signup)
