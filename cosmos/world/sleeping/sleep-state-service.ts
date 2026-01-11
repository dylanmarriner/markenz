import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { AgentSleepingState } from '../types/world.types';

export class SleepStateService {
  constructor(private db: Pool) {}

  /**
   * Start sleeping for an agent
   * Validates permissions and bed capacity
   */
  async startSleeping(
    agentId: string,
    locationId: string,
    objectId?: string,
    isVoluntary: boolean = true,
    reason?: string
  ): Promise<{ success: boolean; message: string }> {
    const client = await this.db.connect();
    
    try {
      await client.query('BEGIN');
      
      // Validate location allows sleeping
      const locationCheck = await client.query(`
        SELECT sleep_allowed, max_occupancy 
        FROM world_locations 
        WHERE location_id = $1 AND is_active = true
      `, [locationId]);
      
      if (locationCheck.rows.length === 0) {
        await client.query('ROLLBACK');
        return { success: false, message: 'Invalid location' };
      }
      
      const location = locationCheck.rows[0];
      if (!location.sleep_allowed) {
        await client.query('ROLLBACK');
        return { success: false, message: 'Location does not allow sleeping' };
      }
      
      // Check if object exists and is a bed (if specified)
      if (objectId) {
        const objectCheck = await client.query(`
          SELECT type, subtype, capacity, is_usable 
          FROM world_objects 
          WHERE object_id = $1 AND is_active = true
        `, [objectId]);
        
        if (objectCheck.rows.length === 0) {
          await client.query('ROLLBACK');
          return { success: false, message: 'Invalid object' };
        }
        
        const object = objectCheck.rows[0];
        if (object.type !== 'bed') {
          await client.query('ROLLBACK');
          return { success: false, message: 'Object is not a bed' };
        }
        
        if (!object.is_usable) {
          await client.query('ROLLBACK');
          return { success: false, message: 'Bed is not usable' };
        }
        
        // Check bed capacity for king beds
        if (object.capacity) {
          const currentSleepers = await client.query(`
            SELECT COUNT(*) as count
            FROM agent_sleeping_states
            WHERE object_id = $1 AND is_sleeping = true
          `, [objectId]);
          
          if (currentSleepers.rows[0].count >= object.capacity) {
            await client.query('ROLLBACK');
            return { 
              success: false, 
              message: `Bed is at capacity (${object.capacity}/${object.capacity})` 
            };
          }
        }
      }
      
      // Check agent isn't already sleeping
      const existingSleep = await client.query(`
        SELECT is_sleeping FROM agent_sleeping_states WHERE agent_id = $1
      `, [agentId]);
      
      if (existingSleep.rows.length > 0 && existingSleep.rows[0].is_sleeping) {
        await client.query('ROLLBACK');
        return { success: false, message: 'Agent is already sleeping' };
      }
      
      // Get agent's current position for history
      const positionResult = await client.query(`
        SELECT coordinates FROM agent_world_positions WHERE agent_id = $1
      `, [agentId]);
      
      const lastPosition = positionResult.rows[0]?.coordinates || null;
      
      // Start sleeping
      await client.query(`
        INSERT INTO agent_sleeping_states (
          agent_id,
          location_id,
          object_id,
          sleep_start,
          is_sleeping,
          is_voluntary,
          reason,
          last_position
        ) VALUES (
          $1, $2, $3, NOW(), true, $4, $5, $6
        ) ON CONFLICT (agent_id) DO UPDATE SET
          location_id = EXCLUDED.location_id,
          object_id = EXCLUDED.object_id,
          sleep_start = NOW(),
          is_sleeping = true,
          is_voluntary = EXCLUDED.is_voluntary,
          reason = EXCLUDED.reason,
          last_position = EXCLUDED.last_position,
          updated_at = NOW()
      `, [agentId, locationId, objectId, isVoluntary, reason || null, lastPosition]);
      
      // Update agent activity
      await client.query(`
        UPDATE agent_world_positions SET
          last_activity = 'sleeping',
          last_state_change = NOW()
        WHERE agent_id = $1
      `, [agentId]);
      
      await client.query('COMMIT');
      
      logger.info(`Agent ${agentId} started sleeping in ${locationId}${objectId ? ` on ${objectId}` : ''}`);
      
      return { 
        success: true, 
        message: `Started sleeping in ${locationId}` 
      };
      
    } catch (error) {
      await client.query('ROLLBACK');
      logger.error(`Failed to start sleeping for ${agentId}:`, error);
      return { 
        success: false, 
        message: `Failed to start sleeping: ${error.message}` 
      };
    } finally {
      client.release();
    }
  }

  /**
   * Stop sleeping for an agent
   */
  async stopSleeping(agentId: string): Promise<{ success: boolean; message: string }> {
    const client = await this.db.connect();
    
    try {
      await client.query('BEGIN');
      
      // Get current sleep state
      const sleepResult = await client.query(`
        SELECT location_id, object_id, sleep_start, is_voluntary, reason
        FROM agent_sleeping_states
        WHERE agent_id = $1 AND is_sleeping = true
      `, [agentId]);
      
      if (sleepResult.rows.length === 0) {
        await client.query('ROLLBACK');
        return { success: false, message: 'Agent is not sleeping' };
      }
      
      const sleepState = sleepResult.rows[0];
      const sleepEnd = new Date();
      const durationSeconds = Math.floor(
        (sleepEnd.getTime() - sleepState.sleep_start.getTime()) / 1000
      );
      
      // Update sleep state
      await client.query(`
        UPDATE agent_sleeping_states SET
          is_sleeping = false,
          sleep_end = NOW(),
          updated_at = NOW()
        WHERE agent_id = $1
      `, [agentId]);
      
      // Add to sleep history
      await client.query(`
        INSERT INTO agent_sleep_history (
          agent_id,
          location_id,
          object_id,
          sleep_start,
          sleep_end,
          duration_seconds,
          reason
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7
        )
      `, [
        agentId,
        sleepState.location_id,
        sleepState.object_id,
        sleepState.sleep_start,
        sleepEnd,
        durationSeconds,
        sleepState.reason
      ]);
      
      // Update agent activity
      await client.query(`
        UPDATE agent_world_positions SET
          last_activity = 'awake',
          last_state_change = NOW()
        WHERE agent_id = $1
      `, [agentId]);
      
      await client.query('COMMIT');
      
      logger.info(`Agent ${agentId} stopped sleeping after ${durationSeconds}s`);
      
      return { 
        success: true, 
        message: `Stopped sleeping (slept for ${Math.floor(durationSeconds / 60)} minutes)` 
      };
      
    } catch (error) {
      await client.query('ROLLBACK');
      logger.error(`Failed to stop sleeping for ${agentId}:`, error);
      return { 
        success: false, 
        message: `Failed to stop sleeping: ${error.message}` 
      };
    } finally {
      client.release();
    }
  }

  /**
   * Get agent's current sleep state
   */
  async getSleepState(agentId: string): Promise<AgentSleepingState | null> {
    const result = await this.db.query(`
      SELECT 
        agent_id,
        location_id,
        object_id,
        sleep_start,
        sleep_end,
        is_sleeping,
        is_voluntary,
        comfort_level,
        noise_level,
        temperature_comfort,
        reason,
        last_position,
        created_at,
        updated_at
      FROM agent_sleeping_states
      WHERE agent_id = $1
    `, [agentId]);
    
    if (result.rows.length === 0) {
      return null;
    }
    
    return result.rows[0];
  }

  /**
   * Get all currently sleeping agents
   */
  async getSleepingAgents(): Promise<Array<{
    agent_id: string;
    location_id: string;
    object_id?: string;
    sleep_start: Date;
    location_name: string;
    object_name?: string;
    bed_type?: string;
    hours_sleeping: number;
  }>> {
    const result = await this.db.query(`
      SELECT 
        ass.agent_id,
        ass.location_id,
        ass.object_id,
        ass.sleep_start,
        wl.name as location_name,
        wo.name as object_name,
        wo.subtype as bed_type,
        EXTRACT(EPOCH FROM (NOW() - ass.sleep_start))/3600 as hours_sleeping
      FROM agent_sleeping_states ass
      JOIN world_locations wl ON wl.location_id = ass.location_id
      LEFT JOIN world_objects wo ON wo.object_id = ass.object_id
      WHERE ass.is_sleeping = true
      ORDER BY ass.sleep_start DESC
    `);
    
    return result.rows;
  }

  /**
   * Check if agents can share a bed
   */
  async canShareBed(objectId: string): Promise<{
    canShare: boolean;
    currentOccupancy: number;
    maxCapacity: number;
    bedType: string;
  }> {
    const result = await this.db.query(`
      SELECT 
        wo.capacity,
        wo.subtype,
        COUNT(ass.agent_id) as current_occupancy
      FROM world_objects wo
      LEFT JOIN agent_sleeping_states ass ON ass.object_id = wo.object_id AND ass.is_sleeping = true
      WHERE wo.object_id = $1
      GROUP BY wo.capacity, wo.subtype
    `, [objectId]);
    
    if (result.rows.length === 0) {
      return { canShare: false, currentOccupancy: 0, maxCapacity: 0, bedType: 'unknown' };
    }
    
    const row = result.rows[0];
    const canShare = row.current_occupancy < row.capacity;
    
    return {
      canShare,
      currentOccupancy: row.current_occupancy,
      maxCapacity: row.capacity,
      bedType: row.subtype
    };
  }

  /**
   * Update sleep quality metrics
   */
  async updateSleepMetrics(
    agentId: string,
    metrics: {
      comfort_level?: number;
      noise_level?: number;
      temperature_comfort?: number;
    }
  ): Promise<void> {
    await this.db.query(`
      UPDATE agent_sleeping_states SET
        comfort_level = COALESCE($1, comfort_level),
        noise_level = COALESCE($2, noise_level),
        temperature_comfort = COALESCE($3, temperature_comfort),
        updated_at = NOW()
      WHERE agent_id = $4 AND is_sleeping = true
    `, [
      metrics.comfort_level,
      metrics.noise_level,
      metrics.temperature_comfort,
      agentId
    ]);
  }

  /**
   * Get sleep history for an agent
   */
  async getSleepHistory(
    agentId: string, 
    limit: number = 30,
    offset: number = 0
  ): Promise<any[]> {
    const result = await this.db.query(`
      SELECT 
        ash.*,
        wl.name as location_name,
        wo.name as object_name,
        wo.subtype as bed_type
      FROM agent_sleep_history ash
      LEFT JOIN world_locations wl ON wl.location_id = ash.location_id
      LEFT JOIN world_objects wo ON wo.object_id = ash.object_id
      WHERE ash.agent_id = $1
      ORDER BY ash.sleep_start DESC
      LIMIT $2 OFFSET $3
    `, [agentId, limit, offset]);
    
    return result.rows;
  }

  /**
   * Force wake all sleeping agents (for emergency/server stop)
   */
  async forceWakeAll(): Promise<number> {
    const result = await this.db.query(`
      UPDATE agent_sleeping_states SET
        is_sleeping = false,
        sleep_end = NOW(),
        updated_at = NOW()
      WHERE is_sleeping = true
      RETURNING agent_id
    `);
    
    logger.warn(`Force woke ${result.rows.length} sleeping agents`);
    return result.rows.length;
  }

  /**
   * Restore sleeping state on server boot
   * This is called during server initialization to restore exact state
   */
  async restoreSleepingState(): Promise<void> {
    const sleepingAgents = await this.db.query(`
      SELECT agent_id, location_id, object_id, sleep_start
      FROM agent_sleeping_states
      WHERE is_sleeping = true
    `);
    
    if (sleepingAgents.rows.length > 0) {
      logger.info(`Restoring sleep state for ${sleepingAgents.rows.length} agents`);
      
      for (const agent of sleepingAgents.rows) {
        logger.debug(`Agent ${agent.agent_id} is sleeping in ${agent.location_id}`);
      }
    }
  }

  /**
   * Get sleep statistics
   */
  async getSleepStatistics(agentId?: string): Promise<{
    totalSleepSessions: number;
    averageSleepDuration: number;
    mostUsedLocation: string;
    favoriteBedType: string;
  }> {
    let whereClause = '';
    let params: any[] = [];
    
    if (agentId) {
      whereClause = 'WHERE ash.agent_id = $1';
      params = [agentId];
    }
    
    const result = await this.db.query(`
      SELECT 
        COUNT(*) as total_sessions,
        AVG(duration_seconds) as avg_duration,
        mode() WITHIN GROUP (ORDER BY ash.location_id) as most_common_location,
        mode() WITHIN GROUP (ORDER BY wo.subtype) as favorite_bed_type
      FROM agent_sleep_history ash
      LEFT JOIN world_objects wo ON wo.object_id = ash.object_id
      ${whereClause}
    `, params);
    
    const row = result.rows[0];
    
    return {
      totalSleepSessions: parseInt(row.total_sessions) || 0,
      averageSleepDuration: Math.floor(row.avg_duration / 60) || 0, // in minutes
      mostUsedLocation: row.most_common_location || 'none',
      favoriteBedType: row.favorite_bed_type || 'none'
    };
  }
}
