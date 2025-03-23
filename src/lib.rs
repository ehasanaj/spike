mod tools;
mod utils;

use dotenv::dotenv;
use tools::Tools;
use utils::command_line_args::CommandLineArgs;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let message = CommandLineArgs::parse().get_message();
    let tools = Tools::init(&message);

    tools.jira().await?;

    Ok(())
}
