CREATE TABLE open_cloud_authorisations (
	id bigserial primary key,
	author_id bigint REFERENCES users(id),

	access_token text NOT NULL,
	refresh_token text NOT NULL,
	token_type text NOT NULL,
	metadata jsonb NOT NULL,

	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	expires_at timestamptz NOT NULL
);

CREATE TABLE server_integrations (
	id bigserial primary key,
	kind text NOT NULL,
	author_id bigint REFERENCES users(id),
	server_id bigint REFERENCES servers(id) NOT NULL,

	authorisation_id bigint REFERENCES open_cloud_authorisations(id),

	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE user_roblox_accounts
	DROP CONSTRAINT user_roblox_accounts_user_id_fkey,
	ADD CONSTRAINT user_roblox_accounts_user_id_fkey FOREIGN KEY (user_id) REFERENCES users (id);

ALTER TABLE user_roblox_accounts
	DROP COLUMN access_token,
	DROP COLUMN refresh_token,
	DROP COLUMN token_type,
	DROP COLUMN scopes,
	DROP COLUMN expires_at,
	ADD COLUMN authorisation_id bigint REFERENCES open_cloud_authorisations(id);