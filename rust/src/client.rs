use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::types::{ListOptions, SendEmail, SendResponse};
use crate::Error;

pub const DEFAULT_BASE_URL: &str = "https://api.groundmail.dev";

const USER_AGENT: &str = concat!("groundmail-rust/", env!("CARGO_PKG_VERSION"));

fn segment(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

/// A Groundmail API client. Cheap to clone; share one across your app.
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }

    /// Reads the key from `GROUNDMAIL_API_KEY`.
    pub fn from_env() -> Result<Self, Error> {
        let key = std::env::var("GROUNDMAIL_API_KEY")
            .map_err(|_| Error::InvalidApiKey("GROUNDMAIL_API_KEY is not set".to_owned()))?;
        Self::new(key)
    }

    pub fn with_base_url(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        Self::with_http_client(api_key, base_url, http)
    }

    /// Bring your own `reqwest::Client`, for timeouts or a proxy.
    pub fn with_http_client(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        http: reqwest::Client,
    ) -> Result<Self, Error> {
        let api_key = api_key.into();
        if !api_key.starts_with("gm_") {
            return Err(Error::InvalidApiKey(
                "Groundmail API key must start with gm_".to_owned(),
            ));
        }
        Ok(Self {
            api_key,
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            http,
        })
    }

    pub fn emails(&self) -> Emails<'_> {
        Emails { client: self }
    }

    pub fn domains(&self) -> Domains<'_> {
        Domains { client: self }
    }

    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    pub fn templates(&self) -> Templates<'_> {
        Templates { client: self }
    }

    pub fn suppressions(&self) -> Suppressions<'_> {
        Suppressions { client: self }
    }

    /// Calls any endpoint, for what the typed methods don't cover yet.
    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, Error> {
        let mut request = self.builder(method, path);
        if let Some(body) = body {
            request = request.json(body);
        }
        self.run(request).await
    }

    fn builder(&self, method: Method, path: &str) -> RequestBuilder {
        self.http
            .request(method, format!("{}{}", self.base_url, path))
            .bearer_auth(&self.api_key)
            .header(reqwest::header::USER_AGENT, USER_AGENT)
    }

    async fn run<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, Error> {
        let response = request.send().await?;
        let status = response.status();
        let raw = response.bytes().await?;
        if !status.is_success() {
            let body = serde_json::from_slice(&raw).unwrap_or(Value::Null);
            return Err(Error::from_response(status.as_u16(), &body));
        }
        if raw.is_empty() {
            return Ok(serde_json::from_value(json!({}))?);
        }
        Ok(serde_json::from_slice(&raw)?)
    }

    async fn get(&self, path: &str, query: &[(&str, String)]) -> Result<Value, Error> {
        self.run(self.builder(Method::GET, path).query(query)).await
    }

    async fn send_json(&self, method: Method, path: &str, body: &Value) -> Result<Value, Error> {
        self.run(self.builder(method, path).json(body)).await
    }

    async fn delete(&self, path: &str) -> Result<Value, Error> {
        self.run(self.builder(Method::DELETE, path)).await
    }
}

/// Sending and reading emails.
#[derive(Debug)]
pub struct Emails<'a> {
    client: &'a Client,
}

impl Emails<'_> {
    pub async fn send(&self, email: &SendEmail) -> Result<SendResponse, Error> {
        let mut request = self.client.builder(Method::POST, "/emails").json(email);
        if let Some(key) = &email.idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        self.client.run(request).await
    }

    pub async fn batch(&self, emails: &[SendEmail]) -> Result<Value, Error> {
        self.client
            .run(
                self.client
                    .builder(Method::POST, "/emails/batch")
                    .json(&json!({ "emails": emails })),
            )
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Value, Error> {
        self.client
            .get(&format!("/emails/{}", segment(id)), &[])
            .await
    }

    pub async fn list(&self, options: &ListOptions) -> Result<Value, Error> {
        self.client.get("/emails", &options.query()).await
    }

    /// Cancels a scheduled email.
    pub async fn cancel(&self, id: &str) -> Result<Value, Error> {
        self.client
            .send_json(
                Method::PATCH,
                &format!("/emails/{}", segment(id)),
                &json!({ "cancel": true }),
            )
            .await
    }
}

/// Sending domains.
#[derive(Debug)]
pub struct Domains<'a> {
    client: &'a Client,
}

impl Domains<'_> {
    pub async fn list(&self, options: &ListOptions) -> Result<Value, Error> {
        self.client.get("/domains", &options.query()).await
    }

    pub async fn get(&self, id_or_name: &str) -> Result<Value, Error> {
        self.client
            .get(&format!("/domains/{}", segment(id_or_name)), &[])
            .await
    }

    pub async fn create(&self, name: &str) -> Result<Value, Error> {
        self.client
            .send_json(Method::POST, "/domains", &json!({ "name": name }))
            .await
    }

    pub async fn verify(&self, id_or_name: &str) -> Result<Value, Error> {
        let path = format!("/domains/{}", segment(id_or_name));
        self.client
            .send_json(Method::POST, &path, &json!({ "action": "verify" }))
            .await
    }

    pub async fn delete(&self, id_or_name: &str) -> Result<Value, Error> {
        self.client
            .delete(&format!("/domains/{}", segment(id_or_name)))
            .await
    }
}

/// Webhook endpoints. Use [`crate::verify_webhook_signature`] for incoming events.
#[derive(Debug)]
pub struct Webhooks<'a> {
    client: &'a Client,
}

impl Webhooks<'_> {
    pub async fn list(&self, options: &ListOptions) -> Result<Value, Error> {
        self.client.get("/webhooks", &options.query()).await
    }

    pub async fn get(&self, id: &str) -> Result<Value, Error> {
        self.client
            .get(&format!("/webhooks/{}", segment(id)), &[])
            .await
    }

    pub async fn create(&self, body: &Value) -> Result<Value, Error> {
        self.client.send_json(Method::POST, "/webhooks", body).await
    }

    pub async fn update(&self, id: &str, body: &Value) -> Result<Value, Error> {
        self.client
            .send_json(Method::PATCH, &format!("/webhooks/{}", segment(id)), body)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<Value, Error> {
        self.client
            .delete(&format!("/webhooks/{}", segment(id)))
            .await
    }
}

/// Email templates.
#[derive(Debug)]
pub struct Templates<'a> {
    client: &'a Client,
}

impl Templates<'_> {
    pub async fn list(&self, options: &ListOptions) -> Result<Value, Error> {
        self.client.get("/templates", &options.query()).await
    }

    pub async fn get(&self, id_or_alias: &str) -> Result<Value, Error> {
        self.client
            .get(&format!("/templates/{}", segment(id_or_alias)), &[])
            .await
    }

    pub async fn create(&self, body: &Value) -> Result<Value, Error> {
        self.client
            .send_json(Method::POST, "/templates", body)
            .await
    }

    pub async fn update(&self, id_or_alias: &str, body: &Value) -> Result<Value, Error> {
        self.client
            .send_json(
                Method::PATCH,
                &format!("/templates/{}", segment(id_or_alias)),
                body,
            )
            .await
    }

    pub async fn publish(&self, id_or_alias: &str) -> Result<Value, Error> {
        let path = format!("/templates/{}", segment(id_or_alias));
        self.client
            .send_json(Method::POST, &path, &json!({ "action": "publish" }))
            .await
    }

    pub async fn delete(&self, id_or_alias: &str) -> Result<Value, Error> {
        self.client
            .delete(&format!("/templates/{}", segment(id_or_alias)))
            .await
    }
}

/// Addresses you won't send to.
#[derive(Debug)]
pub struct Suppressions<'a> {
    client: &'a Client,
}

impl Suppressions<'_> {
    pub async fn list(&self, options: &ListOptions) -> Result<Value, Error> {
        self.client.get("/suppressions", &options.query()).await
    }

    pub async fn create(&self, body: &Value) -> Result<Value, Error> {
        self.client
            .send_json(Method::POST, "/suppressions", body)
            .await
    }

    /// Removes a suppression by id, or by email address.
    pub async fn remove(&self, id_or_email: &str) -> Result<Value, Error> {
        if id_or_email.contains('@') {
            let request = self
                .client
                .builder(Method::DELETE, "/suppressions")
                .query(&[("email", id_or_email)]);
            return self.client.run(request).await;
        }
        self.client
            .delete(&format!("/suppressions/{}", segment(id_or_email)))
            .await
    }
}
