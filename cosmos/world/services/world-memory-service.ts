import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { WorldService, LocationState, EnvironmentalState } from './WorldService';
import { EmbodimentWorldService, EmbodimentState } from './EmbodimentWorldService';
import { EventEmitter } from 'events';

import { WorldMemoryService } from '../domains/cosmos/world/services/world-memory-service';

export interface WorldMemory {
  memoryId: string;
  agentId: string;
  memoryType: 'LOCATION' | 'ENVIRONMENTAL' | 'EVENT' | 'SENSORY' | 'SOCIAL_SPATIAL';
  locationId?: string;
  locationName?: string;

  // Memory content
  title: string;
  description: string;
  sensoryDetails: string[];
  emotionalContext: Record<string, number>;

  // Memory formation
  formationTime: Date;
  triggers: string[];
  intensity: number;
  valence: number; // -1 (negative) to 1 (positive)

  // Identity impact
  identityShift?: {
    trait: string;
    direction: 'increase' | 'decrease';
    magnitude: number;
  };

  // Association strength
  associationStrength: number;
  recallCount: number;
  lastRecalled?: Date;
}

export interface LocationAssociation {
  locationId: string;
  locationName: string;
  emotionalValence: number; // -1 to 1
  visitCount: number;
  totalDuration: number; // seconds
  associatedMemories: string[];
  meaning: string; // What this place represents to the agent
  avoidanceTendency: number; // 0-1, how much agent avoids this place
  seekingTendency: number; // 0-1, how much agent seeks this place
}

export class WorldMemoryService extends EventEmitter {
  private memoryCache: Map<string, WorldMemory[]> = new Map();
  private locationAssociations: Map<string, Map<string, LocationAssociation>> = new Map();

  constructor(
    private db: Pool,
    private worldService: WorldService,
    private embodimentService: EmbodimentWorldService
  ) {
    super();
  }

  /**
   * Initialize the memory system
   */
  async initialize(): Promise<void> {
    try {
      // Load existing location associations
      await this.loadLocationAssociations();

      logger.info('WorldMemoryService initialized');
    } catch (error) {
      logger.error('Failed to initialize WorldMemoryService:', error);
      throw error;
    }
  }

  /**
   * Process world experience and create memories
   */
  async processWorldExperience(
    agentId: string,
    experienceType: 'LOCATION_CHANGE' | 'ENVIRONMENTAL_EXPOSURE' | 'SENSORY_INPUT' | 'WORLD_EVENT' | 'SOCIAL_INTERACTION',
    context: any
  ): Promise<WorldMemory | null> {
    try {
      let memory: WorldMemory | null = null;

      switch (experienceType) {
        case 'LOCATION_CHANGE':
          memory = await this.processLocationChange(agentId, context);
          break;
        case 'ENVIRONMENTAL_EXPOSURE':
          memory = await this.processEnvironmentalExposure(agentId, context);
          break;
        case 'SENSORY_INPUT':
          memory = await this.processSensoryInput(agentId, context);
          break;
        case 'WORLD_EVENT':
          memory = await this.processWorldEvent(agentId, context);
          break;
        case 'SOCIAL_INTERACTION':
          memory = await this.processSocialInteraction(agentId, context);
          break;
      }

      if (memory) {
        // Store memory
        await this.storeWorldMemory(memory);

        // Update location associations
        if (memory.locationId) {
          await this.updateLocationAssociation(agentId, memory.locationId, memory);
        }

        // Check for identity impact
        await this.processIdentityImpact(agentId, memory);

        this.emit('memoryFormed', { agentId, memory });
      }

      return memory;

    } catch (error) {
      logger.error(`Failed to process world experience for ${agentId}:`, error);
      return null;
    }
  }

  /**
   * Process location change into memory
   */
  private async processLocationChange(
    agentId: string,
    context: { fromLocation: string; toLocation: string; purpose?: string }
  ): Promise<WorldMemory | null> {
    try {
      // Get location details
      const locationState = await this.worldService.getAgentLocationState(agentId);
      if (!locationState) return null;

      // Get embodiment state
      const embodimentState = this.embodimentService.getEmbodimentState(agentId);
      if (!embodimentState) return null;

      // Calculate emotional response based on environment
      const emotionalContext = this.calculateEmotionalResponse(locationState, embodimentState, agentId);

      // Determine if this is a significant memory
      const intensity = this.calculateMemoryIntensity(emotionalContext);
      if (intensity < 0.3) return null; // Not significant enough to remember

      const memory: WorldMemory = {
        memoryId: `world_mem_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`,
        agentId,
        memoryType: 'LOCATION',
        locationId: locationState.locationId,
        locationName: locationState.name,

        title: `Arrived at ${locationState.name}`,
        description: context.purpose
          ? `Came to ${locationState.name} to ${context.purpose}`
          : `Moved to ${locationState.name}`,

        sensoryDetails: this.generateSensoryDetails(locationState, embodimentState),
        emotionalContext,

        formationTime: new Date(),
        triggers: ['location_change', locationState.name.toLowerCase()],
        intensity,
        valence: emotionalContext.valence || 0,

        identityShift: this.calculateIdentityShift(locationState, emotionalContext, agentId),

        associationStrength: intensity,
        recallCount: 0
      };

      return memory;

    } catch (error) {
      logger.error('Failed to process location change:', error);
      return null;
    }
  }

  /**
   * Process environmental exposure into memory
   */
  private async processEnvironmentalExposure(
    agentId: string,
    context: { locationId: string; environmentalState: EnvironmentalState; duration: number }
  ): Promise<WorldMemory | null> {
    try {
      const { environmentalState, duration } = context;

      // Only significant exposures form memories
      if (environmentalState.intensity < 0.5 || duration < 60) return null;

      // Get location name
      const locationResult = await this.db.query(
        'SELECT name FROM world_locations WHERE location_id = $1',
        [context.locationId]
      );

      const locationName = locationResult.rows[0]?.name || 'Unknown Location';

      // Calculate emotional impact
      const emotionalContext = {
        stress: environmentalState.effects.stress || 0,
        discomfort: environmentalState.effects.discomfort || 0,
        pleasure: environmentalState.effects.pleasure || 0,
        fear: environmentalState.effects.fear || 0,
        valence: this.calculateValenceFromEffects(environmentalState.effects)
      };

      const memory: WorldMemory = {
        memoryId: `world_mem_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`,
        agentId,
        memoryType: 'ENVIRONMENTAL',
        locationId: context.locationId,
        locationName,

        title: `Experienced ${environmentalState.type}`,
        description: environmentalState.sensoryDescription,

        sensoryDetails: [environmentalState.sensoryDescription],
        emotionalContext,

        formationTime: new Date(),
        triggers: [environmentalState.type.toLowerCase(), ...environmentalState.sensoryDescription.split(' ').slice(0, 3)],
        intensity: environmentalState.intensity,
        valence: emotionalContext.valence,

        identityShift: this.calculateEnvironmentalIdentityShift(environmentalState, agentId),

        associationStrength: environmentalState.intensity * 0.8,
        recallCount: 0
      };

      return memory;

    } catch (error) {
      logger.error('Failed to process environmental exposure:', error);
      return null;
    }
  }

  /**
   * Process sensory input into memory
   */
  private async processSensoryInput(
    agentId: string,
    context: { modality: string; intensity: number; description: string; quality?: string }
  ): Promise<WorldMemory | null> {
    try {
      // Only intense or novel sensory inputs form memories
      if (context.intensity < 0.7) return null;

      const locationState = await this.worldService.getAgentLocationState(agentId);
      if (!locationState) return null;

      // Calculate emotional response to sensory input
      const emotionalContext = this.calculateSensoryEmotionalResponse(context, agentId);

      const memory: WorldMemory = {
        memoryId: `world_mem_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`,
        agentId,
        memoryType: 'SENSORY',
        locationId: locationState.locationId,
        locationName: locationState.name,

        title: `Sensory Experience: ${context.modality}`,
        description: context.description,

        sensoryDetails: [context.description],
        emotionalContext,

        formationTime: new Date(),
        triggers: [context.modality.toLowerCase(), context.quality || 'sensory'],
        intensity: context.intensity,
        valence: emotionalContext.valence || 0,

        associationStrength: context.intensity * 0.6,
        recallCount: 0
      };

      return memory;

    } catch (error) {
      logger.error('Failed to process sensory input:', error);
      return null;
    }
  }

  /**
   * Process world event into memory
   */
  private async processWorldEvent(
    agentId: string,
    context: { eventId: string; eventType: string; description: string; severity: number }
  ): Promise<WorldMemory | null> {
    try {
      // Only significant events form memories
      if (context.severity < 0.4) return null;

      const locationState = await this.worldService.getAgentLocationState(agentId);
      if (!locationState) return null;

      // Calculate emotional response to event
      const emotionalContext = this.calculateEventEmotionalResponse(context, agentId);

      const memory: WorldMemory = {
        memoryId: `world_mem_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`,
        agentId,
        memoryType: 'EVENT',
        locationId: locationState.locationId,
        locationName: locationState.name,

        title: `World Event: ${context.eventType}`,
        description: context.description,

        sensoryDetails: [`A ${context.eventType} occurred: ${context.description}`],
        emotionalContext,

        formationTime: new Date(),
        triggers: [context.eventType.toLowerCase(), 'world_event'],
        intensity: context.severity,
        valence: emotionalContext.valence || 0,

        identityShift: this.calculateEventIdentityShift(context, agentId),

        associationStrength: context.severity,
        recallCount: 0
      };

      return memory;

    } catch (error) {
      logger.error('Failed to process world event:', error);
      return null;
    }
  }

  /**
   * Process social interaction in spatial context
   */
  private async processSocialInteraction(
    agentId: string,
    context: { otherAgent: string; locationId: string; interactionType: string; outcome: string }
  ): Promise<WorldMemory | null> {
    try {
      // Get location details
      const locationResult = await this.db.query(
        'SELECT name, type FROM world_locations WHERE location_id = $1',
        [context.locationId]
      );

      const location = locationResult.rows[0];
      if (!location) return null;

      // Calculate emotional response
      const emotionalContext = this.calculateSocialEmotionalResponse(context, agentId);

      // Social interactions in certain locations are more memorable
      let intensityBonus = 0;
      if (location.type === 'PRIVATE') intensityBonus += 0.2;
      if (location.type === 'PUBLIC') intensityBonus += 0.1;

      const memory: WorldMemory = {
        memoryId: `world_mem_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`,
        agentId,
        memoryType: 'SOCIAL_SPATIAL',
        locationId: context.locationId,
        locationName: location.name,

        title: `Interaction with ${context.otherAgent}`,
        description: `${context.interactionType} with ${context.otherAgent} at ${location.name}`,

        sensoryDetails: [
          `The setting: ${location.name}`,
          `The interaction: ${context.interactionType}`,
          `The outcome: ${context.outcome}`
        ],
        emotionalContext,

        formationTime: new Date(),
        triggers: [context.otherAgent, location.name.toLowerCase(), context.interactionType.toLowerCase()],
        intensity: Math.min(1, 0.5 + intensityBonus),
        valence: emotionalContext.valence || 0,

        identityShift: this.calculateSocialIdentityShift(context, emotionalContext, agentId),

        associationStrength: 0.7 + intensityBonus,
        recallCount: 0
      };

      return memory;

    } catch (error) {
      logger.error('Failed to process social interaction:', error);
      return null;
    }
  }

  /**
   * Store world memory in database
   */
  private async storeWorldMemory(memory: WorldMemory): Promise<void> {
    try {
      await this.db.query(`
        INSERT INTO world_memories (
          memory_id, agent_id, memory_type, location_id, location_name,
          title, description, sensory_details, emotional_context,
          formation_time, triggers, intensity, valence,
          identity_shift, association_strength, recall_count
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9,
          $10, $11, $12, $13, $14, $15, $16
        )
      `, [
        memory.memoryId,
        memory.agentId,
        memory.memoryType,
        memory.locationId,
        memory.locationName,
        memory.title,
        memory.description,
        JSON.stringify(memory.sensoryDetails),
        JSON.stringify(memory.emotionalContext),
        memory.formationTime,
        JSON.stringify(memory.triggers),
        memory.intensity,
        memory.valence,
        JSON.stringify(memory.identityShift),
        memory.associationStrength,
        memory.recallCount
      ]);

      // Cache memory
      if (!this.memoryCache.has(memory.agentId)) {
        this.memoryCache.set(memory.agentId, []);
      }
      this.memoryCache.get(memory.agentId)!.push(memory);

    } catch (error) {
      logger.error('Failed to store world memory:', error);
    }
  }

  /**
   * Update location association for agent
   */
  private async updateLocationAssociation(
    agentId: string,
    locationId: string,
    memory: WorldMemory
  ): Promise<void> {
    try {
      // Get or create association map for agent
      if (!this.locationAssociations.has(agentId)) {
        this.locationAssociations.set(agentId, new Map());
      }

      const agentAssociations = this.locationAssociations.get(agentId)!;

      // Get or create association for location
      let association = agentAssociations.get(locationId);
      if (!association) {
        association = {
          locationId,
          locationName: memory.locationName || 'Unknown',
          emotionalValence: 0,
          visitCount: 0,
          totalDuration: 0,
          associatedMemories: [],
          meaning: '',
          avoidanceTendency: 0,
          seekingTendency: 0
        };
        agentAssociations.set(locationId, association);
      }

      // Update association
      association.emotionalValence = (association.emotionalValence * 0.8) + (memory.valence * 0.2);
      association.visitCount += 1;
      association.associatedMemories.push(memory.memoryId);

      // Update meaning based on memories
      association.meaning = this.deriveLocationMeaning(association, agentId);

      // Update tendencies
      if (memory.valence < -0.5) {
        association.avoidanceTendency = Math.min(1, association.avoidanceTendency + 0.1);
      } else if (memory.valence > 0.5) {
        association.seekingTendency = Math.min(1, association.seekingTendency + 0.1);
      }

      // Persist to database
      await this.db.query(`
        INSERT INTO location_associations (
          agent_id, location_id, location_name, emotional_valence,
          visit_count, total_duration, associated_memories,
          meaning, avoidance_tendency, seeking_tendency
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (agent_id, location_id) DO UPDATE SET
          emotional_valence = EXCLUDED.emotional_valence,
          visit_count = EXCLUDED.visit_count,
          associated_memories = EXCLUDED.associated_memories,
          meaning = EXCLUDED.meaning,
          avoidance_tendency = EXCLUDED.avoidance_tendency,
          seeking_tendency = EXCLUDED.seeking_tendency
      `, [
        agentId,
        locationId,
        association.locationName,
        association.emotionalValence,
        association.visitCount,
        association.totalDuration,
        JSON.stringify(association.associatedMemories),
        association.meaning,
        association.avoidanceTendency,
        association.seekingTendency
      ]);

    } catch (error) {
      logger.error('Failed to update location association:', error);
    }
  }

  /**
   * Process identity impact from memory
   */
  private async processIdentityImpact(agentId: string, memory: WorldMemory): Promise<void> {
    if (!memory.identityShift) return;

    try {
      // Store identity shift
      await this.db.query(`
        INSERT INTO identity_shifts (
          agent_id, memory_id, trait, direction, magnitude,
          shift_time, context
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
      `, [
        agentId,
        memory.memoryId,
        memory.identityShift.trait,
        memory.identityShift.direction,
        memory.identityShift.magnitude,
        memory.formationTime,
        JSON.stringify({
          memoryType: memory.memoryType,
          location: memory.locationName,
          intensity: memory.intensity
        })
      ]);

      this.emit('identityShifted', {
        agentId,
        shift: memory.identityShift,
        memory
      });

    } catch (error) {
      logger.error('Failed to process identity impact:', error);
    }
  }

  /**
   * Get agent's location associations
   */
  async getLocationAssociations(agentId: string): Promise<LocationAssociation[]> {
    try {
      const result = await this.db.query(`
        SELECT * FROM location_associations
        WHERE agent_id = $1
        ORDER BY emotional_valence DESC
      `, [agentId]);

      return result.rows.map(row => ({
        locationId: row.location_id,
        locationName: row.location_name,
        emotionalValence: row.emotional_valence,
        visitCount: row.visit_count,
        totalDuration: row.total_duration,
        associatedMemories: JSON.parse(row.associated_memories || '[]'),
        meaning: row.meaning,
        avoidanceTendency: row.avoidance_tendency,
        seekingTendency: row.seeking_tendency
      }));

    } catch (error) {
      logger.error('Failed to get location associations:', error);
      return [];
    }
  }

  /**
   * Recall memories related to a location
   */
  async recallLocationMemories(agentId: string, locationId: string, limit: number = 5): Promise<WorldMemory[]> {
    try {
      const result = await this.db.query(`
        SELECT * FROM world_memories
        WHERE agent_id = $1 AND location_id = $2
        ORDER BY intensity DESC, formation_time DESC
        LIMIT $3
      `, [agentId, locationId, limit]);

      // Update recall counts
      for (const row of result.rows) {
        await this.db.query(`
          UPDATE world_memories
          SET recall_count = recall_count + 1,
          last_recalled = NOW()
          WHERE memory_id = $1
        `, [row.memory_id]);
      }

      return result.rows.map(row => ({
        memoryId: row.memory_id,
        agentId: row.agent_id,
        memoryType: row.memory_type,
        locationId: row.location_id,
        locationName: row.location_name,
        title: row.title,
        description: row.description,
        sensoryDetails: JSON.parse(row.sensory_details || '[]'),
        emotionalContext: JSON.parse(row.emotional_context || '{}'),
        formationTime: row.formation_time,
        triggers: JSON.parse(row.triggers || '[]'),
        intensity: row.intensity,
        valence: row.valence,
        identityShift: JSON.parse(row.identity_shift || '{}'),
        associationStrength: row.association_strength,
        recallCount: row.recall_count,
        lastRecalled: row.last_recalled
      }));

    } catch (error) {
      logger.error('Failed to recall location memories:', error);
      return [];
    }
  }

  /**
   * Helper methods for calculating emotional responses and impacts
   */
  private calculateEmotionalResponse(
    location: LocationState,
    embodiment: EmbodimentState,
    agentId: string
  ): Record<string, number> {
    const response: Record<string, number> = {};

    // Comfort affects mood
    response.comfort = location.comfort;

    // Safety affects security
    response.security = location.safety;

    // Crowding affects social anxiety
    if (location.socialDensity > 0.7) {
      response.anxiety = location.socialDensity * 0.5;
    }

    // Pain affects distress
    if (embodiment.painLevel > 0) {
      response.distress = embodiment.painLevel;
    }

    // Calculate overall valence
    response.valence = (response.comfort + response.security - response.anxiety - response.distress) / 4;

    return response;
  }

  private calculateMemoryIntensity(emotionalContext: Record<string, number>): number {
    const values = Object.values(emotionalContext).filter(v => typeof v === 'number');
    return values.reduce((sum, val) => sum + Math.abs(val), 0) / values.length;
  }

  private generateSensoryDetails(location: LocationState, embodiment: EmbodimentState): string[] {
    const details: string[] = [];

    if (location.lighting > 0.8) details.push('Bright, well-lit space');
    else if (location.lighting < 0.4) details.push('Dim, shadowy environment');

    if (location.soundLevel > 0.6) details.push('Audibly active area');
    else if (location.soundLevel < 0.3) details.push('Quiet and peaceful');

    if (embodiment.painLevel > 0) details.push('Physical discomfort present');
    if (embodiment.stressLevel > 0.5) details.push('Tense atmosphere');

    return details;
  }

  private calculateIdentityShift(
    location: LocationState,
    emotionalContext: Record<string, number>,
    agentId: string
  ): { trait: string; direction: 'increase' | 'decrease'; magnitude: number } | undefined {
    // Significant experiences can shift identity
    if (Math.abs(emotionalContext.valence || 0) < 0.6) return undefined;

    if (location.type === 'PRIVATE') {
      if (emotionalContext.valence! > 0.6) {
        return { trait: 'introversion', direction: 'increase', magnitude: 0.1 };
      }
    } else if (location.type === 'PUBLIC') {
      if (emotionalContext.valence! > 0.6) {
        return { trait: 'extroversion', direction: 'increase', magnitude: 0.1 };
      }
    }

    return undefined;
  }

  private calculateValenceFromEffects(effects: Record<string, number>): number {
    const positive = effects.pleasure || effects.comfort || 0;
    const negative = (effects.pain || effects.stress || effects.fear || 0);
    return (positive - negative) / Math.max(1, positive + negative);
  }

  private calculateEnvironmentalIdentityShift(
    environmentalState: EnvironmentalState,
    agentId: string
  ): { trait: string; direction: 'increase' | 'decrease'; magnitude: number } | undefined {
    if (environmentalState.type === 'DANGER' && environmentalState.intensity > 0.7) {
      return { trait: 'caution', direction: 'increase', magnitude: environmentalState.intensity * 0.2 };
    }
    return undefined;
  }

  private calculateSensoryEmotionalResponse(
    context: { modality: string; intensity: number; description: string; quality?: string },
    agentId: string
  ): Record<string, number> {
    const response: Record<string, number> = {};

    if (context.quality === 'painful') {
      response.pain = context.intensity;
      response.valence = -context.intensity;
    } else if (context.quality === 'comforting') {
      response.comfort = context.intensity;
      response.valence = context.intensity;
    } else {
      response.valence = 0;
    }

    return response;
  }

  private calculateEventEmotionalResponse(
    context: { eventType: string; description: string; severity: number },
    agentId: string
  ): Record<string, number> {
    const response: Record<string, number> = {};

    if (context.eventType === 'DANGER') {
      response.fear = context.severity;
      response.valence = -context.severity;
    } else if (context.eventType === 'OPPORTUNITY') {
      response.excitement = context.severity;
      response.valence = context.severity;
    } else {
      response.valence = 0;
    }

    return response;
  }

  private calculateEventIdentityShift(
    context: { eventType: string; description: string; severity: number },
    agentId: string
  ): { trait: string; direction: 'increase' | 'decrease'; magnitude: number } | undefined {
    if (context.eventType === 'DANGER' && context.severity > 0.6) {
      return { trait: 'risk_aversion', direction: 'increase', magnitude: context.severity * 0.15 };
    }
    return undefined;
  }

  private calculateSocialEmotionalResponse(
    context: { otherAgent: string; locationId: string; interactionType: string; outcome: string },
    agentId: string
  ): Record<string, number> {
    const response: Record<string, number> = {};

    // Simple valence based on outcome
    if (context.outcome.includes('positive') || context.outcome.includes('good')) {
      response.valence = 0.7;
    } else if (context.outcome.includes('negative') || context.outcome.includes('bad')) {
      response.valence = -0.7;
    } else {
      response.valence = 0;
    }

    return response;
  }

  private calculateSocialIdentityShift(
    context: { otherAgent: string; locationId: string; interactionType: string; outcome: string },
    emotionalContext: Record<string, number>,
    agentId: string
  ): { trait: string; direction: 'increase' | 'decrease'; magnitude: number } | undefined {
    if (emotionalContext.valence! > 0.6 && context.interactionType === 'collaboration') {
      return { trait: 'cooperation', direction: 'increase', magnitude: 0.1 };
    }
    return undefined;
  }

  private deriveLocationMeaning(association: LocationAssociation, agentId: string): string {
    if (association.emotionalValence > 0.5) {
      return 'A place of comfort and positive experiences';
    } else if (association.emotionalValence < -0.5) {
      return 'A place associated with negative experiences';
    } else if (association.visitCount > 10) {
      return 'A familiar and frequently visited place';
    } else {
      return 'A place of neutral significance';
    }
  }

  private async loadLocationAssociations(): Promise<void> {
    try {
      const result = await this.db.query('SELECT * FROM location_associations');

      for (const row of result.rows) {
        if (!this.locationAssociations.has(row.agent_id)) {
          this.locationAssociations.set(row.agent_id, new Map());
        }

        this.locationAssociations.get(row.agent_id)!.set(row.location_id, {
          locationId: row.location_id,
          locationName: row.location_name,
          emotionalValence: row.emotional_valence,
          visitCount: row.visit_count,
          totalDuration: row.total_duration,
          associatedMemories: JSON.parse(row.associated_memories || '[]'),
          meaning: row.meaning,
          avoidanceTendency: row.avoidance_tendency,
          seekingTendency: row.seeking_tendency
        });
      }
    } catch (error) {
      logger.error('Failed to load location associations:', error);
    }
  }
}
