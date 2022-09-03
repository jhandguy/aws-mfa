use anyhow::{anyhow, Result};
use aws_config::load_from_env;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_sts::config::Builder;
use aws_sdk_sts::{Client, Region};

pub async fn get_client(profile: &str, suffix: &str, region: String) -> Client {
    let sdk_config = load_from_env().await;
    let provider = ProfileFileCredentialsProvider::builder()
        .profile_name(format!("{}-{}", profile, suffix))
        .build();
    let config = Builder::from(&sdk_config)
        .region(Region::new(region))
        .credentials_provider(provider)
        .build();

    Client::from_conf(config)
}

pub async fn get_mfa_device_arn(client: &Client) -> Result<String> {
    let identity = client.get_caller_identity().send().await?;

    let account = identity
        .account()
        .ok_or_else(|| anyhow!("account identity missing"))?;
    let user = identity
        .arn()
        .ok_or_else(|| anyhow!("account arn missing"))?
        .split('/')
        .last()
        .ok_or_else(|| anyhow!("cannot parse arn"))?;
    let arn = format!("arn:aws:iam::{}:mfa/{}", account, user);

    Ok(arn)
}

pub async fn get_auth_credential(
    client: &Client,
    profile: &str,
    arn: &str,
    code: &str,
    duration: i32,
) -> Result<String> {
    let session = client
        .get_session_token()
        .serial_number(arn)
        .token_code(code)
        .duration_seconds(duration)
        .send()
        .await?;

    let credentials = session
        .credentials()
        .ok_or_else(|| anyhow!("credentials field missing"))?;
    let access_key_id = credentials
        .access_key_id()
        .ok_or_else(|| anyhow!("access_key_id field missing"))?;
    let secret_access_key = credentials
        .secret_access_key()
        .ok_or_else(|| anyhow!("secret_access_key field missing"))?;
    let session_token = credentials
        .session_token()
        .ok_or_else(|| anyhow!("session_token field missing"))?;

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

#[cfg(test)]
mod tests {
    use crate::{get_auth_credential, get_mfa_device_arn};
    use anyhow::Result;
    use aws_sdk_sts::{Client, Config, Credentials, Region};
    use aws_smithy_client::test_connection::capture_request;
    use aws_smithy_http::body::SdkBody;
    use http::Response;

    #[tokio::test]
    async fn test_get_mfa_device_arn() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
        let conf = Config::builder()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .build();
        let response = Response::builder().status(200).body(SdkBody::from(
            "
        <GetCallerIdentityResponse>
            <GetCallerIdentityResult>
                <UserId>user_id</UserId>
                <Account>account</Account>
                <Arn>arn:aws:iam::account:user/user_name</Arn>
            </GetCallerIdentityResult>
        </GetCallerIdentityResponse>",
        ))?;
        let (conn, _request) = capture_request(Some(response));
        let client = Client::from_conf_conn(conf, conn);
        let arn = get_mfa_device_arn(&client).await?;

        assert_eq!(arn, "arn:aws:iam::account:mfa/user_name");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_auth_credential() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
        let conf = Config::builder()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .build();
        let response = Response::builder().status(200).body(SdkBody::from(
            "
        <GetSessionTokenResponse>
            <GetSessionTokenResult>
                <Credentials>
                    <AccessKeyId>access_key_id</AccessKeyId>
                    <SecretAccessKey>secret_access_key</SecretAccessKey>
                    <SessionToken>session_token</SessionToken>
                    <Expiration>2022-08-31T19:55:58Z</Expiration>
                </Credentials>
            </GetSessionTokenResult>
        </GetSessionTokenResponse>",
        ))?;
        let (conn, _request) = capture_request(Some(response));
        let client = Client::from_conf_conn(conf, conn);
        let arn = get_auth_credential(&client, "profile", "arn", "code", 0).await?;

        assert_eq!(
            arn,
            "

[profile]
aws_access_key_id = access_key_id
aws_secret_access_key = secret_access_key
aws_session_token = session_token"
        );

        Ok(())
    }
}
