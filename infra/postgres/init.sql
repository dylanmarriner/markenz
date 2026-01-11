-- Connect to markenz database (created by postgres image)
\c markenz;

-- Create immutable append-only tables
CREATE TABLE IF NOT EXISTS input_events (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    source_agent_id BIGINT NOT NULL,
    sequence BIGINT NOT NULL,
    payload_json JSONB NOT NULL,
    hash BYTEA NOT NULL,
    prev_hash BYTEA,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS observation_events (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    input_id BIGINT REFERENCES input_events(id) ON DELETE SET NULL,
    payload_json JSONB NOT NULL,
    hash BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS snapshots (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL UNIQUE,
    state_blob BYTEA NOT NULL,
    world_hash BYTEA NOT NULL,
    input_hash BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS hash_checkpoints (
    tick BIGINT PRIMARY KEY,
    world_hash BYTEA NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enforce append-only constraints (no UPDATE/DELETE on immutable tables)
CREATE RULE prevent_update_input_events AS ON UPDATE TO input_events DO INSTEAD NOTHING;
CREATE RULE prevent_delete_input_events AS ON DELETE TO input_events DO INSTEAD NOTHING;

CREATE RULE prevent_update_observation_events AS ON UPDATE TO observation_events DO INSTEAD NOTHING;
CREATE RULE prevent_delete_observation_events AS ON DELETE TO observation_events DO INSTEAD NOTHING;

CREATE RULE prevent_update_snapshots AS ON UPDATE TO snapshots DO INSTEAD NOTHING;
CREATE RULE prevent_delete_snapshots AS ON DELETE TO snapshots DO INSTEAD NOTHING;

-- Create indexes for performance
CREATE UNIQUE INDEX idx_input_events_hash ON input_events(hash);
CREATE INDEX idx_input_events_tick ON input_events(tick);
CREATE INDEX idx_input_events_source_agent_id ON input_events(source_agent_id);
CREATE INDEX idx_observation_events_tick ON observation_events(tick);
CREATE INDEX idx_observation_events_input_id ON observation_events(input_id);
CREATE INDEX idx_snapshots_tick ON snapshots(tick);

-- Set up hash-chain foreign keys
ALTER TABLE input_events ADD CONSTRAINT fk_input_events_prev_hash 
    FOREIGN KEY (prev_hash) REFERENCES input_events(hash) DEFERRABLE INITIALLY DEFERRED;
CREATE INDEX idx_hash_checkpoints_tick ON hash_checkpoints(tick);

-- Seed initial JWKS cache table for Keycloak OIDC
CREATE TABLE IF NOT EXISTS jwks_cache (
    kid VARCHAR(255) PRIMARY KEY,
    public_key TEXT NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_jwks_cache_expires_at ON jwks_cache(expires_at);
