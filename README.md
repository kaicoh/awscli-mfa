# awscli-mfa

[![Version](https://img.shields.io/crates/v/awsmfa)](https://crates.io/crates/awsmfa)
[![License](https://img.shields.io/crates/l/awsmfa)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/kaicoh/awscli-mfa/build.yml)](https://github.com/kaicoh/awscli-mfa/actions/workflows/build.yml)

The automation tool for Multi-Factor Authentication (MFA) process to use awscli. It generates one time password, gets session token of AWS STS and updates both AWS Credentials(`~/.aws/credentials`) and Config(`~/.aws/config`) files automatically and you can run any aws cli commands without [complecated process](https://aws.amazon.com/premiumsupport/knowledge-center/authenticate-mfa-cli/).

## Installation

You can install **awsmfa** via `cargo install` command.

```
$ cargo install awsmfa
```

Or download the binary file from the [release page](https://github.com/kaicoh/awscli-mfa/releases).

## Usage

### 1. Configure config files

First, make sure that the `mfa_serial` for the profile you want to use is defined in AWS Config(`~/.aws/config`). For more details about AWS Config and `mfa_serial`, see [the official document](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html).

```
[default]
region=us-east-1
output=json
mfa_serial=arn:aws:iam::123456789012:mfa/kaicoh
...

[profile alpha]
region=ap-northeast-1
output=json
mfa_serial=arn:aws:iam::999999999999:mfa/alpha
...
```

Then, create `awsmfa.yml` in `~/.aws` directory and set secret key for each MFA device.

```
secrets:
  - profile: default
    value: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01
  - profile: alpha
    value: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98
```

You can get the secret key for each MFA device during the registration process for that in AWS Consol. If you want some MFA codes in that process, run [otp subcommand](#otp).

![How to get secret](https://github.com/kaicoh/awscli-mfa/blob/images/assets/How_to_get_secret.png)

### 2. Run the command

```
$ awsmfa --profile alpha --duration 43200
New credentials is available as profile "alpha-mfa".
It is valid until 2023-01-31 09:00:00.
```

This command generates one time password, gets session token of AWS STS and updates both AWS Credentials(`~/.aws/credentials`) and Config(`~/.aws/config`) files internally. After that, the new credentials and configurations are saved as `[profile]-mfa`.

**~/.aws/credentials**

```
$ cat ~/.aws/credentials
...

[alpha-mfa] <= The command creates this.
aws_access_key_id=ZZZZZZZZZZZZZZZZZZZZ
aws_secret_access_key=zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
aws_session_token=aaaaaaaaa.....
```

**~/.aws/config**

```
$ cat ~/.aws/config
...

[profile alpha-mfa] <= The command creates this.
region=ap-northeast-1
output=json
```

#### Options

| name | short | requried | type | description |
| :---: | :---: | :---: | :---: | :--- |
| profile | p | no | string | The profile name to execute aws-sts `get-session-token` action. When not provided the `default` is used. |
| duration | d | no | number | The duration seconds the generated credentials persists. |

### 3. Run any aws cli commands with profile option

```
$ aws s3 ls --profile alpha-mfa
...
```

## Subcommands

This binary has some subcommands to configure or operate MFA related processes.

### otp

Once you [configure](#1-configure-config-files), you can generate MFA code(= one time password) from this command.

```
$ awsmfa otp --profile alpha --clip
123456
```

#### Options

| name | short | requried | type | description |
| :---: | :---: | :---: | :---: | :--- |
| profile | p | no | string | The profile name in the config file. When not provided the `default` is used. |
| clip | c | no | bool | Whether copying the generated MFA code to the clipboad or not. |

---

### ls

Show the current configuration from the `~/.aws/awsmfa.yml`.

```
$ awsmfa ls
[profile default]
secret	: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01

[profile alpha]
secret	: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98
```

---

### set

Set the MFA device to the `~/.aws/awsmfa.yml`.

```
$ awsmfa set \
> --profile beta \
> --secret 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR
Save the secret key for profile "beta" successfully.

$ awsmfa ls
[profile default]
secret	: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01

[profile alpha]
secret	: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98

[profile beta]
secret	: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR
```

#### Options

| name | short | requried | type | description |
| :---: | :---: | :---: | :---: | :--- |
| profile | p | yes | string | The profile name in the config file. |
| secret | s | yes | string | The secret key for the MFA device. |

---

### rm

Remove the MFA device from the `~/.aws/awsmfa.yml`.

```
$ awsmfa rm --profile beta
Remove the secret key for profile "beta" successfully.
```

#### Options

| name | short | requried | type | description |
| :---: | :---: | :---: | :---: | :--- |
| profile | p | yes | string | The profile name in the config file. |

## License

This software is released under the [MIT License](LICENSE).
