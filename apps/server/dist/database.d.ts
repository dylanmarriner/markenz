/**
 * PostgreSQL adapter for Markenz Server
 * Enforces append-only behavior and fail-closed startup
 */
export interface InputEvent {
    tick: bigint;
    source_id: bigint;
    payload_json: object;
    hash: Buffer;
    prev_hash: Buffer | null;
}
export interface ObservationEvent {
    tick: bigint;
    input_id: bigint | null;
    payload_json: object;
    hash: Buffer;
}
export interface Snapshot {
    tick: bigint;
    state_blob: Buffer;
    world_hash: Buffer;
    input_hash: Buffer;
}
export interface HashCheckpoint {
    tick: bigint;
    world_hash: Buffer;
    verified: boolean;
}
export declare class Database {
    private pool;
    constructor(connectionString: string);
    initialize(): Promise<void>;
    private verifySchema;
    appendInputEvent(event: InputEvent): Promise<void>;
    appendObservationEvent(event: ObservationEvent): Promise<void>;
    writeSnapshot(snapshot: Snapshot): Promise<void>;
    writeHashCheckpoint(checkpoint: HashCheckpoint): Promise<void>;
    getInputEventsForTick(tick: bigint): Promise<InputEvent[]>;
    getLatestSnapshot(): Promise<Snapshot | null>;
    close(): Promise<void>;
    static computeHash(data: object): Buffer;
    verifyHashChain(fromTick: bigint, toTick: bigint): Promise<boolean>;
}
//# sourceMappingURL=database.d.ts.map