import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { SpawnResolver } from './SpawnResolver';
import { SleepStateService } from '../sleeping/SleepStateService';

import { PhysWorld } from '../physworld/index';

export interface WorldStateSnapshot {
  timestamp: Date;
  agents: Array<{
    agent_id: string;
    location_id: string;
    coordinates: { x: number; y: number; z: number };
    rotation: number;
    is_moving: boolean;
    vehicle_id?: string;
    is_mounted: boolean;
    last_activity: string;
  }>;
  sleepingAgents: Array<{
    agent_id: string;
    location_id: string;
    object_id?: string;
    sleep_start: Date;
    is_voluntary: boolean;
  }>;
  vehicles: Array<{
    vehicle_id: string;
    location_id: string;
    coordinates: { x: number; y: number; z: number };
    fuel_level: number;
    is_mounted: boolean;
    mounted_agent?: string;
    inventory: any[];
  }>;
  resources: Array<{
    resource_id: string;
    quantity: number;
    is_depleted: boolean;
    last_mined?: Date;
  }>;
}

export class WorldStatePersistenceService {
  constructor(
    private db: Pool,
    private spawnResolver: SpawnResolver,
    private sleepStateService: SleepStateService
  ) {}

  /**
   * Save complete world state snapshot
   * Called on clean shutdown, disconnect, or heartbeat timeout
   */
  async saveWorldState(reason: string = 'manual'): Promise<{ success: boolean; message: string }> {
    try {
      const snapshot = await this.captureWorldState();
      
      // Save to persistent storage (database)
      const client = await this.db.connect();
      
      try {
        await client.query('BEGIN');
        
        // Save world state snapshot to log table
        await client.query(`
          INSERT INTO world_state_snapshots (
            snapshot_data,
            save_reason,
            save_timestamp
          ) VALUES (
            $1, $2, NOW()
          )
        `, [JSON.stringify(snapshot), reason]);
        
        // Update last save timestamp
        await client.query(`
          UPDATE world_config 
          SET last_state_save = NOW(),
              save_count = COALESCE(save_count, 0) + 1
          WHERE id = 1
        `);
        
        await client.query('COMMIT');
        
        logger.info(`World state saved: ${reason}`, {
          agents: snapshot.agents.length,
          sleeping: snapshot.sleepingAgents.length,
          vehicles: snapshot.vehicles.length,
          resources: snapshot.resources.length
        });
        
        return {
          success: true,
          message: `World state saved successfully (${reason})`
        };
        
      } catch (error) {
        await client.query('ROLLBACK');
        throw error;
      } finally {
        client.release();
      }
      
    } catch (error) {
      logger.error('Failed to save world state:', error);
      return {
        success: false,
        message: `Failed to save world state: ${error.message}`
      };
    }
  }

  /**
   * Rehydrate world state on server boot
   * Restores exact previous state without modifications
   */
  async rehydrateWorldState(): Promise<{ success: boolean; message: string; restored: any }> {
    try {
      logger.info('Starting world state rehydration...');
      
      // Get latest world state snapshot
      const latestSnapshot = await this.getLatestSnapshot();
      
      if (!latestSnapshot) {
        logger.info('No previous world state found - starting fresh');
        return {
          success: true,
          message: 'No previous state to restore',
          restored: null
        };
      }
      
      // Restore agent positions
      const restoredAgents = await this.restoreAgentPositions(latestSnapshot.agents);
      
      // Restore sleeping states (critical - must preserve exact state)
      await this.sleepStateService.restoreSleepingState();
      
      // Restore vehicle states
      const restoredVehicles = await this.restoreVehicleStates(latestSnapshot.vehicles);
      
      // Restore resource states
      const restoredResources = await this.restoreResourceStates(latestSnapshot.resources);
      
      const restored = {
        agents: restoredAgents,
        vehicles: restoredVehicles,
        resources: restoredResources,
        timestamp: latestSnapshot.timestamp
      };
      
      logger.info('World state rehydrated successfully', restored);
      
      return {
        success: true,
        message: 'World state rehydrated successfully',
        restored
      };
      
    } catch (error) {
      logger.error('Failed to rehydrate world state:', error);
      return {
        success: false,
        message: `Failed to rehydrate world state: ${error.message}`,
        restored: null
      };
    }
  }

  /**
   * Capture current world state
   */
  private async captureWorldState(): Promise<WorldStateSnapshot> {
    const client = await this.db.connect();
    
    try {
      // Get all agent positions
      const agentsResult = await client.query(`
        SELECT 
          agent_id,
          location_id,
          coordinates,
          rotation,
          is_moving,
          target_location,
          vehicle_id,
          is_mounted,
          last_activity
        FROM agent_world_positions
        ORDER BY agent_id
      `);
      
      // Get sleeping agents
      const sleepingResult = await client.query(`
        SELECT 
          agent_id,
          location_id,
          object_id,
          sleep_start,
          is_voluntary
        FROM agent_sleeping_states
        WHERE is_sleeping = true
        ORDER BY sleep_start
      `);
      
      // Get vehicle states
      const vehiclesResult = await client.query(`
        SELECT 
          vehicle_id,
          location_id,
          coordinates,
          rotation,
          fuel_level,
          is_mounted,
          mounted_agent,
          inventory
        FROM world_vehicles
        WHERE is_active = true
        ORDER BY vehicle_id
      `);
      
      // Get resource states
      const resourcesResult = await client.query(`
        SELECT 
          resource_id,
          quantity,
          is_depleted,
          last_mined
        FROM world_resources
        ORDER BY resource_id
      `);
      
      return {
        timestamp: new Date(),
        agents: agentsResult.rows,
        sleepingAgents: sleepingResult.rows,
        vehicles: vehiclesResult.rows,
        resources: resourcesResult.rows
      };
      
    } finally {
      client.release();
    }
  }

  /**
   * Get latest world state snapshot
   */
  private async getLatestSnapshot(): Promise<WorldStateSnapshot | null> {
    const result = await this.db.query(`
      SELECT snapshot_data
      FROM world_state_snapshots
      ORDER BY save_timestamp DESC
      LIMIT 1
    `);
    
    if (result.rows.length === 0) {
      return null;
    }
    
    return JSON.parse(result.rows[0].snapshot_data);
  }

  /**
   * Restore agent positions
   */
  private async restoreAgentPositions(agents: any[]): Promise<number> {
    let restored = 0;
    
    for (const agent of agents) {
      try {
        await this.spawnResolver.persistAgentPosition(
          agent.agent_id,
          agent.location_id,
          agent.coordinates,
          agent.rotation
        );
        
        // Update additional state if moving
        if (agent.is_moving) {
          await this.spawnResolver.updateAgentPosition(
            agent.agent_id,
            agent.location_id,
            agent.coordinates,
            agent.is_moving,
            agent.target_location,
            null, // target coordinates not saved
            agent.vehicle_id,
            agent.is_mounted
          );
        }
        
        restored++;
      } catch (error) {
        logger.error(`Failed to restore position for ${agent.agent_id}:`, error);
      }
    }
    
    return restored;
  }

  /**
   * Restore vehicle states
   */
  private async restoreVehicleStates(vehicles: any[]): Promise<number> {
    let restored = 0;
    
    for (const vehicle of vehicles) {
      try {
        await this.db.query(`
          UPDATE world_vehicles SET
            location_id = $1,
            coordinates = $2,
            rotation = $3,
            fuel_level = $4,
            is_mounted = $5,
            mounted_agent = $6,
            inventory = $7,
            updated_at = NOW()
          WHERE vehicle_id = $8
        `, [
          vehicle.location_id,
          JSON.stringify(vehicle.coordinates),
          vehicle.rotation,
          vehicle.fuel_level,
          vehicle.is_mounted,
          vehicle.mounted_agent || null,
          JSON.stringify(vehicle.inventory || []),
          vehicle.vehicle_id
        ]);
        
        restored++;
      } catch (error) {
        logger.error(`Failed to restore vehicle ${vehicle.vehicle_id}:`, error);
      }
    }
    
    return restored;
  }

  /**
   * Restore resource states
   */
  private async restoreResourceStates(resources: any[]): Promise<number> {
    let restored = 0;
    
    for (const resource of resources) {
      try {
        await this.db.query(`
          UPDATE world_resources SET
            quantity = $1,
            is_depleted = $2,
            last_mined = $3,
            updated_at = NOW()
          WHERE resource_id = $4
        `, [
          resource.quantity,
          resource.is_depleted,
          resource.last_mined || null,
          resource.resource_id
        ]);
        
        restored++;
      } catch (error) {
        logger.error(`Failed to restore resource ${resource.resource_id}:`, error);
      }
    }
    
    return restored;
  }

  /**
   * Get save history
   */
  async getSaveHistory(limit: number = 10): Promise<any[]> {
    const result = await this.db.query(`
      SELECT 
        id,
        save_reason,
        save_timestamp,
        snapshot_data
      FROM world_state_snapshots
      ORDER BY save_timestamp DESC
      LIMIT $1
    `, [limit]);
    
    return result.rows.map(row => ({
      id: row.id,
      reason: row.save_reason,
      timestamp: row.save_timestamp,
      agentCount: JSON.parse(row.snapshot_data).agents.length,
      sleepingCount: JSON.parse(row.snapshot_data).sleepingAgents.length
    }));
  }

  /**
   * Auto-save on interval
   */
  startAutoSave(intervalMs: number = 300000): NodeJS.Timeout { // 5 minutes default
    logger.info(`Starting auto-save every ${intervalMs / 1000} seconds`);
    
    // DISABLED: setInterval-driven auto-save moved to PhysWorld tick chain
    console.log('✅ WorldStatePersistenceService: setInterval auto-save disabled - use PhysWorld tick integration');
    // Auto-save now handled by tickAutoSave() called from PhysWorld
    
    // Return null timer ID for compatibility
    return null as any;
  }

  // NEW: PhysWorld tick integration method for auto-save
  async tickAutoSave(): Promise<void> {
    // Auto-save every 5 minutes (300000ms)
    const tickCount = Date.now() / 1000;
    if (tickCount % 300 === 0) { // Every 300 seconds = 5 minutes
      const result = await this.saveWorldState('auto_save');
      if (!result.success) {
        logger.error('Auto-save failed:', result.message);
      }
    }
  }

  /**
   * Create world state snapshots table if not exists
   */
  async initialize(): Promise<void> {
    await this.db.query(`
      CREATE TABLE IF NOT EXISTS world_state_snapshots (
        id SERIAL PRIMARY KEY,
        snapshot_data JSONB NOT NULL,
        save_reason VARCHAR(100) NOT NULL,
        save_timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
      );
      
      CREATE INDEX IF NOT EXISTS idx_world_state_snapshots_timestamp 
      ON world_state_snapshots(save_timestamp DESC);
      
      ALTER TABLE world_config 
      ADD COLUMN IF NOT EXISTS last_state_save TIMESTAMP WITH TIME ZONE,
      ADD COLUMN IF NOT EXISTS save_count INTEGER DEFAULT 0;
    `);
    
    logger.info('World state persistence initialized');
  }
}
