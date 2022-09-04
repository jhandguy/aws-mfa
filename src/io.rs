use anyhow::Result;
use std::fs::{read_to_string, write};
use std::ops::Add;

pub fn save_auth_credential(home: &str, profile: &str, credential: &str) -> Result<()> {
    let file_path = format!("{}/.aws/credentials", home);
    let file_content = read_to_string(&file_path)?
        .split("\n\n")
        .filter(|l| !l.is_empty() && !l.contains(format!("[{}]", profile).as_str()))
        .collect::<Vec<&str>>()
        .join("\n\n")
        .add(credential);

    write(&file_path, file_content.as_bytes())?;

    Ok(())
}
