use crate::error::Error;
use crate::error::Error::{
    GetEnvVariableError, InvalidCredentials, ParseSessionTimestampError, ProvideCredentialsError,
};
use crate::Credentials;
use aws_config::environment::EnvironmentVariableCredentialsProvider;
use aws_credential_types::provider::ProvideCredentials;
use std::env::{var, VarError};

const AWS_SESSION_EXPIRATION_TIMESTAMP: &str = "AWS_SESSION_EXPIRATION_TIMESTAMP";

pub async fn get_env_credentials(
    provider: EnvironmentVariableCredentialsProvider,
) -> Result<Option<Credentials>, Error> {
    return match var(AWS_SESSION_EXPIRATION_TIMESTAMP) {
        Ok(var) => match var.parse::<i64>() {
            Ok(session_expiration_timestamp) => {
                let credentials = provider
                    .provide_credentials()
                    .await
                    .map_err(ProvideCredentialsError)?;
                let session_token = credentials
                    .session_token()
                    .ok_or_else(|| InvalidCredentials(String::from("session_token")))?;

                Ok(Some(Credentials::new(
                    credentials.access_key_id(),
                    credentials.secret_access_key(),
                    session_token,
                    session_expiration_timestamp,
                )))
            }
            Err(e) => Err(ParseSessionTimestampError(e)),
        },
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => Err(GetEnvVariableError {
            var: String::from(AWS_SESSION_EXPIRATION_TIMESTAMP),
            source: e,
        }),
    };
}
