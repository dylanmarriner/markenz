import { Pool } from 'pg';
import { logger } from '../../../../utils/logger';
import { EventEmitter } from 'events';

import { PhysWorld } from '../../../../physworld/index';

export interface WorldTimeState {
  worldTime: Date;
  dayNumber: number;
  hourOfDay: number;
  dayPhase: 'DAWN' | 'MORNING' | 'NOON' | 'AFTERNOON' | 'EVENING' | 'NIGHT' | 'MIDNIGHT';
  lightingModifier: number;
  temperatureModifier: number;
  circadianInfluence: Record<string, number>;
}

export interface LocationState {
  locationId: string;
  name: string;
  type: string;
  temperature: number;
  lighting: number;
  soundLevel: number;
  comfort: number;
  privacy: number;
  safety: number;
  socialDensity: number;
  environmentalStates: EnvironmentalState[];
}

export interface EnvironmentalState {
  stateId: string;
  type: 'WEATHER' | 'LIGHTING' | 'TEMPERATURE' | 'SOUND' | 'EVENT' | 'DANGER';
  intensity: number;
  effects: Record<string, number>;
  sensoryDescription: string;
  remainingMinutes: number;
}

export interface SensoryInput {
  inputId: string;
  modality: 'VISUAL' | 'AUDITORY' | 'TACTILE' | 'TEMPERATURE' | 'PROPRIOCEPTION';
  intensity: number;
  quality: string;
  description: string;
  source: string;
  locationId: string;
}

export class WorldService extends EventEmitter {
  private worldTime: Date;
  private timeScale: number = 1.0;
  private tickInterval: NodeJS.Timeout | null = null;
  private isRunning: boolean = false;

  constructor(private db: Pool) {
    super();
    this.worldTime = new Date();
  }

  /**
   * Initialize the world system
   */
  async initialize(): Promise<void> {
    try {
      // Ensure world config exists
      await this.db.query(`
        INSERT INTO world_config (id, world_name, world_version, current_world_time)
        VALUES (1, 'Gemini Digital Universe', '1.0.0', NOW())
        ON CONFLICT (id) DO NOTHING
      `);

      // Load current world time
      const result = await this.db.query(`
        SELECT current_world_time, time_scale FROM world_config WHERE id = 1
      `);

      if (result.rows.length > 0) {
        this.worldTime = result.rows[0].current_world_time;
        this.timeScale = result.rows[0].time_scale;
      }

      logger.info('WorldService initialized', {
        worldTime: this.worldTime,
        timeScale: this.timeScale
      });

    } catch (error) {
      logger.error('Failed to initialize WorldService:', error);
      throw error;
    }
  }

  /**
   * Start the world simulation
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      logger.warn('WorldService is already running');
      return;
    }

    this.isRunning = true;

    // DISABLED: setInterval-driven world time progression moved to PhysWorld tick chain
    console.log('✅ WorldService: setInterval world time progression disabled - use PhysWorld tick integration');
    // World time progression now handled by tickWorldTime() called from PhysWorld

    logger.info('WorldService started - time progression via PhysWorld tick');
    this.emit('started');
  }

  // NEW: PhysWorld tick integration method for world time progression
  async tickWorldTime(): Promise<void> {
    if (!this.isRunning) return;
    // Advance world time (1 minute per real second by default)
    await this.advanceWorldTime(1);
  }

  /**
   * Stop the world simulation
   */
  async stop(): Promise<void> {
    if (!this.isRunning) {
      return;
    }

    this.isRunning = false;

    if (this.tickInterval) {
      clearInterval(this.tickInterval);
      this.tickInterval = null;
    }

    logger.info('WorldService stopped');
    this.emit('stopped');
  }

  /**
   * Advance world time and apply consequences
   */
  async advanceWorldTime(minutes: number = 1): Promise<WorldTimeState> {
    try {
      // Update world time in database
      await this.db.query('SELECT advance_world_time($1)', [minutes]);

      // Get current world state
      const timeResult = await this.db.query(`
        SELECT * FROM world_time_progression 
        ORDER BY world_time DESC 
        LIMIT 1
      `);

      const worldState: WorldTimeState = {
        worldTime: timeResult.rows[0].world_time,
        dayNumber: timeResult.rows[0].day_number,
        hourOfDay: timeResult.rows[0].hour_of_day,
        dayPhase: timeResult.rows[0].day_phase,
        lightingModifier: timeResult.rows[0].lighting_modifier,
        temperatureModifier: timeResult.rows[0].temperature_modifier,
        circadianInfluence: timeResult.rows[0].circadian_influence
      };

      // Apply time-based effects
      await this.applyTimeBasedEffects(worldState);

      // Emit world time update
      this.emit('timeAdvanced', worldState);

      return worldState;

    } catch (error) {
      logger.error('Failed to advance world time:', error);
      throw error;
    }
  }

  /**
   * Get current world time state
   */
  async getWorldTimeState(): Promise<WorldTimeState> {
    const result = await this.db.query(`
      SELECT * FROM world_time_progression 
      ORDER BY world_time DESC 
      LIMIT 1
    `);

    if (result.rows.length === 0) {
      throw new Error('No world time state found');
    }

    return {
      worldTime: result.rows[0].world_time,
      dayNumber: result.rows[0].day_number,
      hourOfDay: result.rows[0].hour_of_day,
      dayPhase: result.rows[0].day_phase,
      lightingModifier: result.rows[0].lighting_modifier,
      temperatureModifier: result.rows[0].temperature_modifier,
      circadianInfluence: result.rows[0].circadian_influence
    };
  }

  /**
   * Get agent's current location and environmental state
   */
  async getAgentLocationState(agentId: string): Promise<LocationState | null> {
    const result = await this.db.query(`
      SELECT 
        wl.location_id,
        wl.name,
        wl.type,
        wl.base_temperature,
        wl.base_lighting,
        wl.base_sound_level,
        wl.base_comfort,
        wl.base_privacy,
        wl.base_safety,
        wl.capacity,
        COUNT(awp.agent_id) as current_occupancy
      FROM agent_world_positions awp
      JOIN world_locations wl ON awp.location_id = wl.location_id
      WHERE awp.agent_id = $1
      GROUP BY wl.location_id, wl.name, wl.type, wl.base_temperature, 
               wl.base_lighting, wl.base_sound_level, wl.base_comfort,
               wl.base_privacy, wl.base_safety, wl.capacity
    `, [agentId]);

    if (result.rows.length === 0) {
      return null;
    }

    const location = result.rows[0];
    const socialDensity = location.current_occupancy / location.capacity;

    // Get active environmental states
    const envStatesResult = await this.db.query(`
      SELECT state_id, state_type, intensity, effects, sensory_description,
             EXTRACT(EPOCH FROM (end_time - NOW())) / 60 as remaining_minutes
      FROM environmental_states
      WHERE location_id = $1 AND is_active = true
    `, [location.location_id]);

    const environmentalStates: EnvironmentalState[] = envStatesResult.rows.map(row => ({
      stateId: row.state_id,
      type: row.state_type,
      intensity: row.intensity,
      effects: row.effects,
      sensoryDescription: row.sensory_description,
      remainingMinutes: Math.max(0, Math.floor(row.remaining_minutes))
    }));

    return {
      locationId: location.location_id,
      name: location.name,
      type: location.type,
      temperature: location.base_temperature,
      lighting: location.base_lighting,
      soundLevel: location.base_sound_level,
      comfort: location.base_comfort,
      privacy: location.base_privacy,
      safety: location.base_safety,
      socialDensity,
      environmentalStates
    };
  }

  /**
   * Move agent to a new location
   */
  async moveAgent(agentId: string, targetLocationId: string, purpose?: string): Promise<void> {
    const client = await this.db.connect();

    try {
      await client.query('BEGIN');

      // Get current position
      const currentResult = await client.query(`
        SELECT location_id, coordinates
        FROM agent_world_positions
        WHERE agent_id = $1
      `, [agentId]);

      if (currentResult.rows.length === 0) {
        throw new Error(`Agent ${agentId} not found in world`);
      }

      const currentLocationId = currentResult.rows[0].location_id;

      // Record location history
      if (currentLocationId !== targetLocationId) {
        await client.query(`
          INSERT INTO agent_location_history (
            agent_id, location_id, arrival_time, purpose
          ) VALUES ($1, $2, NOW(), $3)
        `, [agentId, currentLocationId, purpose]);
      }

      // Update position
      await client.query(`
        UPDATE agent_world_positions SET
          location_id = $1,
          coordinates = '{"x": 0, "y": 0, "z": 0}',
          is_moving = false,
          target_location_id = NULL,
          last_activity = NOW(),
          activity_type = 'MOVING'
        WHERE agent_id = $2
      `, [targetLocationId, agentId]);

      await client.query('COMMIT');

      // Generate sensory input for location change
      await this.generateSensoryInput(
        targetLocationId,
        'PROPRIOCEPTION',
        0.6,
        `Agent ${agentId} has entered a new location`
      );

      this.emit('agentMoved', { agentId, fromLocationId: currentLocationId, toLocationId: targetLocationId });

    } catch (error) {
      await client.query('ROLLBACK');
      throw error;
    } finally {
      client.release();
    }
  }

  /**
   * Create environmental state at location
   */
  async createEnvironmentalState(
    locationId: string,
    type: string,
    intensity: number,
    effects: Record<string, number>,
    durationMinutes: number,
    sensoryDescription: string
  ): Promise<string> {
    const result = await this.db.query(`
      INSERT INTO environmental_states (
        location_id, state_type, intensity, duration_minutes,
        effects, sensory_description, end_time
      ) VALUES ($1, $2, $3, $4, $5, $6, NOW() + INTERVAL '1 minute' * $4)
      RETURNING state_id
    `, [locationId, type, intensity, durationMinutes, effects, sensoryDescription]);

    const stateId = result.rows[0].state_id;

    // Generate immediate sensory input
    await this.generateSensoryInput(
      locationId,
      this.mapStateToModality(type),
      intensity,
      sensoryDescription
    );

    this.emit('environmentalStateChanged', { locationId, stateId, type, intensity });

    return stateId;
  }

  /**
   * Generate sensory input at location
   */
  async generateSensoryInput(
    locationId: string,
    modality: string,
    intensity: number,
    description: string,
    sourceType: string = 'ENVIRONMENT'
  ): Promise<string> {
    const result = await this.db.query(`
      SELECT generate_environmental_sensory_input($1, $2, $3, $4)
    `, [locationId, modality, intensity, description]);

    const inputId = result.rows[0].generate_environmental_sensory_input;

    this.emit('sensoryInputGenerated', {
      inputId,
      locationId,
      modality,
      intensity,
      description
    });

    return inputId;
  }

  /**
   * Get recent sensory inputs for agent
   */
  async getAgentSensoryInputs(agentId: string, limit: number = 10): Promise<SensoryInput[]> {
    const result = await this.db.query(`
      SELECT DISTINCT
        wsi.input_id,
        wsi.sensory_modality,
        wsi.intensity,
        wsi.quality,
        wsi.description,
        wsi.source_type,
        wsi.location_id,
        wl.name as location_name
      FROM world_sensory_inputs wsi
      JOIN agent_world_positions awp ON wsi.location_id = awp.location_id
      JOIN world_locations wl ON wsi.location_id = wl.location_id
      WHERE awp.agent_id = $1 AND wsi.start_time > NOW() - INTERVAL '1 hour'
      ORDER BY wsi.start_time DESC
      LIMIT $2
    `, [agentId, limit]);

    return result.rows.map(row => ({
      inputId: row.input_id,
      modality: row.sensory_modality,
      intensity: row.intensity,
      quality: row.quality || 'neutral',
      description: row.description,
      source: row.source_type,
      locationId: row.location_id
    }));
  }

  /**
   * Apply time-based effects to agents
   */
  private async applyTimeBasedEffects(worldState: WorldTimeState): Promise<void> {
    try {
      // Get all agents
      const agentsResult = await this.db.query(`
        SELECT agent_id, location_id FROM agent_world_positions
      `);

      for (const agent of agentsResult.rows) {
        // Apply circadian effects based on time of day
        const circadianEffects = worldState.circadianInfluence;

        if (Object.keys(circadianEffects).length > 0) {
          // Update agent vitals based on circadian rhythm
          // This would integrate with the GameLoop's vitals system
          this.emit('circadianEffectsApplied', {
            agentId: agent.agent_id,
            effects: circadianEffects,
            dayPhase: worldState.dayPhase
          });
        }

        // Generate ambient sensory inputs based on lighting
        if (worldState.dayPhase === 'MORNING' || worldState.dayPhase === 'DAWN') {
          await this.generateSensoryInput(
            agent.location_id,
            'VISUAL',
            0.3,
            `Soft ${worldState.dayPhase.toLowerCase()} light filters through the space`
          );
        }
      }

    } catch (error) {
      logger.error('Failed to apply time-based effects:', error);
    }
  }

  /**
   * Map environmental state type to sensory modality
   */
  private mapStateToModality(stateType: string): string {
    const mapping: Record<string, string> = {
      'WEATHER': 'TACTILE',
      'LIGHTING': 'VISUAL',
      'TEMPERATURE': 'TEMPERATURE',
      'SOUND': 'AUDITORY',
      'EVENT': 'VISUAL',
      'DANGER': 'TACTILE'
    };
    return mapping[stateType] || 'VISUAL';
  }

  /**
   * Get world statistics
   */
  async getWorldStatistics(): Promise<any> {
    const result = await this.db.query(`
      SELECT * FROM current_world_state
    `);

    return result.rows[0];
  }

  /**
   * Get all locations with their current state
   */
  async getAllLocations(): Promise<LocationState[]> {
    const result = await this.db.query(`
      SELECT * FROM location_activity_summary
    `);

    const locations: LocationState[] = [];

    for (const row of result.rows) {
      // Get environmental states for each location
      const envStatesResult = await this.db.query(`
        SELECT state_id, state_type, intensity, effects, sensory_description,
               EXTRACT(EPOCH FROM (end_time - NOW())) / 60 as remaining_minutes
        FROM environmental_states
        WHERE location_id = $1 AND is_active = true
      `, [row.location_id]);

      const environmentalStates: EnvironmentalState[] = envStatesResult.rows.map(r => ({
        stateId: r.state_id,
        type: r.state_type,
        intensity: r.intensity,
        effects: r.effects,
        sensoryDescription: r.sensory_description,
        remainingMinutes: Math.max(0, Math.floor(r.remaining_minutes))
      }));

      locations.push({
        locationId: row.location_id,
        name: row.name,
        type: row.type,
        temperature: 22, // Base temperature
        lighting: row.base_lighting || 0.7,
        soundLevel: 0.3,
        comfort: row.base_comfort,
        privacy: row.type === 'PRIVATE' ? 1.0 : 0.5,
        safety: row.base_safety,
        socialDensity: row.occupancy_percentage / 100,
        environmentalStates
      });
    }

    return locations;
  }
}
