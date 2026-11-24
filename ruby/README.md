# Groundmail Ruby SDK

Official Ruby client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) API.

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

## Validate an address

Check an address before you accept it, for example on a signup form. It's free and sends nothing:

```ruby
result = tx.validate("ada@gmial.com")
puts result["verdict"], result["suggestion"] # risky, ada@gmail.com
```

`verdict` is `valid`, `risky` or `invalid`. Pass `mailbox_check: true` to also confirm the mailbox exists ($0.01 per answer marked `billed`), and use `tx.validate_batch([...])` for up to 100 addresses at once. See https://docs.groundmail.dev/validation?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks

## Rails Action Mailer

The gem registers a `:groundmail` delivery method when Rails is loaded:

```ruby
# config/environments/production.rb
config.action_mailer.delivery_method = :groundmail
config.action_mailer.groundmail_settings = {
  api_key: ENV.fetch("GROUNDMAIL_API_KEY")
}
```

Your mailers and `deliver_later` calls stay the same. Rails guide: https://docs.groundmail.dev/frameworks/rails?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks

Docs: https://docs.groundmail.dev/sdks/ruby?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
