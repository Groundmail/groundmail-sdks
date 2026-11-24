# Groundmail Rust SDK

Official Rust client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) API.

```bash
cargo add groundmail
```

```rust
use groundmail::{Client, SendEmail};

#[tokio::main]
async fn main() -> Result<(), groundmail::Error> {
    let client = Client::from_env()?; // reads GROUNDMAIL_API_KEY

    let email = SendEmail::new("you@yourdomain.com", "user@example.com", "Hello")
        .html("<p>It works.</p>");
    let sent = client.emails().send(&email).await?;

    println!("{} {}", sent.id, sent.status);
    Ok(())
}
```

Check an address before you accept it, for example on a signup form. It's free and sends nothing:

```rust
use groundmail::ValidateOptions;

let result = client.validate("ada@gmial.com", &ValidateOptions::default()).await?;
println!("{} {}", result["verdict"], result["suggestion"]); // "risky" "ada@gmail.com"
```

`verdict` is `valid`, `risky` or `invalid`. Set `mailbox_check: true` in `ValidateOptions` to also confirm the mailbox exists ($0.01 per answer marked `billed`), and use `client.validate_batch(&emails, &options)` for up to 100 addresses at once. See https://docs.groundmail.dev/validation?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks

Verify a webhook with the raw request body and the `Groundmail-Signature` header:

```rust
let ok = groundmail::verify_webhook_signature(&secret, &body, &signature_header);
```

Docs: https://docs.groundmail.dev/sdks/rust?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
