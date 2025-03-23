use std::error::Error;

use reqwest::StatusCode;
use serde_json::json;
use crate::tools::jira::services::jira_service;

#[derive(Debug, thiserror::Error)]
pub enum IssuesError {
    #[error("Failed to comment on issue: {0} - {1}")]
    FailedToCommentOnIssue(StatusCode, String),
}

pub async fn comment_on_issue(issue_key: &str, comment: &str) -> Result<(), Box<dyn Error>> {
    let response = jira_service::post(&format!("issue/{}/comment", issue_key), &json!({ "body": {
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
        } })).await?;

    if !response.status().is_success() {
        let status_code = response.status();
        let response_body = response.text().await?;
        return Err(IssuesError::FailedToCommentOnIssue(status_code, response_body).into());
    }
    Ok(())
}