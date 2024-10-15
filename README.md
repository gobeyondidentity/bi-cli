# Secure Access CLI

$${\color{red}\textbf{WARNING: This tool is in alpha. Expect breaking changes.}}$$

Secure Access CLI is a command-line interface designed to automate the setup of a Secure Access Tenant with Single Sign-On (SSO) capabilities. This guide will walk you through the installation process, how to configure your environment, and detailed usage instructions for each available command.

## Table of Contents

1. [Installation](#installation)
   - [macOS and Linux](#macos-and-linux)
   - [Windows](#windows)
2. [Usage](#usage)
   - [Commands](#commands)
     - [Beyond Identity Commands](#beyond-identity-commands)
       - [login](#login)
       - [create-scim-app](#create-scim-app)
       - [create-external-sso-connection](#create-external-sso-connection)
       - [get-token](#get-token)
       - [send-enrollment-email](#send-enrollment-email)
       - [delete-all-sso-configs](#delete-all-sso-configs)
       - [review-unenrolled](#review-unenrolled)
     - [Okta Commands](#okta-commands)
       - [setup](#setup-okta)
       - [create-scim-app](#create-scim-app-okta)
       - [create-custom-attribute](#create-custom-attribute)
       - [create-identity-provider](#create-identity-provider)
       - [create-routing-rule](#create-routing-rule)
       - [fast-migrate](#okta-fast-migrate)
     - [OneLogin Commands](#onelogin-commands)
       - [setup](#setup-onelogin)
       - [fast-migrate](#onelogin-fast-migrate)
   - [Options](#options)
3. [Examples](#examples)
4. [Additional Information](#additional-information)

## Installation

### macOS and Linux

You can install bi using the provided install.sh script.

#### Using cURL

Open your terminal and run:

```bash
curl -fsSL https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

#### Using wget

If you prefer wget, run:

```bash
wget -qO- https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

### Windows

For Windows users, follow these steps:

1. Download the executable:

- Go to the Releases page.
- Download bi-vx.x.x-win-amd64.exe.

2. Add to PATH (Optional but Recommended):

- Move the downloaded .exe file to a preferred directory, e.g., C:\Program Files\bi.
- Add this directory to your system's PATH:
  - Press Win + X and select System.
  - Click on Advanced system settings.
  - Click Environment Variables.
  - Under System variables, scroll to Path and click Edit.
  - Click New and add the path to your bi executable.
  - Click OK to close all dialogs.

3. Run bi:

- Open Command Prompt or PowerShell.
- Type bi to verify the installation.

## Usage

To run the CLI tool, use the following syntax:

```sh
bi [OPTIONS] <COMMAND> <SUBCOMMAND> [ARGS]
```

### Commands

#### Beyond Identity Commands

To access Beyond Identity specific commands, use:

```sh
bi api <SUBCOMMAND>
```

##### login

Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied.

```sh
bi api login
```

##### create-scim-app

Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider.

```sh
bi api create-scim-app
```

##### create-external-sso-connection

Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity.

```sh
bi api create-external-sso-connection
```

##### get-token

Gets a bearer token for use with API calls.

```sh
bi api get-token
```

##### send-enrollment-email

Helps you send enrollment emails to one or more (or all) users in Beyond Identity.

```sh
bi api send-enrollment-email
```

##### delete-all-sso-configs

Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch.

```sh
bi api delete-all-sso-configs
```

##### review-unenrolled

Reviews which identities have not completed the enrollment process. An unenrolled identity is defined as one without a passkey for the given tenant/realm configuration.

```sh
bi api review-unenrolled
```

#### Okta Commands

To access Okta specific commands, use:

```sh
bi okta <SUBCOMMAND>
```

##### setup-okta

Setup allows you to provision an Okta tenant to be used for subsequent commands.

```sh
bi okta setup
```

##### create-scim-app

Creates a SCIM app in Okta that is connected to the SCIM app created in the previous step. Note that this command will generate the app and assign all groups to the SCIM app. However, there is a manual step you have to complete on your own which unfortunately cannot be automated. When you run this command the first time, we'll provide you with a SCIM base URL and API token that you'll need to copy into the SCIM app in Okta. You will also have to enable provisioning of identities manually in Okta. The good news is that both of these steps are very easy to do. You can find the exact steps to follow [here](https://docs.beyondidentity.com/docs/directory/directory-integrations/okta#-finish-configuring-the-okta-scim-application).

```sh
bi okta create-scim-app
```

##### create-custom-attribute

Creates a custom attribute in Okta on the default user type that will be used to create an IDP routing rule in Okta. This is a boolean value that gets set to "true" whenever a passkey is bound for a specific user.

```sh
bi okta create-custom-attribute
```

##### create-identity-provider

Takes the external SSO connection you created in Beyond Identity and uses it to configure an identity provider in Okta. This is the identity provider that will be used to authenticate Okta users using Beyond Identity.

```sh
bi okta create-identity-provider
```

##### create-routing-rule

The final step when setting up Beyond Identity as an MFA in Okta. This will use the custom attribute you created using an earlier command to route users who have provisioned a Beyond Identity passkey to Beyond Identity during authentication.

```sh
bi okta create-routing-rule
```

##### okta-fast-migrate

Automatically populates Beyond Identity's SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta.

```sh
bi okta fast-migrate
```

#### OneLogin Commands

To access OneLogin specific commands, use:

```sh
bi onelogin <SUBCOMMAND>
```

##### setup-onelogin

Setup allows you to provision a Onelogin tenant to be used for subsequent commands.

```sh
bi onelogin setup
```

##### onelogin-fast-migrate

Automatically populates Beyond Identity's SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin.

```sh
bi onelogin fast-migrate
```

### Options

- `-l, --log-level <LOG_LEVEL>`: Set the log level (`error`, `warn`, `info`, `debug`, `trace`).
- `-h, --help`: Print help information.

## Additional Information

- **Permissions and API Keys**: Ensure that you have the necessary permissions and API keys for both Beyond Identity and Okta before running the commands.

- **Manual Steps**: Some commands require manual configuration steps that cannot be automated due to platform limitations. Instructions are provided within the command descriptions and linked documentation.

- **Documentation Links**:

  - [Okta SCIM Application Configuration](https://docs.beyondidentity.com/docs/directory/directory-integrations/okta#-finish-configuring-the-okta-scim-application)

- **Logging**: Use the `--log-level` option to control the verbosity of the CLI output. This can be helpful for debugging or monitoring the progress of operations.

- **Help Command**: For more detailed information about a command and its options, use the `--help` flag after any command or subcommand.

  ```sh
  bi api --help
  ```

  ```sh
  bi okta create-scim-app --help
  ```
