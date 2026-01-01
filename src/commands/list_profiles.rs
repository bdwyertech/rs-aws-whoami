use serde_json::json;
use crate::commands::whoami;
use tokio::task;
use aws_types::os_shim_internal::{Fs, Env};
use aws_runtime::env_config::file::EnvConfigFiles;

pub async fn execute(with_whoami: bool) -> Result<(), Box<dyn std::error::Error>> {
    match list_aws_profiles().await {
        Ok(profiles) => {
            if with_whoami {
                let mut tasks = Vec::new();
                
                for profile in profiles.clone() {
                    let task = task::spawn(async move {
                        let result = whoami::execute_with_profile(Some(&profile)).await.unwrap_or_else(|_| {
                            json!({
                                "error": true,
                                "message": "Failed to execute whoami"
                            })
                        });
                        (profile, result)
                    });
                    tasks.push(task);
                }
                
                let mut results = serde_json::Map::new();
                for task in tasks {
                    let (profile, result) = task.await?;
                    results.insert(profile, result);
                }
                
                let output = json!({ "profiles": results });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                let output = json!({ "profiles": profiles });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
        Err(e) => {
            let error_output = json!({
                "error": true,
                "message": format!("Failed to list profiles: {}", e)
            });
            eprintln!("{}", serde_json::to_string_pretty(&error_output)?);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn list_aws_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let fs = Fs::real();
    let env = Env::real();
    let profile_files = EnvConfigFiles::default();
    let profiles_set = aws_config::profile::load(&fs, &env, &profile_files, None).await?;
    let mut profiles: Vec<String> = profiles_set.profiles().map(|p| p.to_string()).collect();
    profiles.sort();
    Ok(profiles)
}
