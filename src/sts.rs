use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_credential_types::provider::ProvideCredentials;
use aws_sdk_sts::config::Builder;
use aws_sdk_sts::Client;

use crate::error::Error;
use crate::error::Error::{
    GetCallerIdentityError, GetSessionTokenError, InvalidIdentity, InvalidSession, Other,
};
use crate::Credentials;

pub fn get_client(config: &SdkConfig, provider: impl ProvideCredentials + 'static) -> Client {
    let builder = Builder::from(config).credentials_provider(provider);

    Client::from_conf(builder.build())
}

pub async fn get_mfa_device_arn(
    client: &Client,
    identifier: Option<String>,
) -> Result<String, Error> {
    let identity = client
        .get_caller_identity()
        .send()
        .await
        .map_err(GetCallerIdentityError)?;

    let account = identity
        .account()
        .ok_or_else(|| InvalidIdentity(String::from("account")))?;

    let arn = identity
        .arn()
        .ok_or_else(|| InvalidIdentity(String::from("arn")))?;

    let user = arn
        .split('/')
        .last()
        .ok_or_else(|| Other(anyhow!("could not extract user in arn `{}`", arn)))?;

    let identifier = match identifier {
        Some(i) => i,
        None => String::from(user),
    };

    let arn = format!("arn:aws:iam::{account}:mfa/{identifier}");

    Ok(arn)
}

pub async fn get_auth_credentials(
    client: &Client,
    arn: &str,
    code: &str,
    duration: i32,
) -> Result<Credentials, Error> {
    let session = client
        .get_session_token()
        .serial_number(arn)
        .token_code(code)
        .duration_seconds(duration)
        .send()
        .await
        .map_err(GetSessionTokenError)?;

    let credentials = session
        .credentials()
        .ok_or_else(|| InvalidSession(String::from("credentials")))?;

    Ok(Credentials::new(
        credentials.access_key_id(),
        credentials.secret_access_key(),
        credentials.session_token(),
        credentials.expiration().secs(),
    ))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use aws_credential_types::Credentials;
    use aws_sdk_sts::config::Region;
    use aws_sdk_sts::{Client, Config};
    use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::date_time::Format;
    use aws_smithy_types::DateTime;
    use http::{Method, Request, Response};

    use crate::sts::{get_auth_credentials, get_mfa_device_arn};

    #[tokio::test]
    async fn test_get_mfa_device_arn_without_identifier() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://sts.eu-west-1.amazonaws.com/")
            .body(SdkBody::from("Action=GetCallerIdentity&Version=2011-06-15"))?;
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
        let replay_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);
        let conf = Config::builder()
            .behavior_version_latest()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_client(replay_client.clone())
            .build();
        let client = Client::from_conf(conf);
        let arn = get_mfa_device_arn(&client, None).await?;

        replay_client.assert_requests_match(&[]);
        assert_eq!(arn, "arn:aws:iam::account:mfa/user_name");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_mfa_device_arn_with_identifier() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://sts.eu-west-1.amazonaws.com/")
            .body(SdkBody::from("Action=GetCallerIdentity&Version=2011-06-15"))?;
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
        let replay_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);
        let conf = Config::builder()
            .behavior_version_latest()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_client(replay_client.clone())
            .build();
        let client = Client::from_conf(conf);
        let arn = get_mfa_device_arn(&client, Some(String::from("device_id"))).await?;

        replay_client.assert_requests_match(&[]);
        assert_eq!(arn, "arn:aws:iam::account:mfa/device_id");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_auth_credentials() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
        let duration = 0;
        let arn = "arn";
        let code = "code";
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://sts.eu-west-1.amazonaws.com/")
            .body(SdkBody::from(format!("Action=GetSessionToken&Version=2011-06-15&DurationSeconds={}&SerialNumber={}&TokenCode={}", duration, arn, code)))?;
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
        let replay_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);
        let conf = Config::builder()
            .behavior_version_latest()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_client(replay_client.clone())
            .build();
        let client = Client::from_conf(conf);
        let credentials = get_auth_credentials(&client, arn, code, duration).await?;

        replay_client.assert_requests_match(&[]);
        assert_eq!(credentials.access_key_id(), "access_key_id");
        assert_eq!(credentials.secret_access_key(), "secret_access_key");
        assert_eq!(credentials.session_token(), "session_token");
        assert_eq!(
            credentials.session_expiration_timestamp(),
            DateTime::from_str("2022-08-31T19:55:58Z", Format::DateTime)?.secs()
        );

        Ok(())
    }
}
