---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_8_RENDERING
phase: 8
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: WebGPU Renderer · Chunk Streaming · Performance · Observable Rendering
requires: PLAN_PHASE_7_GOVERNANCE (100% complete)
---

# PLAN PHASE 8: RENDERING
## (WebGPU Voxel Renderer · Chunk Streaming · Transparency UI)

**AUDIENCE:** Windsurf executor  
**MODE:** BINDING · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement WebGPU rendering pipeline:
- Voxel renderer: blocks, terrain, agents
- Chunk streaming: binary WebSocket protocol
- Transparency: full state inspection UI
- Performance: 60 FPS target on modern hardware
- Replay viewer: time-travel debugging UI

---

## 2. WEBGPU RENDERER

### 2.1 Rendering Pipeline (apps/web/src/renderer/webgpu_renderer.ts)

```typescript
export class WebGPURenderer {
    device: GPUDevice;
    queue: GPUQueue;
    pipeline: GPURenderPipeline;
    
    constructor(canvas: HTMLCanvasElement) {
        // Initialize WebGPU device
        // Compile shader programs
    }
    
    render(
        terrain: TerrainData,
        agents: AgentData[],
        camera: CameraState,
    ): void {
        // Render terrain chunks
        // Render agents
        // Render UI overlays
    }
}
```

### 2.2 Shader Code (apps/web/src/shaders/voxel.wgsl)

```wgsl
@vertex
fn vertex_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.mvp_matrix * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    out.normal = in.normal;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let lighting = dot(normalize(in.normal), vec3<f32>(0.5, 1.0, 0.5));
    return vec4<f32>(in.color * lighting, 1.0);
}
```

---

## 3. CHUNK STREAMING

### 3.1 Chunk Protocol (crates/server/src/streaming/chunk_stream.rs)

```rust
pub struct ChunkStreamMessage {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub voxel_data: Vec<u8>,  // Run-length encoded
}

pub fn encode_chunk(chunk: &Chunk) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut current_block = chunk.voxels[0].block_type;
    let mut count: u32 = 1;
    
    for voxel in &chunk.voxels[1..] {
        if voxel.block_type == current_block && count < 255 {
            count += 1;
        } else {
            // Write run: (block_type, count)
            encoded.push(current_block as u8);
            encoded.push(count as u8);
            current_block = voxel.block_type;
            count = 1;
        }
    }
    
    // Write final run
    encoded.push(current_block as u8);
    encoded.push(count as u8);
    
    encoded
}

pub fn decode_chunk(data: &[u8], chunk_x: i32, chunk_y: i32) -> Result<Chunk, String> {
    let mut voxels = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let block_type = BlockType::from_u8(data[i])?;
        let count = data[i + 1] as usize;
        
        for _ in 0..count {
            voxels.push(Voxel {
                block_type,
                height: 0,
            });
        }
        
        i += 2;
    }
    
    Ok(Chunk {
        position: (chunk_x, chunk_y),
        voxels,
        heightmap: Vec::new(),
        biome_map: Vec::new(),
    })
}
```

---

## 4. TRANSPARENCY UI

### 4.1 State Inspector (apps/web/src/ui/state_inspector.tsx)

```typescript
export const StateInspector: React.FC = () => {
    const [worldState, setWorldState] = useState<UniverseState | null>(null);
    const [selectedAgent, setSelectedAgent] = useState<u64 | null>(null);
    
    useEffect(() => {
        const unsubscribe = observationStream.subscribe((obs) => {
            if (obs.type === 'world_state_snapshot') {
                setWorldState(obs.payload);
            }
        });
        return unsubscribe;
    }, []);
    
    return (
        <div className="state-inspector">
            <h2>World State</h2>
            <div className="agents">
                {worldState?.agents.map(agent => (
                    <div
                        key={agent.id}
                        onClick={() => setSelectedAgent(agent.id)}
                        className={selectedAgent === agent.id ? 'selected' : ''}
                    >
                        <h3>{agent.name}</h3>
                        <p>Energy: {agent.bio_state.energy.toFixed(1)}</p>
                        <p>Position: ({agent.position[0].toFixed(1)}, {agent.position[1].toFixed(1)})</p>
                    </div>
                ))}
            </div>
            
            {selectedAgent && (
                <AgentDetailPanel agentId={selectedAgent} />
            )}
        </div>
    );
};
```

### 4.2 Event Timeline (apps/web/src/ui/event_timeline.tsx)

```typescript
export const EventTimeline: React.FC = () => {
    const [events, setEvents] = useState<ObservationEvent[]>([]);
    const [filterType, setFilterType] = useState<string>('');
    
    const filteredEvents = events.filter(e => 
        !filterType || e.type === filterType
    );
    
    return (
        <div className="event-timeline">
            <input
                type="text"
                placeholder="Filter by event type..."
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
            />
            
            <div className="events">
                {filteredEvents.map((event, idx) => (
                    <div key={idx} className="event-item">
                        <span className="tick">[{event.tick}]</span>
                        <span className="type">{event.type}</span>
                        <span className="details">{JSON.stringify(event.payload)}</span>
                    </div>
                ))}
            </div>
        </div>
    );
};
```

---

## 5. REPLAY VIEWER

### 5.1 Time-Travel Debugger (apps/web/src/replay/replay_viewer.tsx)

```typescript
export const ReplayViewer: React.FC = () => {
    const [currentTick, setCurrentTick] = useState(0);
    const [maxTick, setMaxTick] = useState(0);
    const [isPlaying, setIsPlaying] = useState(false);
    
    useEffect(() => {
        if (isPlaying) {
            const interval = setInterval(() => {
                setCurrentTick(t => {
                    const next = t + 1;
                    if (next > maxTick) {
                        setIsPlaying(false);
                        return t;
                    }
                    return next;
                });
            }, 50);  // 20 ticks/sec playback
            
            return () => clearInterval(interval);
        }
    }, [isPlaying, maxTick]);
    
    return (
        <div className="replay-viewer">
            <h2>Replay</h2>
            
            <input
                type="range"
                min={0}
                max={maxTick}
                value={currentTick}
                onChange={(e) => setCurrentTick(parseInt(e.target.value))}
            />
            
            <button onClick={() => setIsPlaying(!isPlaying)}>
                {isPlaying ? 'Pause' : 'Play'}
            </button>
            
            <p>Tick: {currentTick} / {maxTick}</p>
            
            <ReplayRenderer tick={currentTick} />
        </div>
    );
};
```

---

## 6. PERFORMANCE REQUIREMENTS

- Chunk LOD system for distant terrain
- Frustum culling (don't render outside camera view)
- 60 FPS target on 2560x1440 with 16 visible chunks
- Memory limit: <500MB GPU VRAM

---

## 7. HASH VERIFICATION

### 7.1 Render Packet Hashing (apps/web/src/renderer/render_hash.ts)

```typescript
export function hashRenderPacket(packet: RenderPacket): string {
    const json = JSON.stringify(packet);
    const hash = blake3(json);
    return hash;
}

// Verify server's hash matches our render
if (packet.hash !== hashRenderPacket(packet)) {
    throw new Error('Render packet hash mismatch');
}
```

---

## 8. TEST SUITE

**TEST-WEBGPU-001**: Renderer initializes without errors  
**TEST-CHUNK-ENCODING-001**: Chunk RLE encoding/decoding round-trip  
**TEST-RENDER-PERFORMANCE-001**: 60 FPS maintained  
**TEST-HASH-STABILITY-001**: Render packet hash stable  

---

## 9. SUCCESS CRITERIA

- [ ] WebGPU renderer initialized
- [ ] Terrain renders correctly
- [ ] Agents render at correct positions
- [ ] Chunk streaming working (<100ms per chunk)
- [ ] State inspector functional
- [ ] Event timeline populated
- [ ] Replay viewer working
- [ ] 60 FPS achieved on target hardware
- [ ] All tests passing
- [ ] No regression from Phase 7

---

## 10. FORBIDDEN ACTIONS

- No WebGL (WebGPU only)
- Cannot skip render packet hashing
- No LOD that changes game outcomes
- Cannot render outside authority

---

## END OF PLAN

**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_8_RENDERING  
**Timestamp:** 2026-01-11
