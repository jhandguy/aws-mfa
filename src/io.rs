use std::fs::{read_to_string, write};
use std::ops::Add;

use anyhow::Result;

use crate::config::get_credentials_file;
use crate::error::Error;
use crate::error::Error::{ReadCredentialsFileError, WriteCredentialsFileError};
use crate::Credentials;

const AWS_ACCESS_KEY_ID: &str = "aws_access_key_id";
const AWS_SECRET_ACCESS_KEY: &str = "aws_secret_access_key";
const AWS_SESSION_TOKEN: &str = "aws_session_token";
const AWS_SESSION_EXPIRATION_TIMESTAMP: &str = "aws_session_expiration_timestamp";

fn find_credential_value(credentials: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = ", key);
    credentials
        .split('\n')
        .find(|l| l.contains(&pattern))
        .map(|c| c.replace(&pattern, ""))
}

fn find_credentials(file_content: &str, profile: &str) -> Option<Credentials> {
    let credentials = file_content
        .split("\n\n")
        .find(|l| l.contains(format!("[{profile}]").as_str()))?;

    let access_key_id = find_credential_value(credentials, AWS_ACCESS_KEY_ID)?;
    let secret_access_key = find_credential_value(credentials, AWS_SECRET_ACCESS_KEY)?;
    let session_token = find_credential_value(credentials, AWS_SESSION_TOKEN)?;

    let session_expiration_timestamp =
        match find_credential_value(credentials, AWS_SESSION_EXPIRATION_TIMESTAMP) {
            Some(e) => match e.parse::<i64>() {
                Ok(e) => e,
                Err(_) => return None,
            },
            None => return None,
        };

    Some(Credentials::new(
        &access_key_id,
        &secret_access_key,
        &session_token,
        session_expiration_timestamp,
    ))
}

fn replace_credentials(file_content: &str, profile: &str, content: &str) -> String {
    file_content
        .split("\n\n")
        .filter(|l| !l.is_empty() && !l.contains(format!("[{profile}]").as_str()))
        .collect::<Vec<&str>>()
        .join("\n\n")
        .add(content)
}

pub fn find_auth_credentials(home: &str, profile: &str) -> Result<Option<Credentials>, Error> {
    let file_path = get_credentials_file(home);
    let file_content = read_to_string(file_path.clone()).map_err(|e| ReadCredentialsFileError {
        path: file_path,
        source: e,
    })?;
    let credentials = find_credentials(&file_content, profile);

    Ok(credentials)
}

pub fn save_auth_credentials(
    home: &str,
    profile: &str,
    credentials: &Credentials,
) -> Result<(), Error> {
    let content = format!(
        "

[{profile}]
{} = {}
{} = {}
{} = {}
{} = {}",
        AWS_ACCESS_KEY_ID,
        credentials.access_key_id(),
        AWS_SECRET_ACCESS_KEY,
        credentials.secret_access_key(),
        AWS_SESSION_TOKEN,
        credentials.session_token(),
        AWS_SESSION_EXPIRATION_TIMESTAMP,
        credentials.session_expiration_timestamp(),
    );

    let file_path = get_credentials_file(home);
    let file_content = read_to_string(&file_path).map_err(|e| ReadCredentialsFileError {
        path: file_path.clone(),
        source: e,
    })?;
    let new_content = replace_credentials(&file_content, profile, &content);

    write(file_path.clone(), new_content.as_bytes()).map_err(|e| WriteCredentialsFileError {
        path: file_path,
        source: e,
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::io::{
        find_credential_value, find_credentials, replace_credentials, AWS_ACCESS_KEY_ID,
        AWS_SECRET_ACCESS_KEY, AWS_SESSION_EXPIRATION_TIMESTAMP, AWS_SESSION_TOKEN,
    };

    #[tokio::test]
    async fn test_find_credential_value() -> Result<()> {
        let credentials = "

[profile-1]
aws_access_key_id = aws_access_key_id_1
aws_secret_access_key = aws_secret_access_key_1
aws_session_token = aws_session_token_1
aws_session_expiration_timestamp = 1688903647";

        assert_eq!(
            find_credential_value(credentials, AWS_ACCESS_KEY_ID),
            Some(String::from("aws_access_key_id_1"))
        );
        assert_eq!(
            find_credential_value(credentials, AWS_SECRET_ACCESS_KEY),
            Some(String::from("aws_secret_access_key_1"))
        );
        assert_eq!(
            find_credential_value(credentials, AWS_SESSION_TOKEN),
            Some(String::from("aws_session_token_1"))
        );
        assert_eq!(
            find_credential_value(credentials, AWS_SESSION_EXPIRATION_TIMESTAMP),
            Some(String::from("1688903647"))
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_find_credentials() -> Result<()> {
        let file_content = "
[profile-1]
aws_access_key_id = aws_access_key_id_1
aws_secret_access_key = aws_secret_access_key_1
aws_session_token = aws_session_token_1
aws_session_expiration_timestamp = 1688903647

[profile-2]
aws_access_key_id = aws_access_key_id_2
aws_secret_access_key = aws_secret_access_key_2
aws_session_token = aws_session_token_2
aws_session_expiration_timestamp = 1688905806";

        let credentials = find_credentials(file_content, "profile-1")
            .ok_or_else(|| anyhow!("credentials missing"))?;
        assert_eq!(credentials.access_key_id(), "aws_access_key_id_1");
        assert_eq!(credentials.secret_access_key(), "aws_secret_access_key_1");
        assert_eq!(credentials.session_token(), "aws_session_token_1");
        assert_eq!(credentials.session_expiration_timestamp(), 1688903647);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_credentials() -> Result<()> {
        let file_content = "
[profile-1]
aws_access_key_id = aws_access_key_id_1
aws_secret_access_key = aws_secret_access_key_1
aws_session_token = aws_session_token_1
aws_session_expiration_timestamp = 1688903647

[profile-2]
aws_access_key_id = aws_access_key_id_2
aws_secret_access_key = aws_secret_access_key_2
aws_session_token = aws_session_token_2
aws_session_expiration_timestamp = 1688905806";

        let new_credentials = "

[profile-2]
aws_access_key_id = aws_access_key_id_3
aws_secret_access_key = aws_secret_access_key_3
aws_session_token = aws_session_token_3
aws_session_expiration_timestamp = 1688905943";

        let expected_content = "
[profile-1]
aws_access_key_id = aws_access_key_id_1
aws_secret_access_key = aws_secret_access_key_1
aws_session_token = aws_session_token_1
aws_session_expiration_timestamp = 1688903647

[profile-2]
aws_access_key_id = aws_access_key_id_3
aws_secret_access_key = aws_secret_access_key_3
aws_session_token = aws_session_token_3
aws_session_expiration_timestamp = 1688905943";

        let new_content = replace_credentials(file_content, "profile-2", new_credentials);
        assert_eq!(new_content, expected_content);

        Ok(())
    }
}
