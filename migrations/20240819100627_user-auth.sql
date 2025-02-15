CREATE TABLE user_sessions (
	id bigserial primary key,
	public_key bytea NOT NULL,
	user_id bigint REFERENCES users NOT NULL,
	
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);