use jira::Jira;
use rig::providers::ollama::{self, Client};
use std::{env, error::Error};

mod jira;

pub struct Tools {
    message: String,
    client: Client,
}

impl Tools {
    pub fn init(message: &String) -> Self {
        let url = env::var("OLLAMA_URL").expect("OLLAMA_URL must be set");
        let client = ollama::Client::from_url(&url);

        Tools {
            message: message.to_owned(),
            client,
        }
    }

    pub async fn jira(&self) -> Result<(), Box<dyn Error>> {
        let model = env::var("JIRA_AGENT_MODEL").expect("JIRA_AGENT_MODEL must be set");
        let response = Jira::create_tool(&self.client, model.as_str())
            .prompt(&self.message)
            .await?;
        println!("Jira agent response: {}", response);
        Ok(())
    }
}
