//! Official Rust client for the [Groundmail](https://groundmail.dev) transactional email API.
//!
//! ```no_run
//! use groundmail::{Client, SendEmail};
//!
//! # async fn run() -> Result<(), groundmail::Error> {
//! let client = Client::new("gm_your_key_here")?;
//! let email = SendEmail::new("you@yourdomain.com", "user@example.com", "Hello").html("<p>It works.</p>");
//! let sent = client.emails().send(&email).await?;
//! println!("{}", sent.id);
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
mod types;
mod webhooks;

pub use client::{Client, Domains, Emails, Suppressions, Templates, Webhooks, DEFAULT_BASE_URL};
pub use error::Error;
pub use types::{ListOptions, SendEmail, SendResponse};
pub use webhooks::{
    verify_webhook_signature, verify_webhook_signature_with_tolerance, DEFAULT_TOLERANCE,
};
