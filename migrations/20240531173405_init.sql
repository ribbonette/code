CREATE TABLE servers (
	id bigint primary key,
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE server_member_links (
	id bigserial primary key,
	display_name text NOT NULL,
	creator_id bigint,
	server_id bigint REFERENCES servers NOT NULL,

	connectors jsonb NOT NULL,
	criteria jsonb NOT NULL,

	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
	id bigint primary key,
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_roblox_accounts (
	id bigserial primary key,
	roblox_id bigint NOT NULL,
	user_id bigint REFERENCES users NOT NULL,

	access_token text NOT NULL,
	refresh_token text NOT NULL,
	token_type text NOT NULL,
	expires_at timestamptz NOT NULL,
	scopes text[] NOT NULL DEFAULT '{}',

	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);