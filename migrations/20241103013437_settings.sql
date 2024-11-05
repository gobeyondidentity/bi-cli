-- 1. Create the new settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- 2. Migrate Okta configuration to settings table
INSERT INTO settings (key, value)
SELECT 'okta_config', json_object('domain', domain, 'api_key', api_key)
FROM okta_config
WHERE id = 1;

-- 3. Migrate OneLogin configuration to settings table
INSERT INTO settings (key, value)
SELECT 'onelogin_config', json_object('domain', domain, 'client_id', client_id, 'client_secret', client_secret)
FROM onelogin_config
WHERE id = 1;

-- 4. Drop old configuration tables
DROP TABLE IF EXISTS okta_config;
DROP TABLE IF EXISTS onelogin_config;
