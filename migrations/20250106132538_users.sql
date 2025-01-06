-- Add migration script here

-- create user table
CREATE TABLE IF NOT EXISTS users(
  id bigserial PRIMARY KEY,
  ws_id bigint NOT NULL,
  username varchar(64) NOT NULL,
  email varchar(64) NOT NULL,
  -- hashed argon2 password, length 97
  password_hash varchar(97) NOT NULL,
  created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
