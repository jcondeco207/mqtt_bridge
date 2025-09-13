mod structs_definitions;

use anyhow::Error;
use axum::http::HeaderValue;
use structs_definitions::AuthPayload;

use crate::topics_manager_client::structs_definitions::RegisterPayload;

pub struct TopicsManagerClient {
    username: String,
    password: String,
    address: String,
    port: i32,
    auth_token: Option<String>,
}

impl TopicsManagerClient {
    pub fn new(username: &str, password: &str, address: &str, port: i32) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            address: address.to_string(),
            port: port,
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

    pub fn has_auth_token(&mut self) -> bool {
        self.auth_token.is_some()
    }

    pub async fn register(&mut self) -> Result<(), Error> {
        if !self.has_auth_token() {
            self.renew_auth_token().await?;
        }

        let client = reqwest::Client::builder().build()?;

        // Set headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        if let Some(ref token) = self.auth_token {
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        }

        let register_payload: RegisterPayload = RegisterPayload {
            host: self.address.clone(),
            port: self.port,
        };

        let resp = client
            .post(format!("{}:{}/mqtt_bridge", self.address, self.port))
            .headers(headers)
            .json(&register_payload)
            .send()
            .await?;


        // Check status before consuming the response
        if !resp.status().is_success() {
            return Err(Error::msg(format!(
                "Auth request failed with status: {}",
                resp.status()
            )));
        }
        
        Ok(())
    }
}
