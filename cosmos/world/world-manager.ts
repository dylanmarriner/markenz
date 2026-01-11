import { Pool } from 'pg';
import { logger } from '../../utils/logger';
import { BioSysRegistry } from './services/BioSysRegistry';
import { EventEmitter } from 'events';
import { WorldService } from './services/WorldService';
import { EmbodimentWorldService } from './services/EmbodimentWorldService';
import { WorldMemoryService } from './services/WorldMemoryService';
import { WorldConsequencesService } from './services/WorldConsequencesService';
import { WorldSeederService } from './services/WorldSeederService';
import { BodyStateService } from '../../../modules/embodiment/services/body-state-service';
import { SensoryInputService } from '../../../modules/embodiment/services/sensory-input-service';
import { PhysWorld } from '../../../physworld';
import { SensoryBridge } from '../../../sensory';

export interface WorldState {
  worldTime: {
    current: Date;
    dayNumber: number;
    hourOfDay: number;
    dayPhase: string;
  };
  agents: {
    [agentId: string]: {
      locationId: string;
      locationName: string;
      embodiment: any;
      sensoryInputs: any[];
      activeConsequences: any[];
    };
  };
  locations: {
    [locationId: string]: {
      name: string;
      type: string;
      environmentalStates: any[];
      occupancy: number;
    };
  };
}

export class WorldManager extends EventEmitter {
  private isInitialized: boolean = false;
  protected isRunning: boolean = false;
  private tickCount: number = 0;

  // Services
  public worldService: WorldService;
  public embodimentService: EmbodimentWorldService;
  public memoryService: WorldMemoryService;
  public consequencesService: WorldConsequencesService;
  public seederService: WorldSeederService;
  public bodyStateService: BodyStateService;
  public sensoryInputService: SensoryInputService;
  public bioSysRegistry: BioSysRegistry;
  public physWorld: PhysWorld; // THE LAW of physical authority
  public sensoryBridge: SensoryBridge; // THE LAW of sensory authority

  constructor(private db: Pool) {
    super();
    
    // Initialize services
    this.worldService = new WorldService(db);
    this.bodyStateService = new BodyStateService(db);
    this.sensoryInputService = new SensoryInputService(db);
    this.embodimentService = new EmbodimentWorldService(db, this.worldService, this.bodyStateService, this.sensoryInputService);
    this.memoryService = new WorldMemoryService(db, this.worldService, this.embodimentService);
    this.consequencesService = new WorldConsequencesService(db, this.worldService, this.embodimentService);
    this.seederService = new WorldSeederService(db);
    this.bioSysRegistry = BioSysRegistry.getInstance();
    this.physWorld = new PhysWorld(db); // THE LAW of physical authority
    this.sensoryBridge = new SensoryBridge(db, this.physWorld); // THE LAW of sensory authority

    // Wire up service event handlers
    this.setupServiceEventHandlers();
  }

  /**
   * Initialize the world system
   */
  async initialize(): Promise<void> {
    try {
      if (this.isInitialized) {
        logger.warn('WorldManager already initialized');
        return;
      }

      logger.info('Initializing WorldManager...');

      // Run world migration
      await this.runWorldMigration();

      // Seed the world with locations and objects
      const seedResult = await this.seederService.seedWorld();
      if (seedResult.success) {
        if (seedResult.wasCreated) {
          logger.info('✅ World created and seeded successfully');
        } else {
          logger.info('✅ World already exists');
        }
      } else {
        throw new Error(`World seeding failed: ${seedResult.message}`);
      }

      // Initialize all services
      await Promise.all([
        this.worldService.initialize(),
        this.embodimentService.initialize(),
        this.memoryService.initialize(),
        this.consequencesService.initialize(),
        this.bioSysRegistry.initialize()
      ]);

      // Start PhysWorld - THE LAW of physical authority
      this.physWorld.start();
      
      // Start SensoryBridge - THE LAW of sensory authority
      await this.sensoryBridge.start();

      // Initialize agent embodiments
      await this.initializeAgentEmbodiments();

      // CRITICAL: Validate BioSys system integrity
      const expectedAgentIds = ['gem-d', 'gem-k']; // These should be the only agents
      if (!this.bioSysRegistry.validateSystemIntegrity(expectedAgentIds)) {
        throw new Error('BioSys system integrity validation failed - missing or extra BioSys instances');
      }

      this.isInitialized = true;
      logger.info('WorldManager initialized successfully');

      this.emit('initialized');

    } catch (error) {
      logger.error('Failed to initialize WorldManager:', error);
      throw error;
    }
  }

  /**
   * Start the world simulation
   */
  async start(): Promise<void> {
    try {
      if (!this.isInitialized) {
        throw new Error('WorldManager must be initialized before starting');
      }

      if (this.isRunning) {
        logger.warn('WorldManager already running');
        return;
      }

      logger.info('Starting WorldManager...');

      // Start world service (time progression)
      await this.worldService.start();

      this.isRunning = true;
      logger.info('WorldManager started - world simulation active');

      this.emit('started');

    } catch (error) {
      logger.error('Failed to start WorldManager:', error);
      throw error;
    }
  }

  /**
   * Main tick - called by GameLoop every second
   * CRITICAL: BioSys tick is authoritative for all biological state
   */
  async tick(): Promise<void> {
    if (!this.isRunning) return;

    try {
      this.tickCount++;

      // Process world time advancement
      const worldTimeState = await this.worldService.advanceWorldTime(1);

      // CRITICAL: Tick BioSys for all agents - THE LAW of biological simulation
      const bioSysTickMs = 100; // Fixed 100ms cadence as required
      const bioSysAgentIds = this.bioSysRegistry.getAllAgentIds();
      
      // Use BioSysManager forceTick for authoritative biological timing
      const bioSysManager = this.bioSysRegistry['bioSysManager']; // Access private property
      
      if (bioSysManager && typeof bioSysManager.forceTick === 'function') {
        // Force tick all agents individually - authoritative biological timing
        for (const agentId of bioSysAgentIds) {
          bioSysManager.forceTick();
        }
      } else {
        // Fallback to individual ticks
        for (const agentId of bioSysAgentIds) {
          const humanBody = this.bioSysRegistry.getBioSys(agentId);
          if (!humanBody) {
            // FAIL CLOSED: Missing HumanBody is a system integrity violation
            throw new Error(`HumanBody missing for agent ${agentId} during tick - system integrity violation`);
          }
          // HumanBody handles internal ticking - just validate it's alive
          if (!humanBody.getIsAlive()) {
            throw new Error(`HumanBody died for agent ${agentId} during tick - system integrity violation`);
          }
        }
      }

      // Process time-based consequences
      await this.consequencesService.processTimeConsequences(worldTimeState);

      // Update all agent embodiments (now read-only facades over BioSys)
      await this.updateAllAgentEmbodiments();

      // Process environmental effects
      await this.processEnvironmentalEffects();

      // Emit world state for broadcasting
      const worldState = await this.getWorldState();
      this.emit('worldTick', { tickCount: this.tickCount, worldState });

    } catch (error) {
      logger.error(`WorldManager tick ${this.tickCount} failed:`, error);
      throw error; // FAIL CLOSED - don't hide tick failures
    }
  }

  /**
   * Check if world simulation is running
   */
  public getIsRunning(): boolean {
    return this.isRunning;
  }

  /**
   * Stop the world simulation
   */
  async stop(): Promise<void> {
    if (!this.isRunning) return;

    logger.info('Stopping WorldManager...');

    // Stop world service
    await this.worldService.stop();

    // Stop consequences service
    await this.consequencesService.stop();

    this.isRunning = false;
    logger.info('WorldManager stopped');

    this.emit('stopped');
  }

  /**
   * Move agent to new location
   */
  async moveAgent(agentId: string, targetLocationId: string, purpose?: string): Promise<void> {
    try {
      // Get current location for memory formation
      const currentEmbodiment = this.embodimentService.getEmbodimentState(agentId);
      const fromLocationId = currentEmbodiment?.locationId;

      // Move agent
      await this.embodimentService.moveAgent(agentId, targetLocationId, purpose);

      // Process location change memory
      if (fromLocationId && fromLocationId !== targetLocationId) {
        await this.memoryService.processWorldExperience(
          agentId,
          'LOCATION_CHANGE',
          {
            fromLocation: fromLocationId,
            toLocation: targetLocationId,
            purpose
          }
        );
      }

      // Process location consequences
      await this.consequencesService.processLocationConsequences(
        agentId,
        targetLocationId,
        0 // Just arrived, duration 0
      );

    } catch (error) {
      logger.error(`Failed to move agent ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Create environmental event
   */
  async createEnvironmentalEvent(
    locationId: string,
    type: string,
    intensity: number,
    effects: Record<string, number>,
    duration: number,
    description: string
  ): Promise<void> {
    try {
      // Create environmental state
      const stateId = await this.worldService.createEnvironmentalState(
        locationId,
        type,
        intensity,
        effects,
        duration,
        description
      );

      // Get agents in location
      const agentsInLocation = await this.getAgentsInLocation(locationId);

      // Process consequences for each agent
      for (const agentId of agentsInLocation) {
        await this.consequencesService.processEnvironmentalConsequences(
          agentId,
          locationId,
          {
            stateId,
            type,
            intensity,
            effects,
            duration
          }
        );

        // Process memory formation
        await this.memoryService.processWorldExperience(
          agentId,
          'ENVIRONMENTAL_EXPOSURE',
          {
            locationId,
            environmentalState: {
              stateId,
              type,
              intensity,
              effects,
              sensoryDescription: description
            },
            duration: 0
          }
        );
      }

    } catch (error) {
      logger.error('Failed to create environmental event:', error);
      throw error;
    }
  }

  /**
   * Get complete world state
   */
  async getWorldState(): Promise<WorldState> {
    try {
      // Get world time
      const worldTimeState = await this.worldService.getWorldTimeState();

      // Get all locations
      const locations = await this.worldService.getAllLocations();

      // Get all agent states
      const agents: WorldState['agents'] = {};
      const agentIds = ['gem-d', 'gem-k'];

      for (const agentId of agentIds) {
        const embodiment = this.embodimentService.getEmbodimentState(agentId);
        const sensoryInputs = await this.worldService.getAgentSensoryInputs(agentId, 5);
        const activeConsequences = this.consequencesService.getActiveConsequences(agentId);

        if (embodiment) {
          const location = locations.find(l => l.locationId === embodiment.locationId);
          
          agents[agentId] = {
            locationId: embodiment.locationId,
            locationName: location?.name || 'Unknown',
            embodiment,
            sensoryInputs,
            activeConsequences
          };
        }
      }

      // Format locations
      const formattedLocations: WorldState['locations'] = {};
      for (const location of locations) {
        // Get current occupancy
        const occupants = Object.values(agents).filter(a => a.locationId === location.locationId);
        
        formattedLocations[location.locationId] = {
          name: location.name,
          type: location.type,
          environmentalStates: location.environmentalStates,
          occupancy: occupants.length
        };
      }

      return {
        worldTime: {
          current: worldTimeState.worldTime,
          dayNumber: worldTimeState.dayNumber,
          hourOfDay: worldTimeState.hourOfDay,
          dayPhase: worldTimeState.dayPhase
        },
        agents,
        locations: formattedLocations
      };

    } catch (error) {
      logger.error('Failed to get world state:', error);
      throw error;
    }
  }

  /**
   * Get agent's world context
   */
  async getAgentWorldContext(agentId: string): Promise<any> {
    try {
      const embodiment = this.embodimentService.getEmbodimentState(agentId);
      if (!embodiment) return null;

      const locationState = await this.worldService.getAgentLocationState(agentId);
      const sensoryInputs = await this.worldService.getAgentSensoryInputs(agentId, 10);
      const activeConsequences = this.consequencesService.getActiveConsequences(agentId);
      const locationAssociations = await this.memoryService.getLocationAssociations(agentId);

      return {
        agentId,
        location: locationState,
        embodiment,
        sensoryInputs,
        activeConsequences,
        locationAssociations
      };

    } catch (error) {
      logger.error(`Failed to get world context for ${agentId}:`, error);
      return null;
    }
  }

  /**
   * Process world event
   */
  async processWorldEvent(
    eventId: string,
    eventType: string,
    description: string,
    severity: number,
    affectedLocations: string[]
  ): Promise<void> {
    try {
      // Create world event record
      await this.db.query(`
        INSERT INTO world_events (
          event_id, title, description, event_type, severity,
          affected_locations, start_time
        ) VALUES ($1, $2, $3, $4, $5, $6, NOW())
      `, [
        eventId,
        `Event: ${eventType}`,
        description,
        eventType,
        severity,
        JSON.stringify(affectedLocations)
      ]);

      // Process effects for agents in affected locations
      for (const locationId of affectedLocations) {
        const agentsInLocation = await this.getAgentsInLocation(locationId);
        
        for (const agentId of agentsInLocation) {
          // Process memory
          await this.memoryService.processWorldExperience(
            agentId,
            'WORLD_EVENT',
            {
              eventId,
              eventType,
              description,
              severity
            }
          );
        }
      }

    } catch (error) {
      logger.error('Failed to process world event:', error);
    }
  }

  /**
   * Private methods
   */

  private async runWorldMigration(): Promise<void> {
    try {
      // Check if world tables exist
      const result = await this.db.query(`
        SELECT EXISTS (
          SELECT FROM information_schema.tables 
          WHERE table_name = 'world_config'
        )
      `);

      if (!result.rows[0].exists) {
        logger.info('Running world system migration...');
        
        // Read and execute migration
        const fs = require('fs').promises;
        const path = require('path');
        
        const migrationPath = path.join(__dirname, '../../../migrations/034_comprehensive_world_system.sql');
        const migrationSQL = await fs.readFile(migrationPath, 'utf8');
        
        await this.db.query(migrationSQL);
        logger.info('World system migration completed');
      }

    } catch (error) {
      logger.error('Failed to run world migration:', error);
      throw error;
    }
  }

  private async initializeAgentEmbodiments(): Promise<void> {
    try {
      const agentIds = ['gem-d', 'gem-k'];
      
      for (const agentId of agentIds) {
        await this.embodimentService.initializeEmbodiment(agentId);
      }

      logger.info('Agent embodiments initialized');

    } catch (error) {
      logger.error('Failed to initialize agent embodiments:', error);
    }
  }

  private async updateAllAgentEmbodiments(): Promise<void> {
    try {
      const agentIds = ['gem-d', 'gem-k'];
      
      for (const agentId of agentIds) {
        await this.embodimentService.updateEmbodimentFromWorld(agentId);
      }

    } catch (error) {
      logger.error('Failed to update agent embodiments:', error);
    }
  }

  private async processEnvironmentalEffects(): Promise<void> {
    try {
      // Get all active environmental states
      const result = await this.db.query(`
        SELECT es.*, wl.name as location_name
        FROM environmental_states es
        JOIN world_locations wl ON es.location_id = wl.location_id
        WHERE es.is_active = true
      `);

      for (const state of result.rows) {
        const agentsInLocation = await this.getAgentsInLocation(state.location_id);
        
        for (const agentId of agentsInLocation) {
          // Process consequences
          await this.consequencesService.processEnvironmentalConsequences(
            agentId,
            state.location_id,
            state
          );
        }
      }

    } catch (error) {
      logger.error('Failed to process environmental effects:', error);
    }
  }

  private async getAgentsInLocation(locationId: string): Promise<string[]> {
    const result = await this.db.query(`
      SELECT agent_id FROM agent_world_positions
      WHERE location_id = $1
    `, [locationId]);

    return result.rows.map(row => row.agent_id);
  }

  private setupServiceEventHandlers(): void {
    // World service events
    this.worldService.on('timeAdvanced', (worldTimeState) => {
      this.emit('worldTimeAdvanced', worldTimeState);
    });

    this.worldService.on('environmentalStateChanged', (data) => {
      this.emit('environmentalStateChanged', data);
    });

    // Embodiment service events
    this.embodimentService.on('embodimentUpdated', (data) => {
      this.emit('embodimentUpdated', data);
      
      // Broadcast via WebSocket for real-time UI visibility
      const channelManager = global.channelManager;
      if (channelManager) {
        channelManager.emit('embodiment_updated', {
          type: 'embodiment_update',
          agentId: data.agentId,
          state: data.state,
          effects: data.effects,
          timestamp: new Date().toISOString()
        });
      }
    });

    this.embodimentService.on('sensoryProcessed', (data) => {
      this.emit('sensoryProcessed', data);
      
      // Broadcast via WebSocket for real-time UI visibility
      const channelManager = global.channelManager;
      if (channelManager) {
        channelManager.emit('sensory_processed', {
          type: 'sensory_update',
          agentId: data.agentId,
          modality: data.modality,
          intensity: data.intensity,
          newState: data.newState,
          timestamp: new Date().toISOString()
        });
      }
    });

    // Memory service events
    this.memoryService.on('memoryFormed', (data) => {
      this.emit('memoryFormed', data);
    });

    this.memoryService.on('identityShifted', (data) => {
      this.emit('identityShifted', data);
    });

    // Consequences service events
    this.consequencesService.on('consequenceApplied', (data) => {
      this.emit('consequenceApplied', data);
    });

    this.consequencesService.on('identityShifted', (data) => {
      this.emit('identityShifted', data);
    });
  }
}
