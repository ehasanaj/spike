use std::error::Error;

use comment_on_issue_tool::CommentOnIssueTool;
use rig::{agent::Agent, completion::Prompt, providers::ollama::{Client, CompletionModel}};

pub mod services;
pub mod comment_on_issue_tool;

pub struct Jira {
    pub agent: Agent<CompletionModel>,
}

impl Jira {
    pub fn create_tool(client: &Client, model: &str) -> Jira {
        let agent = client 
        .agent(model)
        .preamble("You are a helpfull assistant that can help with jira issues by doing the followeing things: comment on an issue. 
        If you cannot use a tool for handling the user request that you respond that you cannot help with the request.")
        .tool(CommentOnIssueTool)
        .build();

        Jira { agent }
    }

    pub async fn prompt(&self, message: &str) -> Result<String, Box<dyn Error>> {
        let response = self.agent.prompt(message).await?;
        Ok(response)
    }
}