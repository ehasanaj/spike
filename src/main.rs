mod tools;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let response = tools::jira::services::issues_service::comment_on_issue("ROECIA-423", "m").await;
    match response {
        Ok(_) => println!("Commented on issue"),
        Err(e) => eprintln!("Failed to comment on issue: {}", e),
    }
    Ok(())
}
