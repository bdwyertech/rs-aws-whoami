use aws_config::BehaviorVersion;
use aws_sdk_sts::Client;
use clap::Parser;
use serde_json::json;

#[cfg(target_os = "macos")]
use libz_sys as _;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
#[command(name = "aws-whoami")]
#[command(about = "Get AWS caller identity information")]
struct Cli {
    #[arg(short, long)]
    version: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.version {
        let git_commit = built_info::GIT_COMMIT_HASH_SHORT;
        let release_ver =
            std::env::var("RELEASE_VER").unwrap_or_else(|_| built_info::PKG_VERSION.to_string());
        let release_date = built_info::BUILT_TIME_UTC;

        println!("aws-whoami");
        println!("Version: {}", release_ver);
        println!("Git Commit: {}", git_commit.unwrap_or("unknown"));
        println!("Release Date: {}", release_date);
        return Ok(());
    }

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    match client.get_caller_identity().send().await {
        Ok(resp) => {
            let output = json!({
                "UserId": resp.user_id(),
                "Account": resp.account(),
                "Arn": resp.arn()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Err(e) => {
            let error_str = format!("{:?}", e);

            // Extract code from ErrorMetadata
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

            // Extract message from ErrorMetadata
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
            eprintln!("{}", serde_json::to_string_pretty(&error_output)?);
            std::process::exit(1);
        }
    }
    Ok(())
}
