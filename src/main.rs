use aws_config::BehaviorVersion;
use aws_sdk_sts::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    
    let resp = client.get_caller_identity().send().await?;
    
    let output = json!({
        "UserId": resp.user_id(),
        "Account": resp.account(),
        "Arn": resp.arn()
    });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
