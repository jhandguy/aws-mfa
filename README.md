# aws-auth

Authenticate to AWS with MFA 🔐

## Installation

> Coming soon!

## Usage

```shell
➜ aws-auth -h

aws-auth
Authenticate to AWS with MFA 🔐

USAGE:
    aws-auth [OPTIONS] --code <CODE> <HOME>

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
aws-auth -p <profile_name> -c <mfa_code>
```

Output in `~/.aws/credentials`:
```shell
[<profile_name>]
aws_access_key_id = <aws_access_key_id>
aws_secret_access_key = <aws_secret_access_key>
aws_session_token = <aws_session_token>
```
