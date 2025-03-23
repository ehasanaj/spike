use std::{env, error::Error};
use reqwest::Response;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum JiraApiError {
    #[error("Missing environment variable: {0}")]
    MissingEnvironmentVariable(String),
}

pub async fn post(path: &str, body: Value) -> Result<Response, Box<dyn Error>> {
    let base_url = get_env_var("JIRA_BASE_URL")?;
    let username = get_env_var("JIRA_USERNAME")?;
    let api_token = get_env_var("JIRA_API_TOKEN")?;

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/rest/api/3/{}", base_url, path))
        .basic_auth(username, Some(api_token))
        .json(&body)
        .send()
        .await?;
    Ok(response)
}

fn get_env_var(name: &str) -> Result<String, Box<dyn Error>> {
   let value = env::var(name).map_err(|_| JiraApiError::MissingEnvironmentVariable(name.to_string()))?;
   Ok(value) 
}