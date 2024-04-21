use aws_config::environment::EnvironmentVariableCredentialsProvider;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::{from_env, SdkConfig};
use aws_runtime::env_config::file::EnvConfigFileKind::{Config, Credentials};
use aws_runtime::env_config::file::EnvConfigFiles;
use aws_sdk_sts::config::Region;

fn get_config_file(home: &str) -> String {
    format!("{home}/.aws/config")
}

pub fn get_credentials_file(home: &str) -> String {
    format!("{home}/.aws/credentials")
}

pub async fn get_file_config(
    home: &str,
    region: Option<String>,
    profile: &str,
    suffix: &str,
) -> SdkConfig {
    let files = EnvConfigFiles::builder()
        .with_file(Config, get_config_file(home))
        .with_file(Credentials, get_credentials_file(home))
        .build();

    let mut config = from_env()
        .profile_files(files)
        .profile_name(format!("{profile}-{suffix}"));

    if let Some(region) = region {
        config = config.region(Region::new(region));
    }

    config.load().await
}

pub fn get_file_provider(profile: &str, suffix: &str) -> ProfileFileCredentialsProvider {
    ProfileFileCredentialsProvider::builder()
        .profile_name(format!("{profile}-{suffix}"))
        .build()
}

pub async fn get_env_config() -> SdkConfig {
    from_env().load().await
}

pub fn get_env_provider() -> EnvironmentVariableCredentialsProvider {
    EnvironmentVariableCredentialsProvider::new()
}
