
/**
 * STREAM PHYSICS
 * 
 * Realistic water flow, erosion, and fluid dynamics
 * Particle-based water simulation with environmental interactions
 */

import { ChaosSys } from '../chaos/chaos-sys';

export interface WaterParticle {
  id: string;
  position: { x: number; y: number; z: number };
  velocity: { x: number; y: number; z: number };
  temperature: number; // Celsius
  clarity: number; // 0-1, turbidity
  nutrients: number; // 0-1, dissolved nutrients
}

export interface StreamState {
  flow_rate: number; // m³/s
  depth: number; // meters
  width: number; // meters
  temperature: number; // average water temp
  ph_level: number; // 0-14
  oxygen_level: number; // mg/L dissolved oxygen
  pollution: number; // 0-1, contamination level
}

export interface EnvironmentalFactors {
  ambient_temperature: number; // Celsius
  humidity: number; // 0-1
  wind_speed: number; // m/s
  precipitation: number; // mm/hour
  solar_radiation: number; // W/m²
}

export class StreamPhysics {
  private particles: Map<string, WaterParticle>;
  private state: StreamState;
  private environment: EnvironmentalFactors;
  private gridSize: number;
  private viscosity: number;
  private chaosSys: ChaosSys;

  constructor(width: number = 10, depth: number = 2) {
    this.chaosSys = ChaosSys.getInstance();
    this.particles = new Map();
    this.gridSize = 0.1; // 10cm grid cells
    this.viscosity = 0.001; // Water viscosity at 20°C

    this.state = {
      flow_rate: 2.5, // 2.5 m³/s
      depth,
      width,
      temperature: 15.0,
      ph_level: 7.2,
      oxygen_level: 8.5,
      pollution: 0.1
    };

    this.environment = {
      ambient_temperature: 20.0,
      humidity: 0.6,
      wind_speed: 2.0,
      precipitation: 0,
      solar_radiation: 500
    };

    this._initializeParticles();
  }

  update(dtSeconds: number): void {
    // Update environmental conditions
    this._updateEnvironment(dtSeconds);

    // Update water temperature
    this._updateWaterTemperature(dtSeconds);

    // Simulate fluid dynamics
    this._updateFluidDynamics(dtSeconds);

    // Process erosion and deposition
    this._processErosion(dtSeconds);

    // Update water chemistry
    this._updateWaterChemistry(dtSeconds);

    // Handle precipitation
    this._processPrecipitation(dtSeconds);

    // Clean up old particles
    this._cleanupParticles();
  }

  addParticle(particle: Partial<WaterParticle>): string {
    const id = `particle_${Date.now()}_${this.chaosSys.generateRandomString(9)}`;
    const fullParticle: WaterParticle = {
      id,
      position: particle.position || { x: 0, y: 0, z: 0 },
      velocity: particle.velocity || { x: 0, y: 0, z: 0 },
      temperature: particle.temperature || this.state.temperature,
      clarity: particle.clarity || 0.9,
      nutrients: particle.nutrients || 0.5
    };

    this.particles.set(id, fullParticle);
    return id;
  }

  getStreamState(): StreamState {
    return { ...this.state };
  }

  getFlowVelocity(position: { x: number; y: number }): { x: number; y: number } {
    // Simplified laminar flow profile
    const depthRatio = position.y / this.state.depth;
    const maxVelocity = this.state.flow_rate / (this.state.width * this.state.depth);

    // Parabolic velocity profile (faster in center)
    const velocityProfile = 4 * depthRatio * (1 - depthRatio);

    return {
      x: maxVelocity * velocityProfile,
      y: 0
    };
  }

  getWaterQuality(): {
    clarity: number;
    temperature: number;
    ph: number;
    oxygen: number;
    pollution: number;
  } {
    const avgClarity = this._calculateAverageClarity();

    return {
      clarity: avgClarity,
      temperature: this.state.temperature,
      ph: this.state.ph_level,
      oxygen: this.state.oxygen_level,
      pollution: this.state.pollution
    };
  }

  interactWithObject(object: {
    position: { x: number; y: number };
    size: number;
    type: 'rock' | 'debris' | 'organism';
  }): {
    drag_force: number;
    turbulence: number;
    erosion_impact: number;
  } {
    const flowVelocity = this.getFlowVelocity(object.position);
    const relativeVelocity = Math.sqrt(flowVelocity.x ** 2 + flowVelocity.y ** 2);

    // Calculate drag force
    const dragCoefficient = object.type === 'rock' ? 0.8 :
      object.type === 'debris' ? 1.2 : 0.6;
    const dragForce = 0.5 * dragCoefficient * this.viscosity * relativeVelocity ** 2 * object.size;

    // Calculate turbulence
    const reynoldsNumber = (relativeVelocity * object.size) / this.viscosity;
    const turbulence = reynoldsNumber > 2000 ? Math.min(1, (reynoldsNumber - 2000) / 8000) : 0;

    // Erosion impact
    const erosionImpact = object.type === 'rock' ? dragForce * 0.001 : 0;

    return {
      drag_force: dragForce,
      turbulence,
      erosion_impact: erosionImpact
    };
  }

  private _initializeParticles(): void {
    const particleCount = Math.floor(this.state.width * this.state.depth / this.gridSize);

    for (let i = 0; i < particleCount; i++) {
      const x = (this.chaosSys.boundedRandom(0, this.state.width) - this.state.width / 2);
      const y = this.chaosSys.boundedRandom(0, this.state.depth);
      const z = (this.chaosSys.boundedRandom(0, 0.5) - 0.25);

      this.addParticle({
        position: { x, y, z },
        velocity: { x: 1.0, y: 0, z: 0 }
      });
    }
  }

  private _updateEnvironment(dtSeconds: number): void {
    // Simple environmental variation
    const time = Date.now() / 1000;

    this.environment.ambient_temperature = 20 + Math.sin(time / 100) * 5;
    this.environment.humidity = 0.6 + Math.sin(time / 200) * 0.2;
    this.environment.wind_speed = 2 + Math.sin(time / 150) * 1;
    this.environment.solar_radiation = 500 + Math.sin(time / 80) * 200;
  }

  private _updateWaterTemperature(dtSeconds: number): void {
    // Heat exchange with environment
    const tempDiff = this.environment.ambient_temperature - this.state.temperature;
    const heatTransferRate = 0.01; // Heat transfer coefficient

    this.state.temperature += tempDiff * heatTransferRate * dtSeconds;

    // Solar heating
    const solarHeating = this.environment.solar_radiation * 0.00001;
    this.state.temperature += solarHeating * dtSeconds;

    // Evaporative cooling
    if (this.environment.humidity < 0.8) {
      const evaporativeCooling = 0.001 * (1 - this.environment.humidity);
      this.state.temperature -= evaporativeCooling * dtSeconds;
    }
  }

  private _updateFluidDynamics(dtSeconds: number): void {
    for (const particle of this.particles.values()) {
      // Get flow velocity at particle position
      const flowVelocity = this.getFlowVelocity(particle.position);

      // Apply drag force
      const dragForce = {
        x: (flowVelocity.x - particle.velocity.x) * 0.1,
        y: (flowVelocity.y - particle.velocity.y) * 0.1,
        z: -particle.velocity.z * 0.05 // Settling
      };

      // Update velocity
      particle.velocity.x += dragForce.x * dtSeconds;
      particle.velocity.y += dragForce.y * dtSeconds;
      particle.velocity.z += dragForce.z * dtSeconds;

      // Update position
      particle.position.x += particle.velocity.x * dtSeconds;
      particle.position.y += particle.velocity.y * dtSeconds;
      particle.position.z += particle.velocity.z * dtSeconds;

      // Boundary conditions
      if (particle.position.x > this.state.width / 2) {
        particle.position.x = -this.state.width / 2;
      }
      if (particle.position.y > this.state.depth) {
        particle.position.y = 0;
        particle.velocity.y = Math.abs(particle.velocity.y) * 0.5; // Bounce with energy loss
      }
      if (particle.position.z < -this.state.depth / 2) {
        particle.position.z = -this.state.depth / 2;
        particle.velocity.z = 0; // Settled
      }
    }
  }

  private _processErosion(dtSeconds: number): void {
    // Simplified erosion model
    let totalErosion = 0;

    for (const particle of this.particles.values()) {
      const velocity = Math.sqrt(particle.velocity.x ** 2 + particle.velocity.y ** 2);
      const erosionRate = velocity * 0.00001; // Erosion proportional to velocity
      totalErosion += erosionRate;
    }

    // Erosion affects stream depth and width
    this.state.depth += totalErosion * dtSeconds * 0.1;
    this.state.width += totalErosion * dtSeconds * 0.05;
  }

  private _updateWaterChemistry(dtSeconds: number): void {
    // Oxygen exchange with atmosphere
    const oxygenDiffusion = (8.5 - this.state.oxygen_level) * 0.001;
    this.state.oxygen_level += oxygenDiffusion * dtSeconds;

    // pH changes from pollution
    if (this.state.pollution > 0.3) {
      this.state.ph_level -= 0.001 * dtSeconds; // Acidification
    }

    // Natural pH buffering
    const neutralPh = 7.0;
    this.state.ph_level += (neutralPh - this.state.ph_level) * 0.0001 * dtSeconds;

    // Pollution decay
    this.state.pollution *= (1 - 0.0001 * dtSeconds);
  }

  private _processPrecipitation(dtSeconds: number): void {
    if (this.environment.precipitation > 0) {
      // Add water from rain
      const waterInput = this.environment.precipitation * this.state.width * 0.001;
      this.state.flow_rate += waterInput;

      // Dilution effect on pollution
      const dilutionFactor = 1 / (1 + waterInput * 0.1);
      this.state.pollution *= dilutionFactor;

      // Temperature mixing
      const rainTemp = this.environment.ambient_temperature - 2; // Rain is cooler
      this.state.temperature = (this.state.temperature * 0.9 + rainTemp * 0.1);
    }
  }

  private _calculateAverageClarity(): number {
    if (this.particles.size === 0) return 0.9;

    const totalClarity = Array.from(this.particles.values())
      .reduce((sum, p) => sum + p.clarity, 0);

    return totalClarity / this.particles.size;
  }

  private _cleanupParticles(): void {
    // Remove particles that have left the system
    for (const [id, particle] of this.particles) {
      if (particle.position.x < -this.state.width * 2 ||
        particle.position.x > this.state.width * 2) {
        this.particles.delete(id);
      }
    }
  }

  getParticleCount(): number {
    return this.particles.size;
  }

  setEnvironmentalFactors(factors: Partial<EnvironmentalFactors>): void {
    Object.assign(this.environment, factors);
  }

  resetStream(): void {
    this.particles.clear();
    this.state = {
      flow_rate: 2.5,
      depth: 2,
      width: 10,
      temperature: 15.0,
      ph_level: 7.2,
      oxygen_level: 8.5,
      pollution: 0.1
    };
    this._initializeParticles();
  }
}
