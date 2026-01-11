import { Database } from '../database';

export interface HashCheckpoint {
  tick: number;
  world_hash: Uint8Array;
  prev_hash: Uint8Array;
  verified: boolean;
  created_at: Date;
}

export class CheckpointHandler {
  private db: Database;

  constructor(db: Database) {
    this.db = db;
  }

  async store_hash_checkpoint(tick: number, world_hash: Uint8Array): Promise<void> {
    await this.db.writeHashCheckpoint({
      tick: BigInt(tick),
      world_hash: Buffer.from(world_hash),
      verified: false
    });

    // Verify the hash chain
    await this.verify_hash_chain(tick - 1, tick);
  }

  async verify_hash_chain(start_tick: number, end_tick: number): Promise<boolean> {
    const result = await this.db.getHashCheckpointsInRange(BigInt(start_tick), BigInt(end_tick));

    let is_valid = true;
    let prev_hash = new Uint8Array(32);

    for (const checkpoint of result.rows) {
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

  async get_hash_checkpoint(tick: number): Promise<HashCheckpoint | null> {
    const result = await this.db.getHashCheckpoint(BigInt(tick));

    if (result.rows.length === 0) {
      return null;
    }

    const row = result.rows[0];
    return {
      tick: row.tick,
      world_hash: new Uint8Array(row.world_hash),
      prev_hash: new Uint8Array(row.prev_hash || new Uint8Array(32)),
      verified: row.verified,
      created_at: row.created_at,
    };
  }

  async get_hash_checkpoints_in_range(start_tick: number, end_tick: number): Promise<HashCheckpoint[]> {
    const result = await this.db.getHashCheckpointsInRange(BigInt(start_tick), BigInt(end_tick));

    return result.rows.map((row: any) => ({
      tick: row.tick,
      world_hash: new Uint8Array(row.world_hash),
      prev_hash: new Uint8Array(row.prev_hash),
      verified: row.verified,
      created_at: row.created_at,
    }));
  }

  async get_latest_checkpoint(): Promise<HashCheckpoint | null> {
    const result = await this.db.getLatestHashCheckpoint();

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

  private arrays_equal(a: Uint8Array, b: Uint8Array): boolean {
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) return false;
    }
    return true;
  }

  private to_hex(bytes: Uint8Array): string {
    return Array.from(bytes)
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  }
}
