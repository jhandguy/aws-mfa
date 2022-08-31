use crate::args::parse_args;
use crate::io::save_auth_credential;
use crate::sts::{get_auth_credential, get_client, get_mfa_device_arn};

mod args;
mod error;
mod io;
mod sts;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = parse_args();
    let client = get_client(&args.profile, &args.suffix, args.region).await;
    let arn = get_mfa_device_arn(&client).await?;
    let credential =
        get_auth_credential(&client, &args.profile, &arn, &args.code, args.duration).await?;
    save_auth_credential(&args.home, &args.profile, &credential)?;

    Ok(())
}
