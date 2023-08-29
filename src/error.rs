use aws_credential_types::provider::error::CredentialsError;
use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityError;
use aws_sdk_sts::operation::get_session_token::GetSessionTokenError;
use aws_smithy_http::body::SdkBody;
use aws_smithy_http::result::SdkError;
use http::Response;
use std::env::VarError;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;
use time::error::ComponentRange;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to get caller identity")]
    GetCallerIdentityError(#[source] SdkError<GetCallerIdentityError, Response<SdkBody>>),

    #[error("failed to get session token")]
    GetSessionTokenError(#[source] SdkError<GetSessionTokenError, Response<SdkBody>>),

    #[error("failed to provide credentials")]
    ProvideCredentialsError(#[source] CredentialsError),

    #[error("could not read credentials file `{path:?}`")]
    ReadCredentialsFileError { path: String, source: io::Error },

    #[error("could not write in credentials file `{path:?}`")]
    WriteCredentialsFileError { path: String, source: io::Error },

    #[error("failed to get environment variable `{var:?}`")]
    GetEnvVariableError { var: String, source: VarError },

    #[error("missing field `{0}` in session token")]
    InvalidSession(String),

    #[error("missing field `{0}` in session credentials")]
    InvalidCredentials(String),

    #[error("missing field `{0}` in caller identity")]
    InvalidIdentity(String),

    #[error("failed to parse session timestamp")]
    ParseSessionTimestampError(#[source] ParseIntError),

    #[error("failed to convert session timestamp to datetime")]
    ConvertSessionTimestampError(#[source] ComponentRange),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
