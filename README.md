# awscli-mfa

[![Version](https://img.shields.io/crates/v/awsmfa)](https://crates.io/crates/awsmfa)
[![License](https://img.shields.io/crates/l/awsmfa)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/kaicoh/awscli-mfa/build.yml)](https://github.com/kaicoh/awscli-mfa/actions/workflows/build.yml)
[![Release](https://img.shields.io/github/actions/workflow/status/kaicoh/awscli-mfa/release.yml)](https://github.com/kaicoh/awscli-mfa/actions/workflows/release.yml)

This tool automates Multi-Factor Authentication (MFA) process in using awscli. It generates one time password, gets session token of AWS STS and updates AWS Credential file to use awscli with authenticated credentials.

## Installation

You can install **awsmfa** via `cargo install` command.

```
$ cargo install awsmfa
```

Or download the binary file from the [release page](https://github.com/kaicoh/awscli-mfa/releases).

## Usage

### 1. Configure config file

First, create config file `mfa_config.yml` in `~/.aws` directory like this.

```
devices:
  - profile: default
    arn: arn:aws:iam::123456788990:mfa/kaicoh
    secret: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01
  - profile: alpha
    arn: arn:aws:iam::123456788990:mfa/alpha
    secret: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98
```

You can get the secret for each MFA device from the following screen in the registration process of the MFA device. If you want MFA code in registration process, you can get it via [otp subcommand](#otp).

![How to get secret](https://github.com/kaicoh/awscli-mfa/blob/images/assets/How_to_get_secret.png)

### 2. Save new credentials

Execute the binary and the new credentials are saved as `[profile]-mfa` in AWS Credentials (`~/.aws/credentials`).

```
$ awsmfa --profile alpha --duration 43200
Saved credentials successfully as profile "alpha-mfa".
The new credentials is valid until 2023-01-31 09:00:00.
```

#### Options

| long | short | requried | description
----|---- |---- |----
| profile | p | no | The profile name to execute aws-sts `get-session-token` action. When not provided the `default` is used.
| duration | d | no | The duration seconds the generated credentials persists.

### 3. Execute any aws cli commands with profile option

```
$ aws s3 ls --profile alpha-mfa
...
```

## Subcommands

### otp

Once you configure the [config file](#usage), you can generate MFA code(= one time password) from this command.

```
$ awsmfa otp --profile alpha --clip
123456
```

#### Options

| long | short | requried | description
----|---- |---- |----
| profile | p | no | The profile name in the config file. When not provided the `default` is used.
| clip | c | no | Whether copying the generated MFA code to the clipboad or not.

---

### ls

Show the current configuration from the config file.

```
$ awsmfa ls
[profile default]
arn	: arn:aws:iam::123456788990:mfa/kaicoh
secret	: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01

[profile alpha]
arn	: arn:aws:iam::123456788990:mfa/alpha
secret	: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98
```

---

### set

Set the MFA device to the config file.

```
$ awsmfa set \
> --profile beta \
> --arn arn:aws:iam::123456788990:mfa/beta \
> --secret 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR
Saved MFA device for profile "beta" successfully.

$ awsmfa ls
[profile default]
arn	: arn:aws:iam::123456788990:mfa/kaicoh
secret	: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ01

[profile alpha]
arn	: arn:aws:iam::123456788990:mfa/alpha
secret	: ZYXWVUTSRQPONMLKJIHGFECDBA9876543210ZYXWVUTSRQPONMLKJIHGFECDBA98

[profile beta]
arn	: arn:aws:iam::123456788990:mfa/beta
secret	: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR
```

#### Options

| long | short | requried | description
----|---- |---- |----
| profile | p | yes | The profile name in the config file.
| arn | a | yes | The ARN for the MFA device.
| secret | s | yes | The secret key for the MFA device.

---

### rm

Remove the MFA device from the config file.

```
$ awsmfa rm --profile beta
Removed the MFA device for profile "beta" successfully.
```

#### Options

| long | short | requried | description
----|---- |---- |----
| profile | p | yes | The profile name in the config file.

## License

This software is released under the [MIT License](LICENSE).
