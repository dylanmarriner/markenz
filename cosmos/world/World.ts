
/**
 * World - Main simulation container
 * 
 * Owns the authoritative WorldClock and manages deterministic update order:
 * CLOCK → PHYSICS → AGENTS → EVENTS
 */

import { WorldClock } from './WorldClock';
import { PhysWorld } from '../physworld/phys-world';
import { AgentRegistry } from '../modules/agent/AgentRegistry';

export class World {
  private clock: WorldClock;
  private physics: PhysWorld;
  private agents: AgentRegistry;
  private isRunning: boolean;

  constructor() {
    this.clock = new WorldClock(100); // 100ms fixed timestep
    this.physics = new PhysWorld();
    this.agents = new AgentRegistry(this.physics);
    this.isRunning = false;
  }

  /**
   * Start the simulation loop
   */
  start(): void {
    if (this.isRunning) {
      throw new Error('World is already running');
    }
    this.isRunning = true;
  }

  /**
   * Stop the simulation loop
   */
  stop(): void {
    this.isRunning = false;
  }

  /**
   * Execute one deterministic update step
   * Fixed order: CLOCK → PHYSICS → AGENTS → EVENTS
   */
  step(): void {
    // 1) CLOCK - Advance time
    this.clock.tick();
    const dt = this.clock.getDt();

    // 2) PHYSICS - Step physics simulation
    this.physics.update(dt);

    // 3) AGENTS - Tick all agents (no autonomy, just physics updates)
    this.agents.tickAllAgents();

    // 4) EVENTS - Dispatch pending events
    this.eventDispatch();
  }

  /**
   * Run simulation for specified duration (headless)
   */
  runHeadless(durationMs: number): void {
    const targetTicks = Math.ceil(durationMs / this.clock.getDt());
    
    for (let i = 0; i < targetTicks; i++) {
      this.step();
    }
  }

  /**
   * Get world clock reference
   */
  getClock(): WorldClock {
    return this.clock;
  }

  /**
   * Get physics world reference
   */
  getPhysics(): PhysWorld {
    return this.physics;
  }

  /**
   * Get agent registry reference
   */
  getAgents(): AgentRegistry {
    return this.agents;
  }

  /**
   * Check if world is running
   */
  isWorldRunning(): boolean {
    return this.isRunning;
  }

  /**
   * Get deterministic world state
   */
  getState(): {
    clock: { tickCount: number; simTime: number; dt: number };
    isRunning: boolean;
  } {
    return {
      clock: this.clock.getState(),
      isRunning: this.isRunning
    };
  }

  /**
   * Event dispatch placeholder
   */
  private eventDispatch(): void {
    // Event dispatching will be implemented in later phases
    // For now, this maintains the deterministic update order
  }
}
