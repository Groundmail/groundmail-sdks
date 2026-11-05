# Groundmail Rust SDK

Official Rust client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) transactional email API.

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

Verify a webhook with the raw request body and the `Groundmail-Signature` header:

```rust
let ok = groundmail::verify_webhook_signature(&secret, &body, &signature_header);
```

Docs: https://docs.groundmail.dev/sdks/rust?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
