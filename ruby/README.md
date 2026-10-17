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

## Rails Action Mailer

The gem registers a `:groundmail` delivery method when Rails is loaded:

```ruby
# config/environments/production.rb
config.action_mailer.delivery_method = :groundmail
config.action_mailer.groundmail_settings = {
  api_key: ENV.fetch("GROUNDMAIL_API_KEY")
}
```

Your mailers and `deliver_later` calls stay the same. Rails guide: https://docs.groundmail.dev/frameworks/rails

Docs: https://docs.groundmail.dev/sdks/ruby
