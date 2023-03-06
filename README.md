# aws-mfa

[![Version](https://img.shields.io/crates/v/aws-mfa)](https://crates.io/crates/aws-mfa)
[![Downloads](https://img.shields.io/crates/d/aws-mfa)](https://crates.io/crates/aws-mfa)
[![License](https://img.shields.io/crates/l/aws-mfa)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/jhandguy/aws-mfa/ci.yaml)](https://github.com/jhandguy/aws-mfa/actions/workflows/ci.yaml)
[![Release](https://img.shields.io/github/actions/workflow/status/jhandguy/aws-mfa/cd.yaml?label=release)](https://github.com/jhandguy/aws-mfa/actions/workflows/cd.yaml)

Authenticate to AWS with MFA 🔐

```shell
➜ aws-mfa -h
Authenticate to AWS with MFA 🔐

Usage: aws-mfa [OPTIONS] --code <CODE> <HOME>

Arguments:
  <HOME>  Home directory containing the AWS hidden folder [env: HOME=/Users/JohnDoe]

Options:
  -r, --region <REGION>      Name of the AWS region [default: eu-west-1]
  -p, --profile <PROFILE>    Name of the AWS profile [default: default]
  -s, --suffix <SUFFIX>      Suffix of the original AWS profile [default: noauth]
  -c, --code <CODE>          MFA code
  -d, --duration <DURATION>  Session duration in seconds [default: 3600]
  -h, --help                 Print help
  -V, --version              Print version
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
