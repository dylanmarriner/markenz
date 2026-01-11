"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CheckpointHandler = void 0;
class CheckpointHandler {
    db;
    constructor(db) {
        this.db = db;
    }
    async store_hash_checkpoint(tick, world_hash) {
        const prev_hash = await this.get_previous_hash(tick);
        const checkpoint = {
            tick,
            world_hash,
            prev_hash: prev_hash || new Uint8Array(32),
            verified: false,
        };
        await this.db.query(`
      INSERT INTO hash_checkpoints (tick, world_hash, prev_hash, verified, created_at)
      VALUES ($1, $2, $3, $4, NOW())
    `, [tick, Buffer.from(world_hash), Buffer.from(prev_hash), false]);
        // Verify the hash chain
        await this.verify_hash_chain(tick - 1, tick);
    }
    async verify_hash_chain(start_tick, end_tick) {
        const checkpoints = await this.db.query(`
      SELECT tick, world_hash, prev_hash, verified
      FROM hash_checkpoints
      WHERE tick BETWEEN $1 AND $2
      ORDER BY tick ASC
    `, [start_tick, end_tick]);
        let is_valid = true;
        let prev_hash = new Uint8Array(32);
        for (const checkpoint of checkpoints) {
            const current_hash = new Uint8Array(checkpoint.world_hash);
            const expected_prev_hash = new Uint8Array(checkpoint.prev_hash);
            // Verify that previous hash matches expected
            if (prev_hash.length > 0 && !this.arrays_equal(prev_hash, expected_prev_hash)) {
                console.error(`Hash chain broken at tick ${checkpoint.tick}: expected prev_hash ${this.to_hex(expected_prev_hash)}, got ${this.to_hex(prev_hash)}`);
                is_valid = false;
                break;
            }
            prev_hash = current_hash;
        }
        // Update verification status
        await this.db.query(`
      UPDATE hash_checkpoints
      SET verified = $1
      WHERE tick BETWEEN $2 AND $3
    `, [is_valid, start_tick, end_tick]);
        return is_valid;
    }
    async get_hash_checkpoint(tick) {
        const result = await this.db.query(`
      SELECT tick, world_hash, prev_hash, verified, created_at
      FROM hash_checkpoints
      WHERE tick = $1
    `, [tick]);
        if (result.rows.length === 0) {
            return null;
        }
        const row = result.rows[0];
        return {
            tick: row.tick,
            world_hash: new Uint8Array(row.world_hash),
            prev_hash: new Uint8Array(row.prev_hash),
            verified: row.verified,
            created_at: row.created_at,
        };
    }
    async get_hash_checkpoints_in_range(start_tick, end_tick) {
        const result = await this.db.query(`
      SELECT tick, world_hash, prev_hash, verified, created_at
      FROM hash_checkpoints
      WHERE tick BETWEEN $1 AND $2
      ORDER BY tick ASC
    `, [start_tick, end_tick]);
        return result.rows.map(row => ({
            tick: row.tick,
            world_hash: new Uint8Array(row.world_hash),
            prev_hash: new Uint8Array(row.prev_hash),
            verified: row.verified,
            created_at: row.created_at,
        }));
    }
    async get_latest_checkpoint() {
        const result = await this.db.query(`
      SELECT tick, world_hash, prev_hash, verified, created_at
      FROM hash_checkpoints
      ORDER BY tick DESC
      LIMIT 1
    `);
        if (result.rows.length === 0) {
            return null;
        }
        const row = result.rows[0];
        return {
            tick: row.tick,
            world_hash: new Uint8Array(row.world_hash),
            prev_hash: new Uint8Array(row.prev_hash),
            verified: row.verified,
            created_at: row.created_at,
        };
    }
    async get_previous_hash(tick) {
        const result = await this.db.query(`
      SELECT world_hash
      FROM hash_checkpoints
      WHERE tick = $1
    `, [tick - 1]);
        if (result.rows.length === 0) {
            return null;
        }
        return new Uint8Array(result.rows[0].world_hash);
    }
    arrays_equal(a, b) {
        if (a.length !== b.length)
            return false;
        for (let i = 0; i < a.length; i++) {
            if (a[i] !== b[i])
                return false;
        }
        return true;
    }
    to_hex(bytes) {
        return Array.from(bytes)
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');
    }
}
exports.CheckpointHandler = CheckpointHandler;
//# sourceMappingURL=checkpoint_handler.js.map