use anyhow::Result;
use async_trait::async_trait;
use clap::{Args, Parser, Subcommand};

use aws_mfa::{CredentialsProvider, EnvCredentialsProvider, FileCredentialsProvider};

use crate::Command::{Env, File};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Authenticate to AWS with MFA using config and credentials files
    File(FileArgs),

    /// Authenticate to AWS with MFA using environment variables
    Env(EnvArgs),
}

#[derive(Args)]
struct AuthArgs {
    /// MFA code
    #[arg(short, long)]
    code: String,

    /// MFA device identifier (defaults to AWS username)
    #[arg(short, long)]
    identifier: Option<String>,

    /// Session duration in seconds
    #[arg(short, long, default_value_t = 3600)]
    duration: i32,
}

#[derive(Args)]
struct FileArgs {
    #[command(flatten)]
    auth: AuthArgs,

    /// Home directory containing the AWS hidden folder
    #[arg(env = "HOME")]
    home: String,

    /// Name of the AWS region
    #[arg(short, long, env = "AWS_REGION")]
    region: Option<String>,

    /// Name of the AWS profile
    #[arg(short, long, default_value = "default", env = "AWS_PROFILE")]
    profile: String,

    /// Suffix of the original AWS profile
    #[arg(short, long, default_value = "noauth")]
    suffix: String,

    /// Force authentication even though current credentials are still valid
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct EnvArgs {
    #[command(flatten)]
    auth: AuthArgs,
}

#[async_trait]
trait Authenticate {
    async fn authenticate(&self) -> Result<()>;
}

impl Cli {
    fn args(self) -> Box<dyn Authenticate> {
        match self.command {
            File(args) => Box::new(args),
            Env(args) => Box::new(args),
        }
    }
}

#[async_trait]
impl Authenticate for FileArgs {
    async fn authenticate(&self) -> Result<()> {
        let provider = FileCredentialsProvider::new(
            &self.auth.code,
            &self.home,
            self.region.clone(),
            &self.profile,
            &self.suffix,
            self.auth.identifier.clone(),
            self.auth.duration,
        );

        if !self.force {
            if let Some(credentials) = provider.validate().await? {
                println!(
                    "Current credentials are still valid and will expire in {}.",
                    credentials.session_duration()?
                );
                println!("Use --force or -f to authenticate anyway.");

                return Ok(());
            }
        }

        println!("Authenticating...");

        let credentials = provider.authenticate().await?;

        println!("Authentication successful!");
        println!(
            "New credentials will expire in {}.",
            credentials.session_duration()?
        );

        Ok(())
    }
}

#[async_trait]
impl Authenticate for EnvArgs {
    async fn authenticate(&self) -> Result<()> {
        let provider = EnvCredentialsProvider::new(
            &self.auth.code,
            self.auth.identifier.clone(),
            self.auth.duration,
        );

        if let Some(credentials) = provider.validate().await? {
            println!(
                "echo \"Current credentials are still valid and will expire in {}.\"",
                credentials.session_duration()?,
            );

            return Ok(());
        }

        let credentials = provider.authenticate().await?;

        println!(
            "export AWS_ACCESS_KEY_ID=\"{}\"
export AWS_SECRET_ACCESS_KEY=\"{}\"
export AWS_SESSION_TOKEN=\"{}\"
export AWS_SESSION_EXPIRATION_TIMESTAMP=\"{}\" && \
echo \"Authentication successful!\" && \
echo \"New credentials will expire in {}.\"",
            credentials.access_key_id(),
            credentials.secret_access_key(),
            credentials.session_token(),
            credentials.session_expiration_timestamp(),
            credentials.session_duration()?,
        );

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Cli::parse().args().authenticate().await
}
