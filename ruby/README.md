# Groundmail Ruby SDK

Official Ruby client for the [Groundmail](https://groundmail.dev) transactional email API.

```bash
gem install groundmail
```

```ruby
require "groundmail"

tx = Groundmail::Client.new("gm_your_key_here")

result = tx.emails.send(
  from: "you@yourdomain.com",
  to: "user@example.com",
  subject: "Hello",
  html: "<p>It works.</p>"
)

puts result["id"], result["status"]
```

## Rails ActionMailer

```ruby
config.action_mailer.delivery_method = :groundmail
config.action_mailer.groundmail_settings = {
  api_key: ENV.fetch("GROUNDMAIL_API_KEY")
}
```

Docs: https://docs.groundmail.dev/sdks/ruby
