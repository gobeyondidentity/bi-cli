# bi

This document contains the help content for the `bi` command-line program.

**Command Overview:**

* [`bi`↴](#bi)
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
* [`bi setup`↴](#bi-setup)
* [`bi setup tenants`↴](#bi-setup-tenants)
* [`bi setup tenants provision`↴](#bi-setup-tenants-provision)
* [`bi setup tenants list`↴](#bi-setup-tenants-list)
* [`bi setup tenants set-default`↴](#bi-setup-tenants-set-default)
* [`bi setup tenants remove`↴](#bi-setup-tenants-remove)
* [`bi helper`↴](#bi-helper)
* [`bi helper create-admin-account`↴](#bi-helper-create-admin-account)
* [`bi helper delete-all-identities`↴](#bi-helper-delete-all-identities)
* [`bi helper send-enrollment-email`↴](#bi-helper-send-enrollment-email)
* [`bi helper review-unenrolled`↴](#bi-helper-review-unenrolled)
* [`bi okta`↴](#bi-okta)
* [`bi okta setup`↴](#bi-okta-setup)
* [`bi okta fast-migrate`↴](#bi-okta-fast-migrate)
* [`bi onelogin`↴](#bi-onelogin)
* [`bi onelogin setup`↴](#bi-onelogin-setup)
* [`bi onelogin fast-migrate`↴](#bi-onelogin-fast-migrate)

## `bi`

Official Beyond Identity command-line interface.

**Usage:** `bi [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `api` — Commands related to Beyond Identity API
* `setup` — Commands related to Beyond Identity API
* `helper` — Commands related to Beyond Identity API helper functions
* `okta` — Commands related to Okta
* `onelogin` — Commands related to OneLogin

###### **Options:**

* `-l`, `--log-level <LOG_LEVEL>`



## `bi api`

Commands related to Beyond Identity API

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



## `bi setup`

Commands related to Beyond Identity API

**Usage:** `bi setup <COMMAND>`

###### **Subcommands:**

* `tenants` — Tenant management actions



## `bi setup tenants`

Tenant management actions

**Usage:** `bi setup tenants <COMMAND>`

###### **Subcommands:**

* `provision` — Provisions an existing tenant using the provided API token
* `list` — Display a list of all currently provisioned tenants
* `set-default` — Set a specific teannt as the default
* `remove` — Remove a tenant from the list of provisioned tenants



## `bi setup tenants provision`

Provisions an existing tenant using the provided API token

**Usage:** `bi setup tenants provision --token <TOKEN>`

###### **Options:**

* `--token <TOKEN>`



## `bi setup tenants list`

Display a list of all currently provisioned tenants

**Usage:** `bi setup tenants list`



## `bi setup tenants set-default`

Set a specific teannt as the default

**Usage:** `bi setup tenants set-default`



## `bi setup tenants remove`

Remove a tenant from the list of provisioned tenants

**Usage:** `bi setup tenants remove`



## `bi helper`

Commands related to Beyond Identity API helper functions

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



## `bi okta`

Commands related to Okta

**Usage:** `bi okta <COMMAND>`

###### **Subcommands:**

* `setup` — Setup allows you to provision an Okta tenant to be used for subsequent commands
* `fast-migrate` — Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta



## `bi okta setup`

Setup allows you to provision an Okta tenant to be used for subsequent commands

**Usage:** `bi okta setup [OPTIONS] --domain <DOMAIN> --api-key <API_KEY>`

###### **Options:**

* `--domain <DOMAIN>` — Okta domain
* `--api-key <API_KEY>` — Okta API key
* `--force` — Flag to allow force reconfiguration



## `bi okta fast-migrate`

Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta

**Usage:** `bi okta fast-migrate`



## `bi onelogin`

Commands related to OneLogin

**Usage:** `bi onelogin <COMMAND>`

###### **Subcommands:**

* `setup` — Setup allows you to provision a Onelogin tenant to be used for subsequent commands
* `fast-migrate` — Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin



## `bi onelogin setup`

Setup allows you to provision a Onelogin tenant to be used for subsequent commands

**Usage:** `bi onelogin setup [OPTIONS] --domain <DOMAIN> --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET>`

###### **Options:**

* `--domain <DOMAIN>` — Onelogin domain
* `--client-id <CLIENT_ID>` — Onelogin client id
* `--client-secret <CLIENT_SECRET>` — Onelogin client secret
* `--force` — Flag to allow force reconfiguration



## `bi onelogin fast-migrate`

Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin

**Usage:** `bi onelogin fast-migrate`




