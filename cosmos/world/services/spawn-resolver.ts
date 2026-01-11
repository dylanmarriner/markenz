import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { SpawnLocation, SpawnReason, AgentWorldPosition } from '../types/world.types';

export class SpawnResolver {
  constructor(private db: Pool) {}

  /**
   * Resolve agent spawn location based on world state
   * Separates spawn logic from sleeping state
   */
  async resolveSpawnLocation(agentId: string): Promise<SpawnLocation> {
    try {
      // Check if agent has existing world position
      const existingPosition = await this.getAgentWorldPosition(agentId);
      
      if (existingPosition) {
        // Subsequent boot - spawn at last known position
        const spawnLocation: SpawnLocation = {
          location_id: existingPosition.location_id,
          coordinates: existingPosition.coordinates,
          rotation: existingPosition.rotation,
          reason: {
            type: 'REHYDRATED',
            description: 'Restored from previous session'
          }
        };
        
        logger.info(`Agent ${agentId} rehydrated at ${existingPosition.location_id}`);
        return spawnLocation;
      }
      
      // First-ever spawn - use genesis logic
      const genesisLocation = await this.getGenesisLocation(agentId);
      
      logger.info(`Agent ${agentId} genesis spawn at ${genesisLocation.location_id}`);
      return genesisLocation;
      
    } catch (error) {
      logger.error(`Failed to resolve spawn location for ${agentId}:`, error);
      // Fallback to lounge
      return {
        location_id: 'lounge',
        coordinates: { x: 515, y: 395, z: 0 },
        rotation: 0,
        reason: {
          type: 'REHYDRATED',
          description: 'Fallback spawn due to error'
        }
      };
    }
  }

  /**
   * Get agent's last known world position
   */
  async getAgentWorldPosition(agentId: string): Promise<AgentWorldPosition | null> {
    const result = await this.db.query(`
      SELECT 
        location_id,
        coordinates,
        rotation,
        vehicle_id,
        is_mounted,
        last_activity
      FROM agent_world_positions
      WHERE agent_id = $1
    `, [agentId]);
    
    if (result.rows.length === 0) {
      return null;
    }
    
    const row = result.rows[0];
    return {
      agent_id: agentId,
      location_id: row.location_id,
      coordinates: row.coordinates,
      rotation: row.rotation,
      is_moving: false,
      vehicle_id: row.vehicle_id,
      is_mounted: row.is_mounted,
      last_activity: row.last_activity,
      last_state_change: new Date()
    };
  }

  /**
   * Determine genesis spawn location based on agent identity
   * CRITICAL: This does NOT consider sleeping state
   */
  private async getGenesisLocation(agentId: string): Promise<SpawnLocation> {
    // Twin agents spawn in their respective rooms
    if (agentId === 'gem-d') {
      return {
        location_id: 'twin_room_a',
        coordinates: { x: 514, y: 413, z: 0 },
        rotation: 0,
        reason: {
          type: 'GENESIS',
          description: 'First spawn - Gem-D in Twin Room A'
        }
      };
    }
    
    if (agentId === 'gem-k') {
      return {
        location_id: 'twin_room_b',
        coordinates: { x: 524, y: 413, z: 0 },
        rotation: 0,
        reason: {
          type: 'GENESIS',
          description: 'First spawn - Gem-K in Twin Room B'
        }
      };
    }
    
    // Default for any other agents
    return {
      location_id: 'lounge',
      coordinates: { x: 515, y: 395, z: 0 },
      rotation: 0,
      reason: {
        type: 'GENESIS',
        description: 'First spawn - default location'
      }
    };
  }

  /**
   * Persist agent spawn location
   */
  async persistAgentPosition(
    agentId: string, 
    locationId: string, 
    coordinates: { x: number; y: number; z: number },
    rotation: number = 0
  ): Promise<void> {
    try {
      await this.db.query(`
        INSERT INTO agent_world_positions (
          agent_id,
          location_id,
          coordinates,
          rotation,
          is_moving,
          last_activity,
          last_state_change
        ) VALUES (
          $1, $2, $3, $4, $5, $6, NOW()
        ) ON CONFLICT (agent_id) DO UPDATE SET
          location_id = EXCLUDED.location_id,
          coordinates = EXCLUDED.coordinates,
          rotation = EXCLUDED.rotation,
          last_state_change = NOW()
      `, [
        agentId,
        locationId,
        JSON.stringify(coordinates),
        rotation,
        false,
        'spawned'
      ]);
      
      logger.debug(`Persisted position for ${agentId}: ${locationId}`);
    } catch (error) {
      logger.error(`Failed to persist position for ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Update agent position during movement
   */
  async updateAgentPosition(
    agentId: string,
    locationId: string,
    coordinates: { x: number; y: number; z: number },
    isMoving: boolean = false,
    targetLocation?: string,
    targetCoordinates?: { x: number; y: number; z: number },
    vehicleId?: string,
    isMounted: boolean = false
  ): Promise<void> {
    try {
      await this.db.query(`
        UPDATE agent_world_positions SET
          location_id = $1,
          coordinates = $2,
          is_moving = $3,
          target_location = $4,
          target_coordinates = $5,
          move_start_time = CASE 
            WHEN $3 = true THEN NOW()
            ELSE move_start_time
          END,
          estimated_arrival = CASE 
            WHEN $3 = false THEN NULL
            ELSE estimated_arrival
          END,
          vehicle_id = $6,
          is_mounted = $7,
          last_state_change = NOW()
        WHERE agent_id = $8
      `, [
        locationId,
        JSON.stringify(coordinates),
        isMoving,
        targetLocation || null,
        targetCoordinates ? JSON.stringify(targetCoordinates) : null,
        vehicleId || null,
        isMounted,
        agentId
      ]);
      
      logger.debug(`Updated position for ${agentId}: ${locationId} (moving: ${isMoving})`);
    } catch (error) {
      logger.error(`Failed to update position for ${agentId}:`, error);
      throw error;
    }
  }

  /**
   * Get all agent positions for world state
   */
  async getAllAgentPositions(): Promise<AgentWorldPosition[]> {
    const result = await this.db.query(`
      SELECT 
        agent_id,
        location_id,
        coordinates,
        rotation,
        is_moving,
        target_location,
        target_coordinates,
        move_start_time,
        estimated_arrival,
        vehicle_id,
        is_mounted,
        mount_position,
        last_activity,
        last_state_change
      FROM agent_world_positions
      ORDER BY last_state_change DESC
    `);
    
    return result.rows.map(row => ({
      agent_id: row.agent_id,
      location_id: row.location_id,
      coordinates: row.coordinates,
      rotation: row.rotation,
      is_moving: row.is_moving,
      target_location: row.target_location,
      target_coordinates: row.target_coordinates,
      move_start_time: row.move_start_time,
      estimated_arrival: row.estimated_arrival,
      vehicle_id: row.vehicle_id,
      is_mounted: row.is_mounted,
      mount_position: row.mount_position,
      last_activity: row.last_activity,
      last_state_change: row.last_state_change
    }));
  }

  /**
   * Check if agent is at spawn location
   */
  async isAtSpawnLocation(agentId: string): Promise<boolean> {
    const currentPos = await this.getAgentWorldPosition(agentId);
    if (!currentPos) return false;
    
    const spawnLoc = await this.getGenesisLocation(agentId);
    
    return (
      currentPos.location_id === spawnLoc.location_id &&
      Math.abs(currentPos.coordinates.x - spawnLoc.coordinates.x) < 1 &&
      Math.abs(currentPos.coordinates.y - spawnLoc.coordinates.y) < 1 &&
      Math.abs(currentPos.coordinates.z - spawnLoc.coordinates.z) < 1
    );
  }

  /**
   * Get spawn history for analytics
   */
  async getSpawnHistory(agentId: string, limit: number = 10): Promise<any[]> {
    // This would require a spawn history table if needed
    // For now, return current position info
    const position = await this.getAgentWorldPosition(agentId);
    
    return position ? [{
      agent_id: agentId,
      location_id: position.location_id,
      coordinates: position.coordinates,
      timestamp: position.last_state_change,
      type: 'current_position'
    }] : [];
  }
}
