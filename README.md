# Secure Access CLI

Secure Access CLI is a command-line interface designed to automate the setup of a Secure Access Tenant with Single Sign-On (SSO) capabilities. This guide will walk you through the installation process, how to configure your environment, and detailed usage instructions for each available command.

## Table of Contents

1. [Installation](#installation)
    - [Install Rust](#install-rust)
    - [Clone the Repository](#clone-the-repository)
    - [Build the Project](#build-the-project)
2. [Configuration](#configuration)
    - [Environment Variables](#environment-variables)
3. [Usage](#usage)
    - [Commands](#commands)
        - [Beyond Identity Commands](#beyond-identity-commands)
            - [create-tenant](#create-tenant)
            - [provision-existing-tenant](#provision-existing-tenant)
            - [create-scim-app](#create-scim-app)
            - [create-external-sso-connection](#create-external-sso-connection)
            - [get-token](#get-token)
            - [send-enrollment-email](#send-enrollment-email)
            - [delete-all-sso-configs](#delete-all-sso-configs)
            - [review-unenrolled](#review-unenrolled)
        - [Okta Commands](#okta-commands)
            - [create-scim-app](#create-scim-app-okta)
            - [create-custom-attribute](#create-custom-attribute)
            - [create-identity-provider](#create-identity-provider)
            - [create-routing-rule](#create-routing-rule)
            - [fast-migrate](#fast-migrate)
    - [Options](#options)
4. [Examples](#examples)
5. [Additional Information](#additional-information)

## Installation

### Install Rust

If you don't have Rust installed, you need to install it first. Follow these steps to install Rust:

1. **Download Rustup**: Rustup is an installer for the Rust programming language.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Follow the On-Screen Instructions**: The installer will guide you through the installation process. Once installed, configure your current shell session to use Rust by running:

    ```sh
    source $HOME/.cargo/env
    ```

3. **Verify Installation**: To ensure Rust is installed correctly, you can run:

    ```sh
    rustc --version
    ```

### Clone the Repository

Clone the project repository to your local machine:

```sh
git clone git@github.com:gobeyondidentity/secure-access-cli.git
cd secure-access-cli
```

### Build the Project

Build the project using Cargo (Rust's package manager):

```sh
cargo build --release
```

## Configuration

### Environment Variables

Secure Access CLI uses environment variables for configuration. Create a `.env` file in the root of the project directory and populate it with the necessary variables. Here is an example of the required variables:

```sh
OKTA_API_KEY="<OKTA_API_KEY_READ+WRITE>"
OKTA_DOMAIN="<YOUR_OKTA_DOMAIN>"
OKTA_REGISTRATION_SYNC_ATTRIBUTE="byndidRegistered" # You can update this to be any non-conflicting value if you need to
BEYOND_IDENTITY_API_BASE_URL="https://api-<eu|us>.beyondidentity.<run|xyz|com>"
BEYOND_IDENTITY_AUTH_BASE_URL="https://auth-<eu|us>.beyondidentity.<run|xyz|com>"
ADMIN_DISPLAY_NAME="<YOUR_NAME>"
ADMIN_PRIMARY_EMAIL_ADDRESS="<YOUR_EMAIL_ADDRESS>"
```

Make sure to replace the placeholders with your actual configuration values.

## Usage

To run the CLI tool, use the following syntax:

```sh
./target/release/secure-access-cli [OPTIONS] <COMMAND> <SUBCOMMAND> [ARGS]
```

### Commands

#### Beyond Identity Commands

To access Beyond Identity specific commands, use:

```sh
./target/release/secure-access-cli beyond-identity <SUBCOMMAND>
```

##### create-tenant

Creates a new Secure Access tenant. This command is required for all the remaining commands to work as it provides the base configuration. The first time you run this command, it will ask you to open a browser with a magic link to complete the provisioning process. Subsequent runs will show you the existing tenant configuration.

```sh
./target/release/secure-access-cli beyond-identity create-tenant
```

##### provision-existing-tenant

Provisions configuration for an existing tenant provided a tenant ID, realm ID, and API token are supplied.

```sh
./target/release/secure-access-cli beyond-identity provision-existing-tenant
```

##### create-scim-app

Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider.

```sh
./target/release/secure-access-cli beyond-identity create-scim-app
```

##### create-external-sso-connection

Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity.

```sh
./target/release/secure-access-cli beyond-identity create-external-sso-connection
```

##### get-token

Gets a bearer token for use with API calls.

```sh
./target/release/secure-access-cli beyond-identity get-token
```

##### send-enrollment-email

Helps you send enrollment emails to one or more (or all) users in Beyond Identity.

```sh
./target/release/secure-access-cli beyond-identity send-enrollment-email
```

##### delete-all-sso-configs

Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch.

```sh
./target/release/secure-access-cli beyond-identity delete-all-sso-configs
```

##### review-unenrolled

Reviews which identities have not completed the enrollment process. An unenrolled identity is defined as one without a passkey for the given tenant/realm configuration.

```sh
./target/release/secure-access-cli beyond-identity review-unenrolled
```

#### Okta Commands

To access Okta specific commands, use:

```sh
./target/release/secure-access-cli okta <SUBCOMMAND>
```

##### create-scim-app

Creates a SCIM app in Okta that is connected to the SCIM app created in the previous step. Note that this command will generate the app and assign all groups to the SCIM app. However, there is a manual step you have to complete on your own which unfortunately cannot be automated. When you run this command the first time, we'll provide you with a SCIM base URL and API token that you'll need to copy into the SCIM app in Okta. You will also have to enable provisioning of identities manually in Okta. The good news is that both of these steps are very easy to do. You can find the exact steps to follow [here](https://docs.beyondidentity.com/docs/directory/directory-integrations/okta#-finish-configuring-the-okta-scim-application).

```sh
./target/release/secure-access-cli okta create-scim-app
```

##### create-custom-attribute

Creates a custom attribute in Okta on the default user type that will be used to create an IDP routing rule in Okta. This is a boolean value that gets set to "true" whenever a passkey is bound for a specific user.

```sh
./target/release/secure-access-cli okta create-custom-attribute
```

##### create-identity-provider

Takes the external SSO connection you created in Beyond Identity and uses it to configure an identity provider in Okta. This is the identity provider that will be used to authenticate Okta users using Beyond Identity.

```sh
./target/release/secure-access-cli okta create-identity-provider
```

##### create-routing-rule

The final step when setting up Beyond Identity as an MFA in Okta. This will use the custom attribute you created using an earlier command to route users who have provisioned a Beyond Identity passkey to Beyond Identity during authentication.

```sh
./target/release/secure-access-cli okta create-routing-rule
```

##### fast-migrate

Automatically populates Beyond Identity's SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta.

```sh
./target/release/secure-access-cli okta fast-migrate
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
  ./target/release/secure-access-cli beyond-identity --help
  ```

  ```sh
  ./target/release/secure-access-cli okta create-scim-app --help
  ```
