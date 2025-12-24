use aws_config::BehaviorVersion;
use aws_sdk_sts::Client;
use serde_json::json;
use clap::Parser;

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
        let git_commit = std::env::var("GIT_COMMIT").unwrap_or_else(|_| "unknown".to_string());
        let release_ver = std::env::var("RELEASE_VER").unwrap_or_else(|_| built_info::PKG_VERSION.to_string());
        let release_date = std::env::var("RELEASE_DATE").unwrap_or_else(|_| "unknown".to_string());
        
        println!("aws-whoami");
        println!("Version: {}", release_ver);
        println!("Git Commit: {}", git_commit);
        println!("Release Date: {}", release_date);
        return Ok(());
    }
    
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
