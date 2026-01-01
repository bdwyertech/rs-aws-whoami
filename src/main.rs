use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;

mod commands;

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
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List all known AWS profiles")]
    ListProfiles {
        #[arg(long, help = "Execute whoami for each profile")]
        whoami: bool,
    },
    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell to generate completions for")]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.version {
        let git_commit = built_info::GIT_COMMIT_HASH_SHORT;
        let release_ver = option_env!("BUILD_VERSION").unwrap_or(built_info::PKG_VERSION);
        let release_date = built_info::BUILT_TIME_UTC;

        println!("aws-whoami");
        println!("Version: {}", release_ver);
        println!("Git Commit: {}", git_commit.unwrap_or("unknown"));
        println!("Release Date: {}", release_date);
        return Ok(());
    }

    match cli.command {
        Some(Commands::ListProfiles { whoami }) => {
            commands::list_profiles::execute(whoami).await?;
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "aws-whoami", &mut io::stdout());
        }
        None => {
            commands::whoami::execute().await?;
        }
    }
    Ok(())
}
