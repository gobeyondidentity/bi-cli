CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS realms (
    id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    application_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    open_id_configuration_url TEXT NOT NULL,
    auth_base_url TEXT NOT NULL,
    api_base_url TEXT NOT NULL,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS defaults (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    tenant_id TEXT NOT NULL,
    realm_id TEXT NOT NULL,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
    FOREIGN KEY (tenant_id, realm_id) REFERENCES realms(tenant_id, id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS tokens (
    access_token TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    tenant_id TEXT NOT NULL,
    realm_id TEXT NOT NULL,
    application_id TEXT NOT NULL,
    PRIMARY KEY (tenant_id, realm_id)
);

CREATE TABLE IF NOT EXISTS okta_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    domain TEXT NOT NULL,
    api_key TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS onelogin_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    domain TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL
);
