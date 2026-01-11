-- RNG Audit Log Table for Phase 1 Determinism
-- Stores every random draw with full audit trail

CREATE TABLE IF NOT EXISTS rng_audit_log (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    subsystem TEXT NOT NULL,       -- "Physics", "Biology", etc.
    stream_id BIGINT NOT NULL,
    callsite TEXT NOT NULL,        -- "physics.rs:42"
    value BIGINT NOT NULL,         -- The actual u64 value
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_rng_tick ON rng_audit_log(tick);
CREATE INDEX IF NOT EXISTS idx_rng_subsystem ON rng_audit_log(subsystem);
CREATE INDEX IF NOT EXISTS idx_rng_stream ON rng_audit_log(subsystem, stream_id);

-- Ensure table is append-only (no updates/deletes)
-- This will be enforced at application level
