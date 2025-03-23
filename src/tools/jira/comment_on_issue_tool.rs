use crate::tools::jira::services::issues_service;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct CommentOnIssueArgs {
    pub issue_key: String,
    pub comment: String,
}

#[derive(Serialize)]
pub struct CommentOnIssueResult {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CommentOnIssueError {}

pub struct CommentOnIssueTool;

impl Tool for CommentOnIssueTool {
    const NAME: &'static str = "comment_on_issue_tool";
    type Error = CommentOnIssueError;
    type Args = CommentOnIssueArgs;
    type Output = CommentOnIssueResult;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Adds a comment on a jira ticket".to_string(),
            parameters: json!({
                "issue_key": {
                    "type": "string",
                    "description": "This is the issue key for the jira ticket for example PROJ-123"
                },
                "comment": {
                    "type": "string",
                    "description": "This is the comment that will be added to the jira ticket"
                },
                "required": ["issue_key", "comment"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let response = issues_service::comment_on_issue(&args.issue_key, &args.comment).await;
        match response {
            Ok(_) => println!("Commented on issue"),
            Err(e) => eprintln!("Failed to comment on issue: {}", e),
        }
        Ok(CommentOnIssueResult {
            message: "Commented on issue".to_string(),
        })
    }
}
