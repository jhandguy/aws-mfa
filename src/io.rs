use std::fs::{read_to_string, write};
use std::ops::Add;

use anyhow::Result;

use crate::auth::Credentials;

pub fn save_auth_credentials(home: &str, profile: &str, credentials: &Credentials) -> Result<()> {
    let content = format!(
        "

[{}]
aws_access_key_id = {}
aws_secret_access_key = {}
aws_session_token = {}",
        profile,
        credentials.access_key_id(),
        credentials.secret_access_key(),
        credentials.session_token()
    );

    let file_path = format!("{}/.aws/credentials", home);
    let file_content = read_to_string(&file_path)?
        .split("\n\n")
        .filter(|l| !l.is_empty() && !l.contains(format!("[{}]", profile).as_str()))
        .collect::<Vec<&str>>()
        .join("\n\n")
        .add(&content);

    write(&file_path, file_content.as_bytes())?;

    Ok(())
}
