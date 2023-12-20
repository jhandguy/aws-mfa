# aws-mfa

[![Version](https://img.shields.io/crates/v/aws-mfa)](https://crates.io/crates/aws-mfa)
[![Downloads](https://img.shields.io/crates/d/aws-mfa)](https://crates.io/crates/aws-mfa)
[![License](https://img.shields.io/crates/l/aws-mfa)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/jhandguy/aws-mfa/ci.yaml)](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml)
[![Release](https://img.shields.io/github/actions/workflow/status/jhandguy/aws-mfa/cd.yaml?label=release)](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml)

Authenticate to AWS with MFA 🔐

```shell
➜ aws-mfa
Authenticate to AWS with MFA 🔐

Usage: aws-mfa <COMMAND>

Commands:
  file  Authenticate to AWS with MFA using config and credentials files
  env   Authenticate to AWS with MFA using environment variables
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

**aws-mfa** is published on [crates.io](https://crates.io/crates/aws-mfa) and can be installed with

```shell
cargo install aws-mfa
```

or via [homebrew-tap](https://github.com/jhandguy/homebrew-tap) with

```shell
brew install jhandguy/tap/aws-mfa
```

or downloaded as binary from the [releases page](https://github.com/jhandguy/aws-mfa/releases).

## Usage

### Config and credentials files

Add default region in `~/.aws/config`:
```text
[profile <profile_name>-noauth]
region = <aws_region>

[profile <profile_name>]
region = <aws_region>
```

Add basic credentials in `~/.aws/credentials`:

```text
[<profile_name>-noauth]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
```

> **Note**: make sure to add the `-noauth` suffix to the profile name

Run the `aws-mfa file` command:
```shell
aws-mfa file -p <profile_name> -c <mfa_code>
```

Check generated credentials in `~/.aws/credentials`:
```shell
cat ~/.aws/credentials
```
```text
[<profile_name>]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
aws_session_token = <aws_session_token>
aws_session_expiration_timestamp = <aws_session_expiration_timestamp>
```

### Environment variables

Export default region and basic credentials as environment variables:

```shell
export AWS_REGION=<aws_region>
export AWS_ACCESS_KEY_ID=<aws_access_key_id>
export AWS_SECRET_ACCESS_KEY=<aws_secret_access_key>
```

Eval the `aws-mfa env` command:
```shell
eval $(aws-mfa env -c <mfa_code>)
```

Check exported environment variables:
```shell
env | grep AWS_
```
```text
AWS_REGION=<aws_region>
AWS_ACCESS_KEY_ID=<aws_access_key_id>
AWS_SECRET_ACCESS_KEY=<aws_secret_access_key>
AWS_SESSION_TOKEN=<aws_session_token>
AWS_SESSION_EXPIRATION_TIMESTAMP=<aws_session_expiration_timestamp>
```
