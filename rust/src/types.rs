use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One email to send. Set `html`, `text` or a template with [`SendEmail::field`].
#[derive(Debug, Clone, Serialize)]
pub struct SendEmail {
    from: String,
    to: Vec<String>,
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cc: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bcc: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
    #[serde(flatten)]
    fields: BTreeMap<String, Value>,
    #[serde(skip)]
    pub(crate) idempotency_key: Option<String>,
}

impl SendEmail {
    pub fn new(from: impl Into<String>, to: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: vec![to.into()],
            subject: subject.into(),
            html: None,
            text: None,
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: None,
            fields: BTreeMap::new(),
            idempotency_key: None,
        }
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Adds another recipient.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to.push(to.into());
        self
    }

    pub fn cc(mut self, cc: impl Into<String>) -> Self {
        self.cc.push(cc.into());
        self
    }

    pub fn bcc(mut self, bcc: impl Into<String>) -> Self {
        self.bcc.push(bcc.into());
        self
    }

    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    /// Makes a retry of this send return the first result instead of sending twice.
    pub fn idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Any other field of `POST /emails`, such as `tags` or `scheduled_at`.
    pub fn field(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.fields.insert(name.into(), value.into());
        self
    }
}

/// What `POST /emails` returns. Fields beyond these are in `extra`.
#[derive(Debug, Clone, Deserialize)]
pub struct SendResponse {
    pub id: String,
    pub status: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Paging for list calls.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl ListOptions {
    pub(crate) fn query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.clone()));
        }
        query
    }
}
