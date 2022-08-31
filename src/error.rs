use std::fmt::Debug;
use std::io::Error as IOError;
use thiserror::Error;

use aws_sdk_sts::error::{GetCallerIdentityError, GetSessionTokenError};
use aws_sdk_sts::types::SdkError;

#[derive(Error, Debug)]
pub enum AwsAuthError {
    #[error("Failed to perform I/O operation")]
    IO(#[from] IOError),
    #[error("Failed to get caller identity")]
    CallerIdentity(#[from] SdkError<GetCallerIdentityError>),
    #[error("Failed to get session token")]
    SessionToken(#[from] SdkError<GetSessionTokenError>),
}
