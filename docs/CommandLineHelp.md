# bi

This document contains the help content for the `bi` command-line program.

**Command Overview:**

* [`bi`↴](#bi)
* [`bi config`↴](#bi-config)
* [`bi config tenants`↴](#bi-config-tenants)
* [`bi config tenants add`↴](#bi-config-tenants-add)
* [`bi config tenants list`↴](#bi-config-tenants-list)
* [`bi config tenants default`↴](#bi-config-tenants-default)
* [`bi config tenants default set`↴](#bi-config-tenants-default-set)
* [`bi config tenants default get`↴](#bi-config-tenants-default-get)
* [`bi config tenants remove`↴](#bi-config-tenants-remove)
* [`bi config ai`↴](#bi-config-ai)
* [`bi config ai provider`↴](#bi-config-ai-provider)
* [`bi config ai provider set`↴](#bi-config-ai-provider-set)
* [`bi config ai provider get`↴](#bi-config-ai-provider-get)
* [`bi config ai default`↴](#bi-config-ai-default)
* [`bi config ai default set`↴](#bi-config-ai-default-set)
* [`bi config ai default get`↴](#bi-config-ai-default-get)
* [`bi config okta`↴](#bi-config-okta)
* [`bi config okta set`↴](#bi-config-okta-set)
* [`bi config okta get`↴](#bi-config-okta-get)
* [`bi config onelogin`↴](#bi-config-onelogin)
* [`bi config onelogin set`↴](#bi-config-onelogin-set)
* [`bi config onelogin get`↴](#bi-config-onelogin-get)
* [`bi api`↴](#bi-api)
* [`bi api tenants`↴](#bi-api-tenants)
* [`bi api tenants get`↴](#bi-api-tenants-get)
* [`bi api tenants patch`↴](#bi-api-tenants-patch)
* [`bi api realms`↴](#bi-api-realms)
* [`bi api realms create`↴](#bi-api-realms-create)
* [`bi api realms list`↴](#bi-api-realms-list)
* [`bi api realms get`↴](#bi-api-realms-get)
* [`bi api realms patch`↴](#bi-api-realms-patch)
* [`bi api realms delete`↴](#bi-api-realms-delete)
* [`bi api groups`↴](#bi-api-groups)
* [`bi api groups create`↴](#bi-api-groups-create)
* [`bi api groups list`↴](#bi-api-groups-list)
* [`bi api groups get`↴](#bi-api-groups-get)
* [`bi api groups patch`↴](#bi-api-groups-patch)
* [`bi api groups delete`↴](#bi-api-groups-delete)
* [`bi api groups add-members`↴](#bi-api-groups-add-members)
* [`bi api groups delete-members`↴](#bi-api-groups-delete-members)
* [`bi api groups list-members`↴](#bi-api-groups-list-members)
* [`bi api groups list-roles`↴](#bi-api-groups-list-roles)
* [`bi api identities`↴](#bi-api-identities)
* [`bi api identities create`↴](#bi-api-identities-create)
* [`bi api identities list`↴](#bi-api-identities-list)
* [`bi api identities get`↴](#bi-api-identities-get)
* [`bi api identities patch`↴](#bi-api-identities-patch)
* [`bi api identities delete`↴](#bi-api-identities-delete)
* [`bi api identities list-groups`↴](#bi-api-identities-list-groups)
* [`bi api identities list-roles`↴](#bi-api-identities-list-roles)
* [`bi api credentials`↴](#bi-api-credentials)
* [`bi api credentials list`↴](#bi-api-credentials-list)
* [`bi api credentials get`↴](#bi-api-credentials-get)
* [`bi api credentials revoke`↴](#bi-api-credentials-revoke)
* [`bi helper`↴](#bi-helper)
* [`bi helper create-admin-account`↴](#bi-helper-create-admin-account)
* [`bi helper delete-all-identities`↴](#bi-helper-delete-all-identities)
* [`bi helper send-enrollment-email`↴](#bi-helper-send-enrollment-email)
* [`bi helper review-unenrolled`↴](#bi-helper-review-unenrolled)
* [`bi ai`↴](#bi-ai)
* [`bi ai ask`↴](#bi-ai-ask)
* [`bi okta`↴](#bi-okta)
* [`bi okta fast-migrate`↴](#bi-okta-fast-migrate)
* [`bi onelogin`↴](#bi-onelogin)
* [`bi onelogin fast-migrate`↴](#bi-onelogin-fast-migrate)

## `bi`

Official Beyond Identity command-line interface.

**Usage:** `bi [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `config` — Manage CLI tool configuration settings
* `api` — Interact with Beyond Identity API endpoints
* `helper` — Access helper functions for Beyond Identity API operations
* `ai` — Helper tool to generate example commands for CLI operations
* `okta` — Commands solely for fast migration off of Okta
* `onelogin` — Commands solely for fast migration off of OneLogin

###### **Options:**

* `-l`, `--log-level <LOG_LEVEL>`



## `bi config`

Manage CLI tool configuration settings

**Usage:** `bi config <COMMAND>`

###### **Subcommands:**

* `tenants` — Configure a tenant using an API token to interact with the Beyond Identity API
* `ai` — Commands for configuring the AI helper tool
* `okta` — Configure Okta settings to enable the CLI tool to interact with Okta APIs
* `onelogin` — Configure Onelogin settings to enable the CLI tool to interact with Onelogin APIs



## `bi config tenants`

Configure a tenant using an API token to interact with the Beyond Identity API

**Usage:** `bi config tenants <COMMAND>`

###### **Subcommands:**

* `add` — Provision an existing tenant using the provided API token
* `list` — Display a list of all configured tenants
* `default` — Configure and view the default tenant/realm
* `remove` — Remove a tenant from the configured list



## `bi config tenants add`

Provision an existing tenant using the provided API token

**Usage:** `bi config tenants add --token <TOKEN>`

###### **Options:**

* `--token <TOKEN>` — The API token associated with the tenant/realm you would like to add



## `bi config tenants list`

Display a list of all configured tenants

**Usage:** `bi config tenants list`



## `bi config tenants default`

Configure and view the default tenant/realm

**Usage:** `bi config tenants default <COMMAND>`

###### **Subcommands:**

* `set` — Set the default tenant/realm
* `get` — Get the default tenant/realm



## `bi config tenants default set`

Set the default tenant/realm

**Usage:** `bi config tenants default set`



## `bi config tenants default get`

Get the default tenant/realm

**Usage:** `bi config tenants default get`



## `bi config tenants remove`

Remove a tenant from the configured list

**Usage:** `bi config tenants remove`



## `bi config ai`

Commands for configuring the AI helper tool

**Usage:** `bi config ai <COMMAND>`

###### **Subcommands:**

* `provider` — Configure settings related to an AI provider
* `default` — Configure and view the default AI provider



## `bi config ai provider`

Configure settings related to an AI provider

**Usage:** `bi config ai provider <COMMAND>`

###### **Subcommands:**

* `set` — Set AI provider configuration
* `get` — Get AI provider configuration



## `bi config ai provider set`

Set AI provider configuration

**Usage:** `bi config ai provider set [OPTIONS] --provider <PROVIDER> --api-key <API_KEY>`

###### **Options:**

* `--provider <PROVIDER>` — AI Provider

  Possible values: `openai`, `anthropic`

* `--api-key <API_KEY>` — API Key
* `--force` — Flag to allow force reconfiguration



## `bi config ai provider get`

Get AI provider configuration

**Usage:** `bi config ai provider get --provider <PROVIDER>`

###### **Options:**

* `--provider <PROVIDER>` — AI Provider

  Possible values: `openai`, `anthropic`




## `bi config ai default`

Configure and view the default AI provider

**Usage:** `bi config ai default <COMMAND>`

###### **Subcommands:**

* `set` — Configure the default AI provider
* `get` — Display the current default AI provider



## `bi config ai default set`

Configure the default AI provider

**Usage:** `bi config ai default set --provider <PROVIDER>`

###### **Options:**

* `--provider <PROVIDER>`

  Possible values: `openai`, `anthropic`




## `bi config ai default get`

Display the current default AI provider

**Usage:** `bi config ai default get`



## `bi config okta`

Configure Okta settings to enable the CLI tool to interact with Okta APIs

**Usage:** `bi config okta <COMMAND>`

###### **Subcommands:**

* `set` — Configure Okta integration settings
* `get` — Display current Okta integration settings



## `bi config okta set`

Configure Okta integration settings

**Usage:** `bi config okta set [OPTIONS] --domain <DOMAIN> --api-key <API_KEY>`

###### **Options:**

* `--domain <DOMAIN>` — Okta domain
* `--api-key <API_KEY>` — Okta API key
* `--force` — Flag to allow force reconfiguration



## `bi config okta get`

Display current Okta integration settings

**Usage:** `bi config okta get`



## `bi config onelogin`

Configure Onelogin settings to enable the CLI tool to interact with Onelogin APIs

**Usage:** `bi config onelogin <COMMAND>`

###### **Subcommands:**

* `set` — Configure OneLogin integration settings
* `get` — Display current OneLogin integration settings



## `bi config onelogin set`

Configure OneLogin integration settings

**Usage:** `bi config onelogin set [OPTIONS] --domain <DOMAIN> --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET>`

###### **Options:**

* `--domain <DOMAIN>` — Onelogin domain
* `--client-id <CLIENT_ID>` — Onelogin client id
* `--client-secret <CLIENT_SECRET>` — Onelogin client secret
* `--force` — Flag to allow force reconfiguration



## `bi config onelogin get`

Display current OneLogin integration settings

**Usage:** `bi config onelogin get`



## `bi api`

Interact with Beyond Identity API endpoints

**Usage:** `bi api <COMMAND>`

###### **Subcommands:**

* `tenants` — Tenants
* `realms` — Realms
* `groups` — Groups
* `identities` — Identities
* `credentials` — Credentials



## `bi api tenants`

Tenants

**Usage:** `bi api tenants <COMMAND>`

###### **Subcommands:**

* `get` — Get tenant
* `patch` — Update tenant



## `bi api tenants get`

Get tenant

**Usage:** `bi api tenants get`



## `bi api tenants patch`

Update tenant

**Usage:** `bi api tenants patch --display-name <DISPLAY_NAME>`

###### **Options:**

* `--display-name <DISPLAY_NAME>`



## `bi api realms`

Realms

**Usage:** `bi api realms <COMMAND>`

###### **Subcommands:**

* `create` — Create realm
* `list` — List realms
* `get` — Get realm
* `patch` — Patch realm
* `delete` — Delete realm



## `bi api realms create`

Create realm

**Usage:** `bi api realms create --classification <CLASSIFICATION> --display-name <DISPLAY_NAME>`

###### **Options:**

* `--classification <CLASSIFICATION>`

  Possible values: `secure_customer`, `secure_workforce`

* `--display-name <DISPLAY_NAME>`



## `bi api realms list`

List realms

**Usage:** `bi api realms list [OPTIONS]`

###### **Options:**

* `-n`, `--limit <LIMIT>`



## `bi api realms get`

Get realm

**Usage:** `bi api realms get --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api realms patch`

Patch realm

**Usage:** `bi api realms patch [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `--display-name <DISPLAY_NAME>` — (optional) The display name of the realm



## `bi api realms delete`

Delete realm

**Usage:** `bi api realms delete --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api groups`

Groups

**Usage:** `bi api groups <COMMAND>`

###### **Subcommands:**

* `create` — Create a new group
* `list` — List groups
* `get` — Get a group
* `patch` — Update a group
* `delete` — Delete a group
* `add-members` — Add members to a group
* `delete-members` — Delete members from a group
* `list-members` — List members for a group
* `list-roles` — List role memberships for a group



## `bi api groups create`

Create a new group

**Usage:** `bi api groups create --display-name <DISPLAY_NAME> --description <DESCRIPTION>`

###### **Options:**

* `--display-name <DISPLAY_NAME>`
* `--description <DESCRIPTION>`



## `bi api groups list`

List groups

**Usage:** `bi api groups list [OPTIONS]`

###### **Options:**

* `-n`, `--limit <LIMIT>`



## `bi api groups get`

Get a group

**Usage:** `bi api groups get --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api groups patch`

Update a group

**Usage:** `bi api groups patch [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `--display-name <DISPLAY_NAME>`
* `--description <DESCRIPTION>`



## `bi api groups delete`

Delete a group

**Usage:** `bi api groups delete --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api groups add-members`

Add members to a group

**Usage:** `bi api groups add-members [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `--identity-ids <IDENTITY_IDS>`



## `bi api groups delete-members`

Delete members from a group

**Usage:** `bi api groups delete-members [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `--identity-ids <IDENTITY_IDS>`



## `bi api groups list-members`

List members for a group

**Usage:** `bi api groups list-members [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `-n`, `--limit <LIMIT>`



## `bi api groups list-roles`

List role memberships for a group

**Usage:** `bi api groups list-roles [OPTIONS] --id <ID> --resource-server-id <RESOURCE_SERVER_ID>`

###### **Options:**

* `--id <ID>`
* `--resource-server-id <RESOURCE_SERVER_ID>`
* `-n`, `--limit <LIMIT>`



## `bi api identities`

Identities

**Usage:** `bi api identities <COMMAND>`

###### **Subcommands:**

* `create` — Create a new identity
* `list` — List identities
* `get` — Get an identity
* `patch` — Update an identity
* `delete` — Delete an identity
* `list-groups` — List an identity's groups
* `list-roles` — List an identity's roles



## `bi api identities create`

Create a new identity

**Usage:** `bi api identities create [OPTIONS] --display-name <DISPLAY_NAME> --type <TYPE> --username <USERNAME>`

###### **Options:**

* `--display-name <DISPLAY_NAME>`
* `--type <TYPE>` — (required) The version of the identity's traits

  Possible values: `traits_v0`

* `--username <USERNAME>` — (required) The unique username associated with the identity
* `--primary-email-address <PRIMARY_EMAIL_ADDRESS>` — (optional) The primary email address associated with the identity
* `--external-id <EXTERNAL_ID>` — (optional) An external identifier for the identity
* `--family-name <FAMILY_NAME>` — (optional) The family name (surname) of the identity
* `--given-name <GIVEN_NAME>` — (optional) The given name (first name) of the identity



## `bi api identities list`

List identities

**Usage:** `bi api identities list [OPTIONS]`

###### **Options:**

* `--filter <FILTER>`
* `-n`, `--limit <LIMIT>`



## `bi api identities get`

Get an identity

**Usage:** `bi api identities get --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api identities patch`

Update an identity

**Usage:** `bi api identities patch [OPTIONS] --id <ID> --type <TYPE>`

###### **Options:**

* `--id <ID>`
* `--display-name <DISPLAY_NAME>`
* `--status <STATUS>`

  Possible values: `active`, `suspended`

* `--type <TYPE>`

  Possible values: `traits_v0`

* `--username <USERNAME>`
* `--primary-email-address <PRIMARY_EMAIL_ADDRESS>`
* `--external-id <EXTERNAL_ID>`
* `--family-name <FAMILY_NAME>`
* `--given-name <GIVEN_NAME>`



## `bi api identities delete`

Delete an identity

**Usage:** `bi api identities delete --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api identities list-groups`

List an identity's groups

**Usage:** `bi api identities list-groups [OPTIONS] --id <ID>`

###### **Options:**

* `--id <ID>`
* `-n`, `--limit <LIMIT>`



## `bi api identities list-roles`

List an identity's roles

**Usage:** `bi api identities list-roles [OPTIONS] --id <ID> --resource-server-id <RESOURCE_SERVER_ID>`

###### **Options:**

* `--id <ID>`
* `--resource-server-id <RESOURCE_SERVER_ID>`
* `-n`, `--limit <LIMIT>`



## `bi api credentials`

Credentials

**Usage:** `bi api credentials <COMMAND>`

###### **Subcommands:**

* `list` — List credentials
* `get` — Get a credential
* `revoke` — Revoke a credential



## `bi api credentials list`

List credentials

**Usage:** `bi api credentials list [OPTIONS] --identity-id <IDENTITY_ID>`

###### **Options:**

* `--identity-id <IDENTITY_ID>`
* `-n`, `--limit <LIMIT>`



## `bi api credentials get`

Get a credential

**Usage:** `bi api credentials get --id <ID> --identity-id <IDENTITY_ID>`

###### **Options:**

* `--id <ID>`
* `--identity-id <IDENTITY_ID>`



## `bi api credentials revoke`

Revoke a credential

**Usage:** `bi api credentials revoke --id <ID> --identity-id <IDENTITY_ID>`

###### **Options:**

* `--id <ID>`
* `--identity-id <IDENTITY_ID>`



## `bi helper`

Access helper functions for Beyond Identity API operations

**Usage:** `bi helper <COMMAND>`

###### **Subcommands:**

* `create-admin-account` — Creates an administrator account in the account
* `delete-all-identities` — Deletes all identities from a realm in case you want to set them up from scratch. The identities are unassigned from roles and groups automatically
* `send-enrollment-email` — Helps you send enrollment emails to one or more (or all) users in Beyond Identity
* `review-unenrolled` — Get a list of identities who have not enrolled yet (identities without a passkey)



## `bi helper create-admin-account`

Creates an administrator account in the account

**Usage:** `bi helper create-admin-account <EMAIL>`

###### **Arguments:**

* `<EMAIL>` — Email address of the admin to be created



## `bi helper delete-all-identities`

Deletes all identities from a realm in case you want to set them up from scratch. The identities are unassigned from roles and groups automatically

**Usage:** `bi helper delete-all-identities [OPTIONS] <--all|--norole|--unenrolled>`

###### **Options:**

* `--all`
* `--norole`
* `--unenrolled`
* `--force` — Skip validation when deleting identities



## `bi helper send-enrollment-email`

Helps you send enrollment emails to one or more (or all) users in Beyond Identity

**Usage:** `bi helper send-enrollment-email [OPTIONS] <--all|--groups>`

###### **Options:**

* `--all`
* `--groups`
* `--unenrolled`



## `bi helper review-unenrolled`

Get a list of identities who have not enrolled yet (identities without a passkey)

**Usage:** `bi helper review-unenrolled`



## `bi ai`

Helper tool to generate example commands for CLI operations

**Usage:** `bi ai <COMMAND>`

###### **Subcommands:**

* `ask` — Ask the AI helper tool for assistance in generating CLI commands



## `bi ai ask`

Ask the AI helper tool for assistance in generating CLI commands

**Usage:** `bi ai ask <INPUT>`

###### **Arguments:**

* `<INPUT>` — The question or command you need assistance with



## `bi okta`

Commands solely for fast migration off of Okta

**Usage:** `bi okta <COMMAND>`

###### **Subcommands:**

* `fast-migrate` — Automatically migrate all Okta applications to Beyond Identity SSO and assign users based on existing Okta assignments. Each application tile in Beyond Identity will act as an opaque redirect to Okta



## `bi okta fast-migrate`

Automatically migrate all Okta applications to Beyond Identity SSO and assign users based on existing Okta assignments. Each application tile in Beyond Identity will act as an opaque redirect to Okta

**Usage:** `bi okta fast-migrate`



## `bi onelogin`

Commands solely for fast migration off of OneLogin

**Usage:** `bi onelogin <COMMAND>`

###### **Subcommands:**

* `fast-migrate` — Automatically migrate all OneLogin applications to Beyond Identity SSO and assign users based on existing OneLogin assignments. Each application tile in Beyond Identity will act as an opaque redirect to Onelogin



## `bi onelogin fast-migrate`

Automatically migrate all OneLogin applications to Beyond Identity SSO and assign users based on existing OneLogin assignments. Each application tile in Beyond Identity will act as an opaque redirect to Onelogin

**Usage:** `bi onelogin fast-migrate`




