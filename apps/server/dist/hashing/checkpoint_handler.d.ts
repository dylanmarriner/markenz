import { Database } from '../storage';
export interface HashCheckpoint {
    tick: number;
    world_hash: Uint8Array;
    prev_hash: Uint8Array;
    verified: boolean;
    created_at: Date;
}
export declare class CheckpointHandler {
    private db;
    constructor(db: Database);
    store_hash_checkpoint(tick: number, world_hash: Uint8Array): Promise<void>;
    verify_hash_chain(start_tick: number, end_tick: number): Promise<boolean>;
    get_hash_checkpoint(tick: number): Promise<HashCheckpoint | null>;
    get_hash_checkpoints_in_range(start_tick: number, end_tick: number): Promise<HashCheckpoint[]>;
    get_latest_checkpoint(): Promise<HashCheckpoint | null>;
    private get_previous_hash;
    private arrays_equal;
    private to_hex;
}
//# sourceMappingURL=checkpoint_handler.d.ts.map