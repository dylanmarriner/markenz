import { Pool } from 'pg';
import { logger } from '../../../../../../shared/utils/logger';
import { WorldService, LocationState, SensoryInput } from './world-service';
import { EventEmitter } from 'events';
import { BodyStateService } from '../../../../../../modules/embodiment/services/body-state-service';
import { SensoryInputService } from '../../../../../../modules/embodiment/services/sensory-input-service';
import { EmbodimentState, EnvironmentalEffect } from '../../../../../../modules/embodiment/types';


export class EmbodimentWorldService extends EventEmitter {
  private embodimentStates: Map<string, EmbodimentState> = new Map();
  private environmentTolerance: Map<string, Record<string, number>> = new Map();

  constructor(
    private db: Pool,
    private worldService: WorldService,
    private bodyStateService: BodyStateService,
    private sensoryInputService: SensoryInputService
  ) {
    super();
    
    // Initialize agent tolerances
    this.initializeTolerance();
  }

  /**
   * Initialize agent environmental tolerances
   */
  private initializeTolerance(): void {
    // Gem-D (Water/Pisces) - More sensitive to environment
    this.environmentTolerance.set('gem-d', {
      temperature: 0.7,    // Sensitive to cold
      lighting: 0.6,       // Prefers dim lighting
      sound: 0.8,         // Sensitive to noise
      crowding: 0.5,      // Low crowding tolerance
      safety: 0.9         // High safety need
    });

    // Gem-K (Fire/Sagittarius) - More resilient
    this.environmentTolerance.set('gem-k', {
      temperature: 0.9,    // High heat tolerance
      lighting: 0.8,       // Bright light OK
      sound: 0.6,         // Moderate noise tolerance
      crowding: 0.7,      // Social tolerance
      safety: 0.6         // Lower safety need
    });
  }

  /**
   * Initialize the embodiment service
   */
  async initialize(): Promise<void> {
    try {
      // Initialize environment tolerances
      this.initializeEnvironmentTolerances();
      
      logger.info('EmbodimentWorldService initialized');
    } catch (error) {
      logger.error('Failed to initialize EmbodimentWorldService:', error);
      throw error;
    }
  }

  /**
   * Initialize environment tolerances for agents
   */
  private initializeEnvironmentTolerances(): void {
    // Gem-D (Water/Pisces) - More sensitive to environment
    this.environmentTolerance.set('gem-d', {
      temperature: 0.7,    // Sensitive to cold
      lighting: 0.6,       // Prefers dim lighting
      sound: 0.8,         // Sensitive to noise
      crowding: 0.5,      // Low crowding tolerance
      safety: 0.9         // High safety need
    });

    // Gem-K (Fire/Sagittarius) - More resilient
    this.environmentTolerance.set('gem-k', {
      temperature: 0.9,    // High heat tolerance
      lighting: 0.8,       // Bright light OK
      sound: 0.6,         // Moderate noise tolerance
      crowding: 0.7,      // Social tolerance
      safety: 0.6         // Lower safety need
    });
  }

  /**
   * Initialize embodiment state for agent
   */
  async initializeEmbodiment(agentId: string): Promise<EmbodimentState> {
    try {
      // Get current world position
      const positionResult = await this.db.query(`
        SELECT location_id, coordinates, posture, is_moving, vehicle_id, is_mounted
        FROM agent_world_positions
        WHERE agent_id = $1
      `, [agentId]);

      if (positionResult.rows.length === 0) {
        throw new Error(`Agent ${agentId} not found in world`);
      }

      const position = positionResult.rows[0];

      // Get location state
      const locationState = await this.worldService.getAgentLocationState(agentId);
      if (!locationState) {
        throw new Error(`Agent ${agentId} has no location state`);
      }

      // Calculate initial embodiment state based on location
      const embodimentState: EmbodimentState = {
        agentId,
        locationId: position.location_id,
        coordinates: position.coordinates,
        posture: position.posture || 'STANDING',
        isMoving: position.is_moving || false,
        vehicleId: position.vehicle_id,
        isMounted: position.is_mounted || false,
        
        // Physical needs influenced by environment
        bodyTemperature: this.calculateBodyTemperature(locationState.temperature, agentId),
        energyLevel: 100.0,
        comfortLevel: this.calculateComfortLevel(locationState, agentId),
        stressLevel: this.calculateStressLevel(locationState, agentId),
        painLevel: 0.0,
        
        // Sensory acuity
        visualAcuity: this.calculateVisualAcuity(locationState.lighting, agentId),
        auditorySensitivity: this.calculateAuditorySensitivity(locationState.soundLevel, agentId),
        tactileSensitivity: 0.8,
        proprioceptionAccuracy: 0.9
      };

      // Store in memory
      this.embodimentStates.set(agentId, embodimentState);

      // Generate initial sensory inputs
      await this.generateEnvironmentalSensoryInputs(agentId, locationState);

      logger.info(`Embodiment initialized for ${agentId}`, {
        location: locationState.name,
        bodyTemperature: embodimentState.bodyTemperature,
        comfortLevel: embodimentState.comfortLevel
      });

      this.emit('embodimentInitialized', { agentId, state: embodimentState });

      return embodimentState;

    } catch (error) {
      logger.error(`Failed to initialize embodiment for ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Update embodiment state based on world changes
   */
  async updateEmbodimentFromWorld(agentId: string): Promise<EmbodimentState> {
    try {
      const currentState = this.embodimentStates.get(agentId);
      if (!currentState) {
        return await this.initializeEmbodiment(agentId);
      }

      // Get current location state
      const locationState = await this.worldService.getAgentLocationState(agentId);
      if (!locationState) {
        throw new Error(`Agent ${agentId} has no location state`);
      }

      // Calculate environmental effects
      const effects = this.calculateEnvironmentalEffects(locationState, agentId);

      // Update embodiment state
      const updatedState: EmbodimentState = {
        ...currentState,
        locationId: locationState.locationId,
        
        // Apply environmental effects
        bodyTemperature: Math.max(35, Math.min(40, 
          currentState.bodyTemperature + effects.temperature * 0.1)),
        comfortLevel: Math.max(0, Math.min(1,
          currentState.comfortLevel + effects.comfort * 0.05)),
        stressLevel: Math.max(0, Math.min(1,
          currentState.stressLevel + effects.stress * 0.05)),
        
        // Update sensory acuity based on conditions
        visualAcuity: this.calculateVisualAcuity(locationState.lighting, agentId),
        auditorySensitivity: this.calculateAuditorySensitivity(locationState.soundLevel, agentId)
      };

      // Check for pain/discomfort from extreme conditions
      if (locationState.temperature < 15 || locationState.temperature > 30) {
        updatedState.painLevel = Math.min(1, Math.abs(locationState.temperature - 22.5) / 10);
      }

      // Store updated state in memory
      this.embodimentStates.set(agentId, updatedState);

      // Persist to database
      await this.bodyStateService.updateBodyState(agentId, {
        energy_level: updatedState.energyLevel,
        pain_level: updatedState.painLevel,
        body_temperature: updatedState.bodyTemperature
      });

      // Generate sensory inputs for changes
      await this.generateEnvironmentalSensoryInputs(agentId, locationState);

      // Emit update
      this.emit('embodimentUpdated', { agentId, state: updatedState, effects });

      return updatedState;

    } catch (error) {
      logger.error(`Failed to update embodiment for ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Process sensory input and update embodiment
   */
  async processSensoryInput(agentId: string, sensoryInput: SensoryInput): Promise<void> {
    try {
      const state = this.embodimentStates.get(agentId);
      if (!state) {
        return;
      }

      // Process based on modality
      switch (sensoryInput.modality) {
        case 'TEMPERATURE':
          // Adjust body temperature
          const tempEffect = (sensoryInput.intensity - 0.5) * 2; // -1 to 1
          state.bodyTemperature = Math.max(35, Math.min(40, 
            state.bodyTemperature + tempEffect * 0.2));
          
          // Extreme temperatures cause discomfort/pain
          if (Math.abs(tempEffect) > 0.7) {
            state.painLevel = Math.min(1, state.painLevel + 0.1);
            state.stressLevel = Math.min(1, state.stressLevel + 0.05);
          }
          break;

        case 'VISUAL':
          // Bright light affects stress and energy
          if (sensoryInput.intensity > 0.8) {
            state.stressLevel = Math.min(1, state.stressLevel + 0.02);
            state.energyLevel = Math.max(0, state.energyLevel - 0.5);
          } else if (sensoryInput.intensity < 0.2) {
            // Low light can be calming or cause fatigue
            state.stressLevel = Math.max(0, state.stressLevel - 0.01);
            state.energyLevel = Math.max(0, state.energyLevel - 0.2);
          }
          break;

        case 'AUDITORY':
          // Loud noise increases stress
          if (sensoryInput.intensity > 0.7) {
            state.stressLevel = Math.min(1, state.stressLevel + 0.03);
            state.painLevel = Math.min(1, state.painLevel + 0.02);
          }
          break;

        case 'TACTILE':
          // Tactile sensations can affect comfort and pain
          if (sensoryInput.quality === 'painful') {
            state.painLevel = Math.min(1, state.painLevel + sensoryInput.intensity * 0.2);
            state.stressLevel = Math.min(1, state.stressLevel + sensoryInput.intensity * 0.1);
          } else if (sensoryInput.quality === 'comforting') {
            state.comfortLevel = Math.min(1, state.comfortLevel + sensoryInput.intensity * 0.1);
            state.stressLevel = Math.max(0, state.stressLevel - sensoryInput.intensity * 0.05);
          }
          break;

        case 'PROPRIOCEPTION':
          // Update body awareness
          state.proprioceptionAccuracy = Math.min(1, 
            state.proprioceptionAccuracy + sensoryInput.intensity * 0.1);
          break;
      }

      // Store updated state
      this.embodimentStates.set(agentId, state);

      // Persist sensory changes to database
      await this.sensoryInputService.updateSensoryInput(agentId, {
        visual_acuity: state.visualAcuity,
        volume_sensitivity: state.auditorySensitivity,
        tactile_sensitivity: state.tactileSensitivity,
        spatial_awareness: state.proprioceptionAccuracy
      });

      // Emit sensory processing event
      this.emit('sensoryProcessed', {
        agentId,
        modality: sensoryInput.modality,
        intensity: sensoryInput.intensity,
        newState: state
      });

    } catch (error) {
      logger.error(`Failed to process sensory input for ${agentId}:`, error);
    }
  }

  /**
   * Move agent and update embodiment
   */
  async moveAgent(agentId: string, targetLocationId: string, purpose?: string): Promise<void> {
    try {
      // Move in world
      await this.worldService.moveAgent(agentId, targetLocationId, purpose);

      // Update embodiment
      await this.updateEmbodimentFromWorld(agentId);

      // Generate movement-related sensory inputs
      await this.generateMovementSensoryInputs(agentId);

    } catch (error) {
      logger.error(`Failed to move agent ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Get current embodiment state
   */
  getEmbodimentState(agentId: string): EmbodimentState | null {
    return this.embodimentStates.get(agentId) || null;
  }

  /**
   * Get all embodiment states
   */
  getAllEmbodimentStates(): Map<string, EmbodimentState> {
    return new Map(this.embodimentStates);
  }

  /**
   * Calculate environmental effects on agent
   */
  private calculateEnvironmentalEffects(location: LocationState, agentId: string): Record<string, number> {
    const tolerance = this.environmentTolerance.get(agentId) || {
      temperature: 0.8, lighting: 0.8, sound: 0.8, crowding: 0.8, safety: 0.8
    };

    const effects: Record<string, number> = {};

    // Temperature effect
    const tempDeviation = Math.abs(location.temperature - 22) / 10; // Deviation from ideal
    effects.temperature = tempDeviation * (1 - tolerance.temperature);

    // Lighting effect
    const lightingDeviation = Math.abs(location.lighting - 0.7) / 0.7;
    effects.lighting = lightingDeviation * (1 - tolerance.lighting);

    // Sound effect
    effects.sound = location.soundLevel * (1 - tolerance.sound);

    // Crowding effect
    effects.crowding = location.socialDensity * (1 - tolerance.crowding);

    // Safety effect
    effects.safety = (1 - location.safety) * (1 - tolerance.safety);

    // Combined comfort effect
    effects.comfort = -(effects.temperature + effects.lighting + effects.sound + 
                       effects.crowding + effects.safety) / 5;

    // Combined stress effect
    effects.stress = (effects.temperature + effects.sound + effects.crowding + 
                     effects.safety) / 4;

    return effects;
  }

  /**
   * Calculate body temperature based on environment
   */
  private calculateBodyTemperature(ambientTemp: number, agentId: string): number {
    const tolerance = this.environmentTolerance.get(agentId);
    const tempSensitivity = tolerance ? (1 - tolerance.temperature) : 0.2;

    // Base body temperature + environmental influence
    const baseTemp = 37.0;
    const envEffect = (ambientTemp - 22) * 0.1 * tempSensitivity;
    
    return baseTemp + envEffect;
  }

  /**
   * Calculate comfort level based on location
   */
  private calculateComfortLevel(location: LocationState, agentId: string): number {
    const effects = this.calculateEnvironmentalEffects(location, agentId);
    
    // Start with base comfort and subtract negative effects
    let comfort = location.comfort;
    
    // Apply environmental stressors
    comfort -= effects.temperature * 0.3;
    comfort -= effects.sound * 0.2;
    comfort -= effects.crowding * 0.3;
    comfort -= effects.safety * 0.2;
    
    return Math.max(0, Math.min(1, comfort));
  }

  /**
   * Calculate stress level based on location
   */
  private calculateStressLevel(location: LocationState, agentId: string): number {
    const effects = this.calculateEnvironmentalEffects(location, agentId);
    
    // Base stress from environmental factors
    let stress = effects.stress;
    
    // Low safety increases stress
    if (location.safety < 0.5) {
      stress += (0.5 - location.safety) * 0.5;
    }
    
    // Low privacy can increase stress for some agents
    const tolerance = this.environmentTolerance.get(agentId);
    if (tolerance && location.privacy < 0.5 && tolerance.crowding < 0.6) {
      stress += (0.5 - location.privacy) * 0.3;
    }
    
    return Math.max(0, Math.min(1, stress));
  }

  /**
   * Calculate visual acuity based on lighting
   */
  private calculateVisualAcuity(lighting: number, agentId: string): number {
    const tolerance = this.environmentTolerance.get(agentId);
    const lightPreference = tolerance ? tolerance.lighting : 0.8;
    
    // Optimal acuity at preferred lighting level
    const optimal = lightPreference * 0.7 + 0.3; // Scale preference
    const deviation = Math.abs(lighting - optimal);
    
    return Math.max(0.1, 1 - deviation);
  }

  /**
   * Calculate auditory sensitivity based on sound level
   */
  private calculateAuditorySensitivity(soundLevel: number, agentId: string): number {
    const tolerance = this.environmentTolerance.get(agentId);
    const noiseTolerance = tolerance ? tolerance.sound : 0.8;
    
    // High sensitivity in quiet environments, reduced in noise
    if (soundLevel < 0.3) {
      return 0.9 + (0.3 - soundLevel);
    } else {
      return Math.max(0.3, 1 - (soundLevel * (1 - noiseTolerance)));
    }
  }

  /**
   * Generate environmental sensory inputs for agent
   */
  private async generateEnvironmentalSensoryInputs(agentId: string, location: LocationState): Promise<void> {
    try {
      // Temperature sensation
      if (Math.abs(location.temperature - 22) > 2) {
        const intensity = Math.min(1, Math.abs(location.temperature - 22) / 10);
        const quality = location.temperature > 22 ? 'warm' : 'cool';
        await this.worldService.generateSensoryInput(
          location.locationId,
          'TEMPERATURE',
          intensity,
          `The air feels ${quality} here`,
          'ENVIRONMENT'
        );
      }

      // Lighting sensation
      if (location.lighting > 0.8) {
        await this.worldService.generateSensoryInput(
          location.locationId,
          'VISUAL',
          location.lighting,
          'Bright light illuminates the space',
          'ENVIRONMENT'
        );
      } else if (location.lighting < 0.4) {
        await this.worldService.generateSensoryInput(
          location.locationId,
          'VISUAL',
          1 - location.lighting,
          'The space is dimly lit',
          'ENVIRONMENT'
        );
      }

      // Sound sensation
      if (location.soundLevel > 0.6) {
        await this.worldService.generateSensoryInput(
          location.locationId,
          'AUDITORY',
          location.soundLevel,
          'There is noticeable ambient noise',
          'ENVIRONMENT'
        );
      }

      // Environmental states
      for (const envState of location.environmentalStates) {
        await this.worldService.generateSensoryInput(
          location.locationId,
          this.mapStateTypeToModality(envState.type),
          envState.intensity,
          envState.sensoryDescription,
          'ENVIRONMENT'
        );
      }

    } catch (error) {
      logger.error(`Failed to generate sensory inputs for ${agentId}:`, error);
    }
  }

  /**
   * Generate movement-related sensory inputs
   */
  private async generateMovementSensoryInputs(agentId: string): Promise<void> {
    try {
      const state = this.embodimentStates.get(agentId);
      if (!state) return;

      // Proprioceptive input from movement
      await this.worldService.generateSensoryInput(
        state.locationId,
        'PROPRIOCEPTION',
        0.5,
        'You feel your body moving through space',
        'ENVIRONMENT'
      );

      // Brief disorientation from movement
      state.proprioceptionAccuracy = Math.max(0.5, 
        state.proprioceptionAccuracy - 0.1);

    } catch (error) {
      logger.error(`Failed to generate movement inputs for ${agentId}:`, error);
    }
  }

  /**
   * Map environmental state type to sensory modality
   */
  private mapStateTypeToModality(stateType: string): string {
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
   * Apply damage or healing to agent
   */
  async applyHealthEffect(agentId: string, effect: 'damage' | 'heal', amount: number, source: string): Promise<void> {
    try {
      const state = this.embodimentStates.get(agentId);
      if (!state) {
        return;
      }

      if (effect === 'damage') {
        state.painLevel = Math.min(1, state.painLevel + amount);
        state.stressLevel = Math.min(1, state.stressLevel + amount * 0.5);
        state.energyLevel = Math.max(0, state.energyLevel - amount * 10);
        
        // Generate pain sensation
        await this.worldService.generateSensoryInput(
          state.locationId,
          'TACTILE',
          amount,
          `You feel pain from ${source}`,
          'ENVIRONMENT'
        );
      } else {
        state.painLevel = Math.max(0, state.painLevel - amount);
        state.stressLevel = Math.max(0, state.stressLevel - amount * 0.3);
        state.comfortLevel = Math.min(1, state.comfortLevel + amount * 0.2);
        
        // Generate healing sensation
        await this.worldService.generateSensoryInput(
          state.locationId,
          'TACTILE',
          amount,
          `You feel healing from ${source}`,
          'ENVIRONMENT'
        );
      }

      this.embodimentStates.set(agentId, state);
      
      this.emit('healthEffectApplied', {
        agentId,
        effect,
        amount,
        source,
        newState: state
      });

    } catch (error) {
      logger.error(`Failed to apply health effect to ${agentId}:`, error);
    }
  }
}
