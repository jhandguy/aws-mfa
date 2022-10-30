use anyhow::Result;

use crate::io::save_auth_credentials;
use crate::sts::{get_auth_credentials, get_client, get_mfa_device_arn};

/// Credentials received after authenticating to AWS with MFA via [authenticate](fn.authenticate.html).
pub struct Credentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

impl Credentials {
    pub fn new(access_key_id: &str, secret_access_key: &str, session_token: &str) -> Self {
        Self {
            access_key_id: String::from(access_key_id),
            secret_access_key: String::from(secret_access_key),
            session_token: String::from(session_token),
        }
    }

    pub fn access_key_id(&self) -> &str {
        self.access_key_id.as_ref()
    }

    pub fn secret_access_key(&self) -> &str {
        self.secret_access_key.as_ref()
    }

    pub fn session_token(&self) -> &str {
        self.session_token.as_ref()
    }
}

/// Authenticate to AWS with MFA
pub async fn authenticate(
    profile: &str,
    suffix: &str,
    region: &str,
    code: &str,
    duration: i32,
    home: &str,
) -> Result<Credentials> {
    let client = get_client(profile, suffix, region).await;
    let arn = get_mfa_device_arn(&client).await?;
    let credentials = get_auth_credentials(&client, &arn, code, duration).await?;
    save_auth_credentials(home, profile, &credentials)?;

    Ok(credentials)
}
