
/**
 * WorldClock - Single authoritative simulation clock
 * 
 * Fixed timestep deterministic clock with no drift
 * No wall-clock dependency beyond stepping
 */

export class WorldClock {
  private tickCount: number;
  private simTime: number;
  private readonly dt: number;
  private readonly dtMs: number;

  constructor(dt: number = 100) { // dt in milliseconds
    this.dt = dt;
    this.dtMs = dt;
    this.tickCount = 0;
    this.simTime = 0;
  }

  /**
   * Advance clock by one fixed timestep
   */
  tick(): void {
    this.tickCount++;
    this.simTime += this.dtMs;
  }

  /**
   * Get current tick count (monotonic)
   */
  getTickCount(): number {
    return this.tickCount;
  }

  /**
   * Get current simulation time in milliseconds (monotonic)
   */
  getSimTime(): number {
    return this.simTime;
  }

  /**
   * Get fixed timestep in milliseconds
   */
  getDt(): number {
    return this.dt;
  }

  /**
   * Reset clock to initial state
   */
  reset(): void {
    this.tickCount = 0;
    this.simTime = 0;
  }

  /**
   * Get deterministic state snapshot
   */
  getState(): { tickCount: number; simTime: number; dt: number } {
    return {
      tickCount: this.tickCount,
      simTime: this.simTime,
      dt: this.dt
    };
  }

  /**
   * Restore from deterministic state snapshot
   */
  setState(state: { tickCount: number; simTime: number; dt: number }): void {
    this.tickCount = state.tickCount;
    this.simTime = state.simTime;
    if (state.dt !== this.dt) {
      throw new Error('Cannot restore clock with different timestep');
    }
  }
}
