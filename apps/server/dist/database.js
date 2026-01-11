"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Database = void 0;
const pg_1 = require("pg");
class Database {
    pool;
    constructor(connectionString) {
        this.pool = new pg_1.Pool({
            connectionString,
            max: 20,
            idleTimeoutMillis: 30000,
            connectionTimeoutMillis: 2000,
        });
    }
    async initialize() {
        const client = await this.pool.connect();
        try {
            // Verify all required tables exist
            await this.verifySchema(client);
            console.log('Database schema verified');
        }
        finally {
            client.release();
        }
    }
    async verifySchema(client) {
        const requiredTables = [
            'input_events',
            'observation_events',
            'snapshots',
            'hash_checkpoints',
            'jwks_cache'
        ];
        for (const table of requiredTables) {
            const result = await client.query('SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = $1)', [table]);
            if (!result.rows[0].exists) {
                throw new Error(`Required table '${table}' does not exist. Run migrations first.`);
            }
        }
        // Verify append-only rules are in place
        const rules = await client.query(`
            SELECT rulename 
            FROM pg_rules 
            WHERE tablename IN ('input_events', 'observation_events', 'snapshots')
        `);
        const expectedRules = [
            'prevent_update_input_events',
            'prevent_delete_input_events',
            'prevent_update_observation_events',
            'prevent_delete_observation_events',
            'prevent_update_snapshots',
            'prevent_delete_snapshots'
        ];
        for (const rule of expectedRules) {
            if (!rules.rows.some(row => row.rulename === rule)) {
                throw new Error(`Append-only rule '${rule}' is missing. Schema integrity compromised.`);
            }
        }
    }
    async appendInputEvent(event) {
        const client = await this.pool.connect();
        try {
            const query = `
                INSERT INTO input_events (tick, source_id, payload_json, hash, prev_hash)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id
            `;
            const result = await client.query(query, [
                event.tick,
                event.source_id,
                JSON.stringify(event.payload_json),
                event.hash,
                event.prev_hash
            ]);
            console.log(`InputEvent appended with ID: ${result.rows[0].id}`);
        }
        finally {
            client.release();
        }
    }
    async appendObservationEvent(event) {
        const client = await this.pool.connect();
        try {
            const query = `
                INSERT INTO observation_events (tick, input_id, payload_json, hash)
                VALUES ($1, $2, $3, $4)
                RETURNING id
            `;
            const result = await client.query(query, [
                event.tick,
                event.input_id,
                JSON.stringify(event.payload_json),
                event.hash
            ]);
            console.log(`ObservationEvent appended with ID: ${result.rows[0].id}`);
        }
        finally {
            client.release();
        }
    }
    async writeSnapshot(snapshot) {
        const client = await this.pool.connect();
        try {
            const query = `
                INSERT INTO snapshots (tick, state_blob, world_hash, input_hash)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (tick) DO UPDATE SET
                    state_blob = EXCLUDED.state_blob,
                    world_hash = EXCLUDED.world_hash,
                    input_hash = EXCLUDED.input_hash
                RETURNING id
            `;
            const result = await client.query(query, [
                snapshot.tick,
                snapshot.state_blob,
                snapshot.world_hash,
                snapshot.input_hash
            ]);
            console.log(`Snapshot written for tick ${snapshot.tick} with ID: ${result.rows[0].id}`);
        }
        finally {
            client.release();
        }
    }
    async writeHashCheckpoint(checkpoint) {
        const client = await this.pool.connect();
        try {
            const query = `
                INSERT INTO hash_checkpoints (tick, world_hash, verified)
                VALUES ($1, $2, $3)
                ON CONFLICT (tick) DO UPDATE SET
                    world_hash = EXCLUDED.world_hash,
                    verified = EXCLUDED.verified
            `;
            await client.query(query, [
                checkpoint.tick,
                checkpoint.world_hash,
                checkpoint.verified
            ]);
            console.log(`HashCheckpoint written for tick ${checkpoint.tick}`);
        }
        finally {
            client.release();
        }
    }
    async getInputEventsForTick(tick) {
        const client = await this.pool.connect();
        try {
            const query = `
                SELECT id, tick, source_id, payload_json, hash, prev_hash
                FROM input_events
                WHERE tick = $1
                ORDER BY id ASC
            `;
            const result = await client.query(query, [tick]);
            return result.rows.map(row => ({
                tick: BigInt(row.tick),
                source_id: BigInt(row.source_id),
                payload_json: row.payload_json,
                hash: row.hash,
                prev_hash: row.prev_hash
            }));
        }
        finally {
            client.release();
        }
    }
    async getLatestSnapshot() {
        const client = await this.pool.connect();
        try {
            const query = `
                SELECT tick, state_blob, world_hash, input_hash
                FROM snapshots
                ORDER BY tick DESC
                LIMIT 1
            `;
            const result = await client.query(query);
            if (result.rows.length === 0) {
                return null;
            }
            const row = result.rows[0];
            return {
                tick: BigInt(row.tick),
                state_blob: row.state_blob,
                world_hash: row.world_hash,
                input_hash: row.input_hash
            };
        }
        finally {
            client.release();
        }
    }
    async close() {
        await this.pool.end();
        console.log('Database connection pool closed');
    }
    // Utility method to compute hash for events
    static computeHash(data) {
        const jsonString = JSON.stringify(data, Object.keys(data).sort());
        return createHash('sha256').update(jsonString).digest();
    }
    // Utility method to verify hash chain integrity
    async verifyHashChain(fromTick, toTick) {
        const client = await this.pool.connect();
        try {
            const query = `
                SELECT id, hash, prev_hash
                FROM input_events
                WHERE tick >= $1 AND tick <= $2
                ORDER BY tick ASC, id ASC
            `;
            const result = await client.query(query, [fromTick, toTick]);
            let prevHash = null;
            for (const row of result.rows) {
                if (prevHash && !row.prev_hash?.equals(prevHash)) {
                    console.error(`Hash chain broken at event ID ${row.id}`);
                    return false;
                }
                prevHash = row.hash;
            }
            return true;
        }
        finally {
            client.release();
        }
    }
}
exports.Database = Database;
//# sourceMappingURL=database.js.map