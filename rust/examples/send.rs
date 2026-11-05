//! `GROUNDMAIL_API_KEY=gm_… cargo run --example send -- you@yourdomain.com user@example.com`

use groundmail::{Client, SendEmail};

#[tokio::main]
async fn main() -> Result<(), groundmail::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [from, to] = args.as_slice() else {
        eprintln!("usage: send <from> <to>");
        std::process::exit(2);
    };
    let client = match std::env::var("GROUNDMAIL_BASE_URL") {
        Ok(url) => {
            Client::with_base_url(std::env::var("GROUNDMAIL_API_KEY").unwrap_or_default(), url)?
        }
        Err(_) => Client::from_env()?,
    };
    let email = SendEmail::new(from, to, "Hello from Rust").html("<p>It works.</p>");
    let sent = client.emails().send(&email).await?;
    println!("{} {}", sent.id, sent.status);
    Ok(())
}
