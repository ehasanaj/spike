mod tools;

use dotenv::dotenv;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let response = tools::jira::services::issues_service::comment_on_issue("PROJ-122", "m").await;
    match response {
        Ok(_) => println!("Commented on issue"),
        Err(e) => eprintln!("Failed to comment on issue: {}", e),
    }
    Ok(())
}