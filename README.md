# aws-mfa

[![CI](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml/badge.svg)](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml)
[![CD](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml/badge.svg)](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml)

Authenticate to AWS with MFA 🔐

## Installation

### macOS

#### 64-bit

```shell
curl -OL https://github.com/jhandguy/aws-mfa/releases/download/v0.1.0/x86_64-apple-darwin.gz && tar xzvf x86_64-apple-darwin.gz
```

#### ARM64

```shell
curl -OL https://github.com/jhandguy/aws-mfa/releases/download/v0.1.0/aarch64-apple-darwin.gz && tar xzvf aarch64-apple-darwin.gz
```

### Linux

#### 32-bit

```shell
curl -OL https://github.com/jhandguy/aws-mfa/releases/download/v0.1.0/i686-unknown-linux-gnu.gz && tar xzvf i686-unknown-linux-gnu.gz
```

#### 64-bit

```shell
curl -OL https://github.com/jhandguy/aws-mfa/releases/download/v0.1.0/x86_64-unknown-linux-gnu.gz && tar xzvf x86_64-unknown-linux-gnu.gz
```

#### ARM64

```shell
curl -OL https://github.com/jhandguy/aws-mfa/releases/download/v0.1.0/aarch64-unknown-linux-gnu.gz && tar xzvf aarch64-unknown-linux-gnu.gz
```

## Usage

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

### Create unauthorized credentials

Add in `~/.aws/credentials`:
```shell
[<profile_name>-noauth]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
```

### Generate authorized credentials

Run in terminal:
```shell
aws-mfa -p <profile_name> -c <mfa_code>
```

Output in `~/.aws/credentials`:
```shell
[<profile_name>]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
aws_session_token = <aws_session_token>
```
