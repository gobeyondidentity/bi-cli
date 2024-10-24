# Command-Line Help for `bi-cli`

This document contains the help content for the `bi-cli` command-line program.

**Command Overview:**

* [`bi-cli`↴](#bi-cli)
* [`bi-cli api`↴](#bi-cli-api)
* [`bi-cli api identities`↴](#bi-cli-api-identities)
* [`bi-cli api identities create`↴](#bi-cli-api-identities-create)
* [`bi-cli api identities list`↴](#bi-cli-api-identities-list)
* [`bi-cli api identities get`↴](#bi-cli-api-identities-get)
* [`bi-cli api identities patch`↴](#bi-cli-api-identities-patch)
* [`bi-cli api identities delete`↴](#bi-cli-api-identities-delete)
* [`bi-cli api identities list-groups`↴](#bi-cli-api-identities-list-groups)
* [`bi-cli api identities list-roles`↴](#bi-cli-api-identities-list-roles)
* [`bi-cli helper`↴](#bi-cli-helper)
* [`bi-cli helper setup`↴](#bi-cli-helper-setup)
* [`bi-cli helper setup provision-tenant`↴](#bi-cli-helper-setup-provision-tenant)
* [`bi-cli helper setup list-tenants`↴](#bi-cli-helper-setup-list-tenants)
* [`bi-cli helper setup set-default-tenant`↴](#bi-cli-helper-setup-set-default-tenant)
* [`bi-cli helper setup delete-tenant`↴](#bi-cli-helper-setup-delete-tenant)
* [`bi-cli helper create-scim-app`↴](#bi-cli-helper-create-scim-app)
* [`bi-cli helper create-external-sso-connection`↴](#bi-cli-helper-create-external-sso-connection)
* [`bi-cli helper create-admin-account`↴](#bi-cli-helper-create-admin-account)
* [`bi-cli helper delete-all-identities`↴](#bi-cli-helper-delete-all-identities)
* [`bi-cli helper send-enrollment-email`↴](#bi-cli-helper-send-enrollment-email)
* [`bi-cli helper delete-all-sso-configs`↴](#bi-cli-helper-delete-all-sso-configs)
* [`bi-cli helper review-unenrolled`↴](#bi-cli-helper-review-unenrolled)
* [`bi-cli okta`↴](#bi-cli-okta)
* [`bi-cli okta setup`↴](#bi-cli-okta-setup)
* [`bi-cli okta fast-migrate`↴](#bi-cli-okta-fast-migrate)
* [`bi-cli onelogin`↴](#bi-cli-onelogin)
* [`bi-cli onelogin setup`↴](#bi-cli-onelogin-setup)
* [`bi-cli onelogin fast-migrate`↴](#bi-cli-onelogin-fast-migrate)

## `bi-cli`

Official Beyond Identity command-line interface.

**Usage:** `bi-cli [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `api` — Commands related to Beyond Identity API
* `helper` — Commands related to Beyond Identity API helper functions
* `okta` — Commands related to Okta
* `onelogin` — Commands related to OneLogin

###### **Options:**

* `-l`, `--log-level <LOG_LEVEL>`



## `bi-cli api`

Commands related to Beyond Identity API

**Usage:** `bi-cli api <COMMAND>`

###### **Subcommands:**

* `identities` — Direct API calls for identities



## `bi-cli api identities`

Direct API calls for identities

**Usage:** `bi-cli api identities <COMMAND>`

###### **Subcommands:**

* `create` — 
* `list` — 
* `get` — 
* `patch` — 
* `delete` — 
* `list-groups` — 
* `list-roles` — 



## `bi-cli api identities create`

**Usage:** `bi-cli api identities create [OPTIONS] --display-name <DISPLAY_NAME> --type <TYPE> --username <USERNAME>`

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



## `bi-cli api identities list`

**Usage:** `bi-cli api identities list [OPTIONS]`

###### **Options:**

* `--filter <FILTER>`



## `bi-cli api identities get`

**Usage:** `bi-cli api identities get --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi-cli api identities patch`

**Usage:** `bi-cli api identities patch [OPTIONS] --id <ID> --type <TYPE>`

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



## `bi-cli api identities delete`

**Usage:** `bi-cli api identities delete --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi-cli api identities list-groups`

**Usage:** `bi-cli api identities list-groups --id <ID>`

###### **Options:**

* `--id <ID>`



## `bi-cli api identities list-roles`

**Usage:** `bi-cli api identities list-roles --id <ID> --resource-server-id <RESOURCE_SERVER_ID>`

###### **Options:**

* `--id <ID>`
* `--resource-server-id <RESOURCE_SERVER_ID>`



## `bi-cli helper`

Commands related to Beyond Identity API helper functions

**Usage:** `bi-cli helper <COMMAND>`

###### **Subcommands:**

* `setup` — Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied
* `create-scim-app` — Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider
* `create-external-sso-connection` — Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity
* `create-admin-account` — Creates an administrator account in the account
* `delete-all-identities` — Deletes all identities from a realm in case you want to set them up from scratch. The identities are unassigned from roles and groups automatically
* `send-enrollment-email` — Helps you send enrollment emails to one or more (or all) users in Beyond Identity
* `delete-all-sso-configs` — Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch
* `review-unenrolled` — Get a list of identities who have not enrolled yet (identities without a passkey)



## `bi-cli helper setup`

Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied

**Usage:** `bi-cli helper setup <COMMAND>`

###### **Subcommands:**

* `provision-tenant` — Provisions an existing tenant using the given API token
* `list-tenants` — Lists all provisioned tenants
* `set-default-tenant` — Update which tenant is the default one
* `delete-tenant` — Delete any provisioned tenants



## `bi-cli helper setup provision-tenant`

Provisions an existing tenant using the given API token

**Usage:** `bi-cli helper setup provision-tenant <TOKEN>`

###### **Arguments:**

* `<TOKEN>`



## `bi-cli helper setup list-tenants`

Lists all provisioned tenants

**Usage:** `bi-cli helper setup list-tenants`



## `bi-cli helper setup set-default-tenant`

Update which tenant is the default one

**Usage:** `bi-cli helper setup set-default-tenant`



## `bi-cli helper setup delete-tenant`

Delete any provisioned tenants

**Usage:** `bi-cli helper setup delete-tenant`



## `bi-cli helper create-scim-app`

Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider

**Usage:** `bi-cli helper create-scim-app <OKTA_REGISTRATION_SYNC_ATTRIBUTE>`

###### **Arguments:**

* `<OKTA_REGISTRATION_SYNC_ATTRIBUTE>` — Attribute that controls how and when Okta users are routed to Beyond Identity



## `bi-cli helper create-external-sso-connection`

Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity

**Usage:** `bi-cli helper create-external-sso-connection`



## `bi-cli helper create-admin-account`

Creates an administrator account in the account

**Usage:** `bi-cli helper create-admin-account <EMAIL>`

###### **Arguments:**

* `<EMAIL>` — Email address of the admin to be created



## `bi-cli helper delete-all-identities`

Deletes all identities from a realm in case you want to set them up from scratch. The identities are unassigned from roles and groups automatically

**Usage:** `bi-cli helper delete-all-identities [OPTIONS] <--all|--norole|--unenrolled>`

###### **Options:**

* `--all`
* `--norole`
* `--unenrolled`
* `--force` — Skip validation when deleting identities



## `bi-cli helper send-enrollment-email`

Helps you send enrollment emails to one or more (or all) users in Beyond Identity

**Usage:** `bi-cli helper send-enrollment-email <--all|--unenrolled|--groups>`

###### **Options:**

* `--all`
* `--unenrolled`
* `--groups`



## `bi-cli helper delete-all-sso-configs`

Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch

**Usage:** `bi-cli helper delete-all-sso-configs`



## `bi-cli helper review-unenrolled`

Get a list of identities who have not enrolled yet (identities without a passkey)

**Usage:** `bi-cli helper review-unenrolled`



## `bi-cli okta`

Commands related to Okta

**Usage:** `bi-cli okta <COMMAND>`

###### **Subcommands:**

* `setup` — Setup allows you to provision an Okta tenant to be used for subsequent commands
* `fast-migrate` — Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta



## `bi-cli okta setup`

Setup allows you to provision an Okta tenant to be used for subsequent commands

**Usage:** `bi-cli okta setup [OPTIONS] <DOMAIN> <API_KEY>`

###### **Arguments:**

* `<DOMAIN>`
* `<API_KEY>`

###### **Options:**

* `--force` — Flag to allow force reconfiguration



## `bi-cli okta fast-migrate`

Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta

**Usage:** `bi-cli okta fast-migrate`



## `bi-cli onelogin`

Commands related to OneLogin

**Usage:** `bi-cli onelogin <COMMAND>`

###### **Subcommands:**

* `setup` — Setup allows you to provision a Onelogin tenant to be used for subsequent commands
* `fast-migrate` — Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin



## `bi-cli onelogin setup`

Setup allows you to provision a Onelogin tenant to be used for subsequent commands

**Usage:** `bi-cli onelogin setup [OPTIONS] <DOMAIN> <CLIENT_ID> <CLIENT_SECRET>`

###### **Arguments:**

* `<DOMAIN>`
* `<CLIENT_ID>`
* `<CLIENT_SECRET>`

###### **Options:**

* `--force` — Flag to allow force reconfiguration



## `bi-cli onelogin fast-migrate`

Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin

**Usage:** `bi-cli onelogin fast-migrate`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

