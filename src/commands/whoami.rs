use aws_config::BehaviorVersion;
use aws_sdk_sts::Client;
use serde_json::json;

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let result = execute_with_profile(None).await?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

pub async fn execute_with_profile(profile: Option<&str>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let config = if let Some(profile_name) = profile {
        aws_config::defaults(BehaviorVersion::latest())
            .profile_name(profile_name)
            .load()
            .await
    } else {
        aws_config::load_defaults(BehaviorVersion::latest()).await
    };
    
    let client = Client::new(&config);

    match client.get_caller_identity().send().await {
        Ok(resp) => {
            let output = json!({
                "UserId": resp.user_id(),
                "Account": resp.account(),
                "Arn": resp.arn()
            });
            
            Ok(output)
        }
        Err(e) => {
            let error_str = format!("{:?}", e);

            let code = if let Some(start) = error_str.find("code: Some(\"") {
                let start = start + 12;
                if let Some(end) = error_str[start..].find("\"") {
                    Some(error_str[start..start + end].to_string())
                } else {
                    None
                }
            } else {
                None
            };

            let message = if let Some(start) = error_str.find("message: Some(\"") {
                let start = start + 15;
                if let Some(end) = error_str[start..].find("\"") {
                    error_str[start..start + end].to_string()
                } else {
                    "Unknown error".to_string()
                }
            } else {
                "Unknown error".to_string()
            };

            let error_output = json!({
                "error": true,
                "message": message,
                "code": code
            });
            
            Ok(error_output)
        }
    }
}
