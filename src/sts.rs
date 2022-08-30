use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_sts::error::{GetCallerIdentityError, GetSessionTokenError};
use aws_sdk_sts::types::SdkError;
use aws_sdk_sts::{config, Client, Region};

pub async fn get_client(profile: &str, suffix: &str, region: String) -> Client {
    let sdk_config = aws_config::load_from_env().await;
    let provider = ProfileFileCredentialsProvider::builder()
        .profile_name(format!("{}-{}", profile, suffix))
        .build();
    let config = config::Builder::from(&sdk_config)
        .region(Region::new(region))
        .credentials_provider(provider)
        .build();

    Client::from_conf(config)
}

pub async fn get_mfa_device_arn(
    client: &Client,
) -> Result<String, SdkError<GetCallerIdentityError>> {
    let identity = client.get_caller_identity().send().await?;

    let account = identity.account().unwrap();
    let user = identity.arn().unwrap().split('/').last().unwrap();
    let arn = format!("arn:aws:iam::{}:mfa/{}", account, user);

    Ok(arn)
}

pub async fn get_auth_credential(
    client: &Client,
    profile: &str,
    arn: &str,
    code: &str,
    duration: i32,
) -> Result<String, SdkError<GetSessionTokenError>> {
    let session = client
        .get_session_token()
        .serial_number(arn)
        .token_code(code)
        .duration_seconds(duration)
        .send()
        .await?;

    let credentials = session.credentials().unwrap();
    let access_key_id = credentials.access_key_id().unwrap();
    let secret_access_key = credentials.secret_access_key().unwrap();
    let session_token = credentials.session_token().unwrap();

    let credential = format!(
        "

[{}]
aws_access_key_id = {}
aws_secret_access_key = {}
aws_session_token = {}",
        profile, access_key_id, secret_access_key, session_token
    );

    Ok(credential)
}
