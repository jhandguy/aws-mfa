use anyhow::Result;
use clap::Parser;

use aws_mfa::auth::authenticate;

#[derive(Parser, Default)]
#[clap(version, about)]
pub struct Args {
    /// Name of the AWS region
    #[clap(short, long, default_value = "eu-west-1")]
    pub region: String,

    /// Name of the AWS profile
    #[clap(short, long, default_value = "default")]
    pub profile: String,

    /// Suffix of the original AWS profile
    #[clap(short, long, default_value = "noauth")]
    pub suffix: String,

    /// MFA code
    #[clap(short, long)]
    pub code: String,

    /// Session duration in seconds
    #[clap(short, long, default_value_t = 3600)]
    pub duration: i32,

    /// Home directory containing the AWS hidden folder
    #[clap(env)]
    pub home: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    authenticate(
        &args.profile,
        &args.suffix,
        &args.region,
        &args.code,
        args.duration,
        &args.home,
    )
    .await?;

    Ok(())
}
