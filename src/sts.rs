use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_credential_types::provider::ProvideCredentials;
use aws_sdk_sts::config::Builder;
use aws_sdk_sts::Client;

use crate::error::Error;
use crate::error::Error::{
    GetCallerIdentityError, GetSessionTokenError, InvalidCredentials, InvalidIdentity,
    InvalidSession, Other,
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
    let access_key_id = credentials
        .access_key_id()
        .ok_or_else(|| InvalidCredentials(String::from("access_key_id")))?;
    let secret_access_key = credentials
        .secret_access_key()
        .ok_or_else(|| InvalidCredentials(String::from("secret_access_key")))?;
    let session_token = credentials
        .session_token()
        .ok_or_else(|| InvalidCredentials(String::from("session_token")))?;
    let expiration = credentials
        .expiration()
        .ok_or_else(|| InvalidCredentials(String::from("expiration")))?;

    Ok(Credentials::new(
        access_key_id,
        secret_access_key,
        session_token,
        expiration.secs(),
    ))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use aws_credential_types::Credentials;
    use aws_sdk_sts::config::Region;
    use aws_sdk_sts::{Client, Config};
    use aws_smithy_client::test_connection::capture_request;
    use aws_smithy_http::body::SdkBody;
    use aws_smithy_types::date_time::Format;
    use aws_smithy_types::DateTime;
    use http::header::CONTENT_TYPE;
    use http::{HeaderValue, Method, Response};

    use crate::sts::{get_auth_credentials, get_mfa_device_arn};

    #[tokio::test]
    async fn test_get_mfa_device_arn_without_identifier() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
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
        let (conn, request) = capture_request(Some(response));
        let conf = Config::builder()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_connector(conn)
            .build();
        let client = Client::from_conf(conf);
        let arn = get_mfa_device_arn(&client, None).await?;

        let request = request.expect_request();
        assert_eq!(Method::POST, request.method());
        assert_eq!("https://sts.eu-west-1.amazonaws.com/", request.uri());
        assert_eq!(
            Some(&HeaderValue::from_static(
                "application/x-www-form-urlencoded"
            )),
            request.headers().get(CONTENT_TYPE)
        );
        assert_eq!(arn, "arn:aws:iam::account:mfa/user_name");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_mfa_device_arn_with_identifier() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
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
        let (conn, request) = capture_request(Some(response));
        let conf = Config::builder()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_connector(conn)
            .build();
        let client = Client::from_conf(conf);
        let arn = get_mfa_device_arn(&client, Some(String::from("device_id"))).await?;

        let request = request.expect_request();
        assert_eq!(Method::POST, request.method());
        assert_eq!("https://sts.eu-west-1.amazonaws.com/", request.uri());
        assert_eq!(
            Some(&HeaderValue::from_static(
                "application/x-www-form-urlencoded"
            )),
            request.headers().get(CONTENT_TYPE)
        );
        assert_eq!(arn, "arn:aws:iam::account:mfa/device_id");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_auth_credentials() -> Result<()> {
        let credentials = Credentials::new("", "", None, None, "");
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
        let (conn, request) = capture_request(Some(response));
        let conf = Config::builder()
            .region(Region::new("eu-west-1"))
            .credentials_provider(credentials)
            .http_connector(conn)
            .build();
        let client = Client::from_conf(conf);
        let credentials = get_auth_credentials(&client, "arn", "code", 0).await?;

        let request = request.expect_request();
        assert_eq!(Method::POST, request.method());
        assert_eq!("https://sts.eu-west-1.amazonaws.com/", request.uri());
        assert_eq!(
            Some(&HeaderValue::from_static(
                "application/x-www-form-urlencoded"
            )),
            request.headers().get(CONTENT_TYPE)
        );
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
