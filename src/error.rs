use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as IOError;

use aws_sdk_sts::error::{GetCallerIdentityError, GetSessionTokenError};
use aws_sdk_sts::types::SdkError;

pub struct AwsAuthError {
    message: String,
}

impl Error for AwsAuthError {}

impl Display for AwsAuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for AwsAuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<IOError> for AwsAuthError {
    fn from(e: IOError) -> Self {
        Self {
            message: format!("Failed to perform I/O operation: {}", e),
        }
    }
}

impl From<SdkError<GetCallerIdentityError>> for AwsAuthError {
    fn from(e: SdkError<GetCallerIdentityError>) -> Self {
        Self {
            message: format!("Failed to get caller identity: {}", e),
        }
    }
}

impl From<SdkError<GetSessionTokenError>> for AwsAuthError {
    fn from(e: SdkError<GetSessionTokenError>) -> Self {
        Self {
            message: format!("Failed to get session token: {}", e),
        }
    }
}
