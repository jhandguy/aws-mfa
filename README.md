# aws-mfa

[![CI](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml/badge.svg)](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml)
[![CD](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml/badge.svg)](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml)

Authenticate to AWS with MFA 🔐

```shell
➜ aws-mfa -h
aws-mfa
Authenticate to AWS with MFA 🔐

USAGE:
    aws-mfa [OPTIONS] --code <CODE> <HOME>

ARGS:
    <HOME>    Home directory containing the AWS hidden folder [env: HOME=/Users/JohnDoe]

OPTIONS:
    -c, --code <CODE>            MFA code
    -d, --duration <DURATION>    Session duration in seconds [default: 3600]
    -h, --help                   Print help information
    -p, --profile <PROFILE>      Name of the AWS profile [default: default]
    -r, --region <REGION>        Name of the AWS region [default: eu-west-1]
    -s, --suffix <SUFFIX>        Suffix of the original AWS profile [default: noauth]
```

## Installation

**aws-mfa** is published on [crates.io](https://crates.io/crates/aws-mfa) and can be installed with

```shell
cargo install aws-mfa
```

or downloaded as binary from the [releases page](https://github.com/jhandguy/aws-mfa/releases).

## Usage

Add basic credentials in `~/.aws/credentials`:

```text
[<profile_name>-noauth]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
```

> **Note**: make sure to add the `-noauth` suffix to the profile name

Run `aws-mfa`:
```shell
aws-mfa -p <profile_name> -c <mfa_code>
```

Check generated credentials in `~/.aws/credentials`:
```text
[<profile_name>]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
aws_session_token = <aws_session_token>
```
