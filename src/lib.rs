use std::time::SystemTime;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use time::{Duration, OffsetDateTime};

use crate::config::{get_env_config, get_env_provider, get_file_config, get_file_provider};
use crate::env::get_env_credentials;
use crate::error::Error;
use crate::error::Error::{ConvertSessionTimestampError, Other};
use crate::io::{find_auth_credentials, save_auth_credentials};
use crate::sts::{get_auth_credentials, get_client, get_mfa_device_arn};

mod config;
mod env;
pub mod error;
mod io;
mod sts;

/// Credentials received after authenticating to AWS with MFA
pub struct Credentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    session_expiration_timestamp: i64,
}

impl Credentials {
    pub fn new(
        access_key_id: &str,
        secret_access_key: &str,
        session_token: &str,
        session_expiration_timestamp: i64,
    ) -> Self {
        Self {
            access_key_id: String::from(access_key_id),
            secret_access_key: String::from(secret_access_key),
            session_token: String::from(session_token),
            session_expiration_timestamp,
        }
    }

    pub fn to_aws_credentials(&self) -> aws_credential_types::Credentials {
        aws_credential_types::Credentials::new(
            self.access_key_id(),
            self.secret_access_key(),
            Some(String::from(self.session_token())),
            Some(SystemTime::UNIX_EPOCH + Duration::seconds(self.session_expiration_timestamp())),
            "aws-mfa",
        )
    }

    pub fn access_key_id(&self) -> &str {
        &self.access_key_id
    }

    pub fn secret_access_key(&self) -> &str {
        &self.secret_access_key
    }

    pub fn session_token(&self) -> &str {
        &self.session_token
    }

    pub fn session_expiration_timestamp(&self) -> i64 {
        self.session_expiration_timestamp
    }

    pub fn session_duration(&self) -> Result<Duration, Error> {
        let session_duration =
            OffsetDateTime::from_unix_timestamp(self.session_expiration_timestamp)
                .map_err(ConvertSessionTimestampError)?
                - OffsetDateTime::now_utc();

        Ok(Duration::seconds(session_duration.whole_seconds()))
    }

    pub fn expired(&self) -> bool {
        OffsetDateTime::now_utc().unix_timestamp() > self.session_expiration_timestamp
    }
}

#[async_trait]
pub trait CredentialsProvider {
    async fn validate(&self) -> Result<Option<Credentials>, Error>;
    async fn authenticate(&self) -> Result<Credentials, Error>;
}

/// Provider for authenticating to AWS with MFA using config and credentials files
pub struct FileCredentialsProvider {
    code: String,
    home: String,
    region: Option<String>,
    profile: String,
    suffix: String,
    identifier: Option<String>,
    duration: i32,
}

impl FileCredentialsProvider {
    pub fn new(
        code: &str,
        home: &str,
        region: Option<String>,
        profile: &str,
        suffix: &str,
        identifier: Option<String>,
        duration: i32,
    ) -> Self {
        Self {
            code: String::from(code),
            home: String::from(home),
            region,
            profile: String::from(profile),
            suffix: String::from(suffix),
            identifier,
            duration,
        }
    }
}

#[async_trait]
impl CredentialsProvider for FileCredentialsProvider {
    /// Validate and return current [`Credentials`] from credentials file unless expired
    async fn validate(&self) -> Result<Option<Credentials>, Error> {
        if let Some(credentials) = find_auth_credentials(&self.home, &self.profile)? {
            if !credentials.expired() {
                return Ok(Some(credentials));
            }
        }

        Ok(None)
    }

    /// Authenticate using [`aws_config::profile::ProfileFileCredentialsProvider`] and return new [`Credentials`]
    async fn authenticate(&self) -> Result<Credentials, Error> {
        let config =
            get_file_config(&self.home, self.region.clone(), &self.profile, &self.suffix).await;
        let provider = get_file_provider(&self.profile, &self.suffix);
        let client = get_client(&config, provider);
        let arn = get_mfa_device_arn(&client, self.identifier.clone()).await?;
        let credentials = get_auth_credentials(&client, &arn, &self.code, self.duration).await?;

        save_auth_credentials(&self.home, &self.profile, &credentials)?;

        Ok(credentials)
    }
}

/// Provider for authenticating to AWS with MFA using environment variables
pub struct EnvCredentialsProvider {
    code: String,
    identifier: Option<String>,
    duration: i32,
}

impl EnvCredentialsProvider {
    pub fn new(code: &str, identifier: Option<String>, duration: i32) -> Self {
        Self {
            code: String::from(code),
            identifier,
            duration,
        }
    }
}

#[async_trait]
impl CredentialsProvider for EnvCredentialsProvider {
    /// Validate and return current [`Credentials`] from environment variables unless expired
    async fn validate(&self) -> Result<Option<Credentials>, Error> {
        let provider = get_env_provider();
        if let Some(credentials) = get_env_credentials(provider).await? {
            if credentials.expired() {
                return Err(Other(anyhow!("expired credentials")));
            }

            return Ok(Some(credentials));
        }

        Ok(None)
    }

    /// Authenticate using [`aws_config::environment::EnvironmentVariableCredentialsProvider`]) and return new [`Credentials`]
    async fn authenticate(&self) -> Result<Credentials, Error> {
        let config = get_env_config().await;
        let provider = get_env_provider();
        let client = get_client(&config, provider);
        let arn = get_mfa_device_arn(&client, self.identifier.clone()).await?;
        let credentials = get_auth_credentials(&client, &arn, &self.code, self.duration).await?;

        Ok(credentials)
    }
}
