use std::{env, error::Error};

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum IssuesError {
    #[error("Missing environment variable: {0}")]
    MissingEnvironmentVariable(String),
    #[error("Failed to comment on issue: {0} - {1}")]
    FailedToCommentOnIssue(StatusCode, String),
}


pub async fn comment_on_issue(issue_key: &str, comment: &str) -> Result<(), Box<dyn Error>> {
    let jira_url = env::var("JIRA_BASE_URL").map_err(|_| IssuesError::MissingEnvironmentVariable("JIRA_URL".to_string()))?; 
    let jira_username = env::var("JIRA_USERNAME").map_err(|_| IssuesError::MissingEnvironmentVariable("JIRA_USERNAME".to_string()))?;
    let jira_api_token = env::var("JIRA_API_TOKEN").map_err(|_| IssuesError::MissingEnvironmentVariable("JIRA_API_TOKEN".to_string()))?;

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/rest/api/3/issue/{}/comment", jira_url, issue_key))
        .basic_auth(jira_username, Some(jira_api_token))
        .json(&serde_json::json!({ "body": {
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": comment
                        }
                    ]
                }
            ]
        } }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status_code = response.status();
        let response_body = response.text().await?;
        return Err(IssuesError::FailedToCommentOnIssue(status_code, response_body).into());
    }
    Ok(())
}