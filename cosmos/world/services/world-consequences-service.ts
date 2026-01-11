import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { WorldService, WorldTimeState } from './WorldService';
import { EmbodimentWorldService, EmbodimentState } from './EmbodimentWorldService';
import { EventEmitter } from 'events';

import { WorldConsequencesService } from '../domains/cosmos/world/services/world-consequences-service';
import { PhysWorld } from '../physworld/index';

export interface Consequence {
  consequenceId: string;
  type: 'BIOLOGICAL' | 'PSYCHOLOGICAL' | 'MEMORY' | 'RELATIONAL' | 'SKILL' | 'IDENTITY';
  source: 'WORLD_TIME' | 'ENVIRONMENT' | 'LOCATION' | 'EVENT' | 'EXPOSURE';
  agentId: string;

  // Consequence properties
  description: string;
  magnitude: number; // 0-1 scale
  duration: number; // minutes, 0 for permanent
  startTime: Date;

  // Effects
  effects: {
    biological?: Record<string, number>;
    psychological?: Record<string, number>;
    memory?: {
      formationChance: number;
      emotionalWeight: number;
    };
    identity?: {
      traitShift: string;
      direction: 'increase' | 'decrease';
      amount: number;
    };
  };

  // Tracking
  isApplied: boolean;
  isPermanent: boolean;
  decayRate: number; // per minute
}

export interface ExposureConsequence {
  agentId: string;
  exposureType: string;
  exposureDuration: number; // minutes
  exposureIntensity: number;

  // Cumulative effects
  accumulatedStress: number;
  adaptationLevel: number;
  lastExposure: Date;

  // Long-term consequences
  developedTraits: string[];
  developedAversions: string[];
  resilienceFactors: string[];
}

export class WorldConsequencesService extends EventEmitter {
  private activeConsequences: Map<string, Consequence[]> = new Map();
  private exposureHistory: Map<string, ExposureConsequence[]> = new Map();
  private consequenceTimer: NodeJS.Timeout | null = null;

  constructor(
    private db: Pool,
    private worldService: WorldService,
    private embodimentService: EmbodimentWorldService
  ) {
    super();
  }

  /**
   * Initialize the consequences system
   */
  async initialize(): Promise<void> {
    try {
      // Load existing active consequences
      await this.loadActiveConsequences();

      // Start consequence processing loop
      this.startConsequenceProcessing();

      logger.info('WorldConsequencesService initialized');
    } catch (error) {
      logger.error('Failed to initialize WorldConsequencesService:', error);
      throw error;
    }
  }

  /**
   * Process time-based consequences
   */
  async processTimeConsequences(worldTimeState: WorldTimeState): Promise<void> {
    try {
      // Get all agents
      const agentsResult = await this.db.query(
        'SELECT agent_id FROM agent_world_positions'
      );

      for (const agent of agentsResult.rows) {
        const agentId = agent.agent_id;

        // Circadian consequences
        await this.processCircadianConsequences(agentId, worldTimeState);

        // Time-based decay
        await this.processTimeDecay(agentId, worldTimeState);

        // Long-term exposure effects
        await this.processLongTermExposure(agentId, worldTimeState);
      }

    } catch (error) {
      logger.error('Failed to process time consequences:', error);
    }
  }

  /**
   * Process environmental consequences
   */
  async processEnvironmentalConsequences(
    agentId: string,
    locationId: string,
    environmentalState: any
  ): Promise<void> {
    try {
      // Calculate exposure consequences
      const consequences = this.calculateEnvironmentalConsequences(
        agentId,
        environmentalState
      );

      // Apply immediate effects
      for (const consequence of consequences) {
        await this.applyConsequence(consequence);
      }

      // Track exposure for long-term effects
      await this.trackExposure(agentId, environmentalState);

    } catch (error) {
      logger.error(`Failed to process environmental consequences for ${agentId}:`, error);
    }
  }

  /**
   * Process location-based consequences
   */
  async processLocationConsequences(
    agentId: string,
    locationId: string,
    duration: number
  ): Promise<void> {
    try {
      // Get location details
      const locationResult = await this.db.query(`
        SELECT type, base_comfort, base_safety, base_privacy
        FROM world_locations
        WHERE location_id = $1
      `, [locationId]);

      if (locationResult.rows.length === 0) return;

      const location = locationResult.rows[0];

      // Calculate consequences based on location type and duration
      const consequences = this.calculateLocationConsequences(
        agentId,
        location,
        duration
      );

      // Apply consequences
      for (const consequence of consequences) {
        await this.applyConsequence(consequence);
      }

    } catch (error) {
      logger.error(`Failed to process location consequences for ${agentId}:`, error);
    }
  }

  /**
   * Process circadian consequences
   */
  private async processCircadianConsequences(
    agentId: string,
    worldTimeState: WorldTimeState
  ): Promise<void> {
    try {
      const circadianEffects = worldTimeState.circadianInfluence;

      if (Object.keys(circadianEffects).length === 0) return;

      // Create circadian consequence
      const consequence: Consequence = {
        consequenceId: `circ_${Date.now()}_${agentId}`,
        type: 'BIOLOGICAL',
        source: 'WORLD_TIME',
        agentId,
        description: `Circadian effects during ${worldTimeState.dayPhase}`,
        magnitude: 0.5,
        duration: 60, // 1 hour
        startTime: new Date(),
        effects: {
          biological: circadianEffects
        },
        isApplied: false,
        isPermanent: false,
        decayRate: 0.02
      };

      await this.applyConsequence(consequence);

    } catch (error) {
      logger.error(`Failed to process circadian consequences for ${agentId}:`, error);
    }
  }

  /**
   * Process time-based decay of consequences
   */
  private async processTimeDecay(
    agentId: string,
    worldTimeState: WorldTimeState
  ): Promise<void> {
    try {
      const agentConsequences = this.activeConsequences.get(agentId) || [];
      const updatedConsequences: Consequence[] = [];

      for (const consequence of agentConsequences) {
        if (consequence.isPermanent || consequence.duration === 0) {
          updatedConsequences.push(consequence);
          continue;
        }

        // Calculate decay
        const elapsedMinutes = (Date.now() - consequence.startTime.getTime()) / 60000;

        if (elapsedMinutes >= consequence.duration) {
          // Consequence expired
          await this.removeConsequence(consequence.consequenceId);
          continue;
        }

        // Apply decay
        consequence.magnitude = Math.max(0,
          consequence.magnitude - (consequence.decayRate * 1)); // 1 minute tick

        if (consequence.magnitude > 0.01) {
          updatedConsequences.push(consequence);
        } else {
          await this.removeConsequence(consequence.consequenceId);
        }
      }

      this.activeConsequences.set(agentId, updatedConsequences);

    } catch (error) {
      logger.error(`Failed to process time decay for ${agentId}:`, error);
    }
  }

  /**
   * Process long-term exposure effects
   */
  private async processLongTermExposure(
    agentId: string,
    worldTimeState: WorldTimeState
  ): Promise<void> {
    try {
      const exposureHistory = this.exposureHistory.get(agentId) || [];

      for (const exposure of exposureHistory) {
        // Check for significant exposure duration
        if (exposure.exposureDuration > 1440) { // More than 24 hours
          await this.processChronicExposure(agentId, exposure);
        }
      }

    } catch (error) {
      logger.error(`Failed to process long-term exposure for ${agentId}:`, error);
    }
  }

  /**
   * Calculate environmental consequences
   */
  private calculateEnvironmentalConsequences(
    agentId: string,
    environmentalState: any
  ): Consequence[] {
    const consequences: Consequence[] = [];

    // Stress from extreme conditions
    if (environmentalState.intensity > 0.7) {
      consequences.push({
        consequenceId: `env_${Date.now()}_${agentId}_${ChaosSys.getInstance().next()}`,
        type: 'PSYCHOLOGICAL',
        source: 'ENVIRONMENT',
        agentId,
        description: `Stress from ${environmentalState.type}`,
        magnitude: environmentalState.intensity * 0.6,
        duration: 30,
        startTime: new Date(),
        effects: {
          psychological: {
            stress: environmentalState.intensity * 0.4,
            anxiety: environmentalState.intensity * 0.3
          }
        },
        isApplied: false,
        isPermanent: false,
        decayRate: 0.05
      });
    }

    // Memory formation from significant events
    if (environmentalState.intensity > 0.6) {
      consequences.push({
        consequenceId: `mem_${Date.now()}_${agentId}_${ChaosSys.getInstance().next()}`,
        type: 'MEMORY',
        source: 'ENVIRONMENT',
        agentId,
        description: `Memory of ${environmentalState.type}`,
        magnitude: environmentalState.intensity,
        duration: 0, // Permanent memory
        startTime: new Date(),
        effects: {
          memory: {
            formationChance: environmentalState.intensity,
            emotionalWeight: environmentalState.intensity * 0.8
          }
        },
        isApplied: false,
        isPermanent: true,
        decayRate: 0
      });
    }

    return consequences;
  }

  /**
   * Calculate location-based consequences
   */
  private calculateLocationConsequences(
    agentId: string,
    location: any,
    duration: number
  ): Consequence[] {
    const consequences: Consequence[] = [];

    // Long stays in private spaces increase introversion
    if (location.type === 'PRIVATE' && duration > 120) {
      consequences.push({
        consequenceId: `loc_${Date.now()}_${agentId}_${ChaosSys.getInstance().next()}`,
        type: 'IDENTITY',
        source: 'LOCATION',
        agentId,
        description: 'Extended time in private space',
        magnitude: Math.min(1, duration / 480), // Scale over 8 hours
        duration: 0, // Permanent identity shift
        startTime: new Date(),
        effects: {
          identity: {
            traitShift: 'introversion',
            direction: 'increase',
            amount: 0.1 * Math.min(1, duration / 480)
          }
        },
        isApplied: false,
        isPermanent: true,
        decayRate: 0
      });
    }

    // Uncomfortable locations increase stress
    if (location.base_comfort < 0.5 && duration > 30) {
      consequences.push({
        consequenceId: `comf_${Date.now()}_${agentId}_${ChaosSys.getInstance().next()}`,
        type: 'PSYCHOLOGICAL',
        source: 'LOCATION',
        agentId,
        description: 'Discomfort from environment',
        magnitude: (0.5 - location.base_comfort) * 2,
        duration: 60,
        startTime: new Date(),
        effects: {
          psychological: {
            stress: (0.5 - location.base_comfort) * 0.5,
            irritability: (0.5 - location.base_comfort) * 0.3
          }
        },
        isApplied: false,
        isPermanent: false,
        decayRate: 0.03
      });
    }

    return consequences;
  }

  /**
   * Apply a consequence to the agent
   */
  private async applyConsequence(consequence: Consequence): Promise<void> {
    try {
      if (consequence.isApplied) return;

      // Store consequence
      await this.db.query(`
        INSERT INTO world_consequences (
          consequence_id, type, source, agent_id, description,
          magnitude, duration, start_time, effects, is_permanent, decay_rate
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
      `, [
        consequence.consequenceId,
        consequence.type,
        consequence.source,
        consequence.agentId,
        consequence.description,
        consequence.magnitude,
        consequence.duration,
        consequence.startTime,
        JSON.stringify(consequence.effects),
        consequence.isPermanent,
        consequence.decayRate
      ]);

      // Apply effects based on type
      if (consequence.effects.biological) {
        await this.applyBiologicalEffects(consequence);
      }

      if (consequence.effects.psychological) {
        await this.applyPsychologicalEffects(consequence);
      }

      if (consequence.effects.identity) {
        await this.applyIdentityEffects(consequence);
      }

      // Track active consequence
      if (!this.activeConsequences.has(consequence.agentId)) {
        this.activeConsequences.set(consequence.agentId, []);
      }
      this.activeConsequences.get(consequence.agentId)!.push(consequence);

      consequence.isApplied = true;

      this.emit('consequenceApplied', { agentId: consequence.agentId, consequence });

    } catch (error) {
      logger.error('Failed to apply consequence:', error);
    }
  }

  /**
   * Apply biological effects
   */
  private async applyBiologicalEffects(consequence: Consequence): Promise<void> {
    const embodimentState = this.embodimentService.getEmbodimentState(consequence.agentId);
    if (!embodimentState) return;

    const effects = consequence.effects.biological!;

    // Update embodiment state
    if (effects.energy) {
      embodimentState.energyLevel = Math.max(0, Math.min(100,
        embodimentState.energyLevel + effects.energy * 10));
    }

    if (effects.stress) {
      embodimentState.stressLevel = Math.max(0, Math.min(1,
        embodimentState.stressLevel + effects.stress * 0.1));
    }

    if (effects.sleep_drive) {
      // This would integrate with sleep system
      this.emit('sleepDriveChanged', {
        agentId: consequence.agentId,
        change: effects.sleep_drive
      });
    }
  }

  /**
   * Apply psychological effects
   */
  private async applyPsychologicalEffects(consequence: Consequence): Promise<void> {
    const effects = consequence.effects.psychological!;

    // Store psychological state changes
    await this.db.query(`
      INSERT INTO psychological_state_changes (
        agent_id, change_type, magnitude, source, timestamp
      ) VALUES ($1, $2, $3, $4, $5)
    `, [
      consequence.agentId,
      'consequence',
      consequence.magnitude,
      consequence.source,
      new Date()
    ]);

    // Notify emotion service
    this.emit('psychologicalEffectsApplied', {
      agentId: consequence.agentId,
      effects,
      source: consequence.source
    });
  }

  /**
   * Apply identity effects
   */
  private async applyIdentityEffects(consequence: Consequence): Promise<void> {
    const identityEffect = consequence.effects.identity!;

    // Store identity shift
    await this.db.query(`
      INSERT INTO identity_shifts (
        agent_id, trait, direction, magnitude, shift_time, context
      ) VALUES ($1, $2, $3, $4, $5, $6)
    `, [
      consequence.agentId,
      identityEffect.traitShift,
      identityEffect.direction,
      identityEffect.amount,
      consequence.startTime,
      JSON.stringify({
        source: consequence.source,
        description: consequence.description
      })
    ]);

    this.emit('identityShifted', {
      agentId: consequence.agentId,
      trait: identityEffect.traitShift,
      direction: identityEffect.direction,
      amount: identityEffect.amount
    });
  }

  /**
   * Track exposure for long-term effects
   */
  private async trackExposure(agentId: string, environmentalState: any): Promise<void> {
    try {
      const exposureType = environmentalState.type;

      // Get or create exposure history
      if (!this.exposureHistory.has(agentId)) {
        this.exposureHistory.set(agentId, []);
      }

      const history = this.exposureHistory.get(agentId)!;
      let exposure = history.find(e => e.exposureType === exposureType);

      if (!exposure) {
        exposure = {
          agentId,
          exposureType,
          exposureDuration: 0,
          exposureIntensity: 0,
          accumulatedStress: 0,
          adaptationLevel: 0,
          lastExposure: new Date(),
          developedTraits: [],
          developedAversions: [],
          resilienceFactors: []
        };
        history.push(exposure);
      }

      // Update exposure
      exposure.exposureDuration += 1; // 1 minute
      exposure.exposureIntensity = (exposure.exposureIntensity * 0.9) + (environmentalState.intensity * 0.1);
      exposure.lastExposure = new Date();

      // Calculate accumulated stress
      if (environmentalState.intensity > 0.5) {
        exposure.accumulatedStress += environmentalState.intensity * 0.1;
      }

      // Check for adaptation
      if (exposure.exposureDuration > 100) { // 100+ minutes
        exposure.adaptationLevel = Math.min(1, exposure.exposureDuration / 1000);
      }

      // Store in database
      await this.db.query(`
        INSERT INTO exposure_tracking (
          agent_id, exposure_type, duration, intensity, 
          accumulated_stress, adaptation_level, last_exposure
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (agent_id, exposure_type) DO UPDATE SET
          duration = EXCLUDED.duration,
          intensity = EXCLUDED.intensity,
          accumulated_stress = EXCLUDED.accumulated_stress,
          adaptation_level = EXCLUDED.adaptation_level,
          last_exposure = EXCLUDED.last_exposure
      `, [
        agentId,
        exposureType,
        exposure.exposureDuration,
        exposure.exposureIntensity,
        exposure.accumulatedStress,
        exposure.adaptationLevel,
        exposure.lastExposure
      ]);

    } catch (error) {
      logger.error(`Failed to track exposure for ${agentId}:`, error);
    }
  }

  /**
   * Process chronic exposure effects
   */
  private async processChronicExposure(
    agentId: string,
    exposure: ExposureConsequence
  ): Promise<void> {
    try {
      // Chronic stress effects
      if (exposure.accumulatedStress > 10) {
        const consequence: Consequence = {
          consequenceId: `chronic_${Date.now()}_${agentId}`,
          type: 'PSYCHOLOGICAL',
          source: 'EXPOSURE',
          agentId,
          description: `Chronic stress from ${exposure.exposureType}`,
          magnitude: Math.min(1, exposure.accumulatedStress / 20),
          duration: 1440, // 24 hours
          startTime: new Date(),
          effects: {
            psychological: {
              chronic_stress: 0.3,
              fatigue: 0.4,
              anxiety: 0.2
            },
            identity: {
              traitShift: 'stress_sensitivity',
              direction: 'increase',
              amount: 0.1
            }
          },
          isApplied: false,
          isPermanent: false,
          decayRate: 0.001
        };

        await this.applyConsequence(consequence);
      }

      // Adaptation benefits
      if (exposure.adaptationLevel > 0.8) {
        exposure.resilienceFactors.push(`${exposure.exposureType}_resistance`);

        const consequence: Consequence = {
          consequenceId: `adapt_${Date.now()}_${agentId}`,
          type: 'SKILL',
          source: 'EXPOSURE',
          agentId,
          description: `Adapted to ${exposure.exposureType}`,
          magnitude: exposure.adaptationLevel,
          duration: 0, // Permanent
          startTime: new Date(),
          effects: {
            biological: {
              resilience: exposure.adaptationLevel * 0.5
            }
          },
          isApplied: false,
          isPermanent: true,
          decayRate: 0
        };

        await this.applyConsequence(consequence);
      }

    } catch (error) {
      logger.error(`Failed to process chronic exposure for ${agentId}:`, error);
    }
  }

  /**
   * Remove expired consequence
   */
  private async removeConsequence(consequenceId: string): Promise<void> {
    try {
      await this.db.query(`
        DELETE FROM world_consequences
        WHERE consequence_id = $1
      `, [consequenceId]);

      // Remove from active tracking
      for (const [agentId, consequences] of this.activeConsequences.entries()) {
        const filtered = consequences.filter(c => c.consequenceId !== consequenceId);
        this.activeConsequences.set(agentId, filtered);
      }

    } catch (error) {
      logger.error('Failed to remove consequence:', error);
    }
  }

  /**
   * Start consequence processing timer
   */
  private startConsequenceProcessing(): void {
    // DISABLED: setInterval-driven consequence processing moved to PhysWorld tick chain
    console.log('✅ WorldConsequencesService: setInterval consequence processing disabled - use PhysWorld tick integration');
    // Consequence processing now handled by tickConsequenceProcessing() called from PhysWorld
  }

  // NEW: PhysWorld tick integration method for consequence processing
  async tickConsequenceProcessing(): Promise<void> {
    try {
      // Process decay for all agents every minute
      const tickCount = Date.now() / 1000;
      if (tickCount % 60 === 0) { // Every 60 seconds = 1 minute
        for (const agentId of this.activeConsequences.keys()) {
          await this.processTimeDecay(agentId, await this.worldService.getWorldTimeState());
        }
      }
    } catch (error) {
      logger.error('Consequence processing error:', error);
    }
  }

  /**
   * Load active consequences from database
   */
  private async loadActiveConsequences(): Promise<void> {
    try {
      const result = await this.db.query(`
        SELECT * FROM world_consequences
        WHERE (duration = 0 OR start_time + INTERVAL '1 minute' * duration > NOW())
        AND is_active = true
      `);

      for (const row of result.rows) {
        const consequence: Consequence = {
          consequenceId: row.consequence_id,
          type: row.type,
          source: row.source,
          agentId: row.agent_id,
          description: row.description,
          magnitude: row.magnitude,
          duration: row.duration,
          startTime: row.start_time,
          effects: JSON.parse(row.effects || '{}'),
          isApplied: row.is_applied,
          isPermanent: row.is_permanent,
          decayRate: row.decay_rate
        };

        if (!this.activeConsequences.has(consequence.agentId)) {
          this.activeConsequences.set(consequence.agentId, []);
        }
        this.activeConsequences.get(consequence.agentId)!.push(consequence);
      }

    } catch (error) {
      logger.error('Failed to load active consequences:', error);
    }
  }

  /**
   * Get active consequences for agent
   */
  getActiveConsequences(agentId: string): Consequence[] {
    return this.activeConsequences.get(agentId) || [];
  }

  /**
   * Get exposure history for agent
   */
  getExposureHistory(agentId: string): ExposureConsequence[] {
    return this.exposureHistory.get(agentId) || [];
  }

  /**
   * Stop the consequences service
   */
  async stop(): Promise<void> {
    if (this.consequenceTimer) {
      clearInterval(this.consequenceTimer);
      this.consequenceTimer = null;
    }
    logger.info('WorldConsequencesService stopped');
  }
}
