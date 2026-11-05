use serde_json::Value;

/// Anything that can go wrong calling Groundmail.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The API key is missing or isn't a Groundmail key.
    #[error("{0}")]
    InvalidApiKey(String),
    /// Groundmail answered with an error.
    #[error("{message}")]
    Api {
        /// The HTTP status.
        status: u16,
        message: String,
        /// A stable code such as `validation_error`, when there is one.
        code: Option<String>,
        details: Option<Value>,
        /// Where to upgrade, when a plan limit was reached.
        upgrade_url: Option<String>,
    },
    /// The request didn't complete.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    /// The response wasn't what the client expected.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Error {
    pub(crate) fn from_response(status: u16, body: &Value) -> Self {
        let error = body.get("error");
        let text = |key: &str| {
            error
                .and_then(|e| e.get(key))
                .and_then(Value::as_str)
                .map(str::to_owned)
        };
        let message = error
            .and_then(Value::as_str)
            .map(str::to_owned)
            .or_else(|| text("message"))
            .unwrap_or_else(|| format!("Request failed with status {status}"));
        Error::Api {
            status,
            message,
            code: text("code"),
            details: error.and_then(|e| e.get("details")).cloned(),
            upgrade_url: text("upgrade_url"),
        }
    }
}
