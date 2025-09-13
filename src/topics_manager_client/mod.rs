mod structs_definitions;

use anyhow::Error;
use structs_definitions::AuthPayload;

pub struct TopicsManagerClient {
    username: String,
    password: String,
    address: String,
    port: String,
    auth_token: Option<String>,
}

impl TopicsManagerClient {
    pub fn new(username: &str, password: &str, address: &str, port: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            address: address.to_string(),
            port: port.to_string(),
            auth_token: None,
        }
    }

    pub async fn renew_auth_token(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::builder().build()?;

        // Set headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);

        // Set body payload
        let auth_payload = AuthPayload {
            username: self.username.clone(),
            password: self.password.clone(),
        };

        let resp = client
            .post(format!("{}:{}/auth/login", self.address, self.port))
            .headers(headers)
            .json(&auth_payload)
            .send()
            .await?;

        // Check status before consuming the response
        if !resp.status().is_success() {
            return Err(Error::msg(format!(
                "Auth request failed with status: {}",
                resp.status()
            )));
        }

        // Extract JSON and auth token
        let token = resp.text().await?;
        self.auth_token = Some(token);
        Ok(())
        // let body: serde_json::Value = serde_json::from_str(&text)?;
        // if let Some(token) = body.get("token").and_then(|t| t.as_str()) {
        //     self.auth_token = Some(token.to_string());
        //     Ok(())
        // } else {
        //     Err(Error::msg("No token field in response"))
        // }
    }

    pub async fn has_auth_token(&mut self) -> bool {
        self.auth_token.is_some()
    }
}
