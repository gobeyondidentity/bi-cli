# bi

This document contains the help content for the `bi` command-line program.

**Command Overview:**

* [`bi`↴](#bi)
* [`bi api`↴](#bi-api)
* [`bi api tenants`↴](#bi-api-tenants)
* [`bi api tenants get`↴](#bi-api-tenants-get)
* [`bi api tenants patch`↴](#bi-api-tenants-patch)
* [`bi api identities`↴](#bi-api-identities)
* [`bi api identities create`↴](#bi-api-identities-create)
* [`bi api identities list`↴](#bi-api-identities-list)
* [`bi api identities get`↴](#bi-api-identities-get)
* [`bi api identities patch`↴](#bi-api-identities-patch)
* [`bi api identities delete`↴](#bi-api-identities-delete)
* [`bi api identities list-groups`↴](#bi-api-identities-list-groups)
* [`bi api identities list-roles`↴](#bi-api-identities-list-roles)
* [`bi helper`↴](#bi-helper)
* [`bi helper setup`↴](#bi-helper-setup)
* [`bi helper setup provision-tenant`↴](#bi-helper-setup-provision-tenant)
* [`bi helper setup list-tenants`↴](#bi-helper-setup-list-tenants)
* [`bi helper setup set-default-tenant`↴](#bi-helper-setup-set-default-tenant)
* [`bi helper setup delete-tenant`↴](#bi-helper-setup-delete-tenant)
* [`bi helper create-scim-app`↴](#bi-helper-create-scim-app)
* [`bi helper create-external-sso-connection`↴](#bi-helper-create-external-sso-connection)
* [`bi helper create-admin-account`↴](#bi-helper-create-admin-account)
* [`bi helper delete-all-identities`↴](#bi-helper-delete-all-identities)
* [`bi helper send-enrollment-email`↴](#bi-helper-send-enrollment-email)
* [`bi helper delete-all-sso-configs`↴](#bi-helper-delete-all-sso-configs)
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
* `identities` — Identities



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

* `--display-name <DISPLAY_NAME>` — (required) The display name of the identity
* `--status <STATUS>` — (optional) Indicator for the identity's administrative status

  Possible values: `active`, `suspended`

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

**Usage:** `bi api identities list-groups --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi api identities list-roles`

List an identity's roles

**Usage:** `bi api identities list-roles --id <ID> --resource-server-id <RESOURCE_SERVER_ID>`

###### **Options:**

* `--id <ID>`
* `--resource-server-id <RESOURCE_SERVER_ID>`



## `bi helper`

Commands related to Beyond Identity API helper functions

**Usage:** `bi helper <COMMAND>`

###### **Subcommands:**

* `setup` — Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied
* `create-scim-app` — Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider
* `create-external-sso-connection` — Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity
* `create-admin-account` — Creates an administrator account in the account
* `delete-all-identities` — Deletes all identities from a realm in case you want to set them up from scratch. The identities are unassigned from roles and groups automatically
* `send-enrollment-email` — Helps you send enrollment emails to one or more (or all) users in Beyond Identity
* `delete-all-sso-configs` — Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch
* `review-unenrolled` — Get a list of identities who have not enrolled yet (identities without a passkey)



## `bi helper setup`

Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied

**Usage:** `bi helper setup <COMMAND>`

###### **Subcommands:**

* `provision-tenant` — Provisions an existing tenant using the given API token
* `list-tenants` — Lists all provisioned tenants
* `set-default-tenant` — Update which tenant is the default one
* `delete-tenant` — Delete any provisioned tenants



## `bi helper setup provision-tenant`

Provisions an existing tenant using the given API token

**Usage:** `bi helper setup provision-tenant <TOKEN>`

###### **Arguments:**

* `<TOKEN>`



## `bi helper setup list-tenants`

Lists all provisioned tenants

**Usage:** `bi helper setup list-tenants`



## `bi helper setup set-default-tenant`

Update which tenant is the default one

**Usage:** `bi helper setup set-default-tenant`



## `bi helper setup delete-tenant`

Delete any provisioned tenants

**Usage:** `bi helper setup delete-tenant`



## `bi helper create-scim-app`

Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider

**Usage:** `bi helper create-scim-app <OKTA_REGISTRATION_SYNC_ATTRIBUTE>`

###### **Arguments:**

* `<OKTA_REGISTRATION_SYNC_ATTRIBUTE>` — Attribute that controls how and when Okta users are routed to Beyond Identity



## `bi helper create-external-sso-connection`

Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity

**Usage:** `bi helper create-external-sso-connection`



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



## `bi helper delete-all-sso-configs`

Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch

**Usage:** `bi helper delete-all-sso-configs`



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

**Usage:** `bi okta setup [OPTIONS] <DOMAIN> <API_KEY>`

###### **Arguments:**

* `<DOMAIN>`
* `<API_KEY>`

###### **Options:**

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

**Usage:** `bi onelogin setup [OPTIONS] <DOMAIN> <CLIENT_ID> <CLIENT_SECRET>`

###### **Arguments:**

* `<DOMAIN>`
* `<CLIENT_ID>`
* `<CLIENT_SECRET>`

###### **Options:**

* `--force` — Flag to allow force reconfiguration



## `bi onelogin fast-migrate`

Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin

**Usage:** `bi onelogin fast-migrate`




