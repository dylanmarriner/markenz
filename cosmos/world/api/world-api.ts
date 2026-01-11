import { Router } from 'express';
import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { SpawnResolver } from '../services/SpawnResolver';
import { SleepStateService } from '../sleeping/SleepStateService';
import { WorldStatePersistenceService } from '../services/WorldStatePersistenceService';

export function createWorldAPI(
  db: Pool,
  spawnResolver: SpawnResolver,
  sleepStateService: SleepStateService,
  worldStatePersistence: WorldStatePersistenceService
): Router {
  const router = Router();

  // Get complete world state
  router.get('/state', async (req, res) => {
    try {
      const result = await db.query(`
        SELECT * FROM current_world_state
      `);
      
      res.json({
        success: true,
        data: result.rows[0] || {},
        timestamp: new Date().toISOString()
      });
    } catch (error) {
      logger.error('Failed to get world state:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get all locations
  router.get('/locations', async (req, res) => {
    try {
      const { type } = req.query;
      
      let query = `
        SELECT 
          location_id,
          name,
          type,
          parent_location,
          coordinates,
          dimensions,
          sleep_allowed,
          max_occupancy,
          allowed_activities,
          utilities,
          affordances,
          is_active
        FROM world_locations
        WHERE is_active = true
      `;
      
      const params: any[] = [];
      
      if (type) {
        query += ' AND type = $1';
        params.push(type);
      }
      
      query += ' ORDER BY location_id';
      
      const result = await db.query(query, params);
      
      res.json({
        success: true,
        data: result.rows,
        count: result.rows.length
      });
    } catch (error) {
      logger.error('Failed to get locations:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get specific location
  router.get('/locations/:locationId', async (req, res) => {
    try {
      const { locationId } = req.params;
      
      const result = await db.query(`
        SELECT 
          wl.*,
          (SELECT JSON_AGG(
            json_build_object(
              'object_id', wo.object_id,
              'name', wo.name,
              'type', wo.type,
              'subtype', wo.subtype,
              'coordinates', wo.coordinates,
              'is_interactable', wo.is_interactable,
              'is_usable', wo.is_usable,
              'capacity', wo.capacity,
              'owner_agent', wo.owner_agent
            )
          ) FROM world_objects wo 
          WHERE wo.location_id = wl.location_id AND wo.is_active = true
          ) as objects
        FROM world_locations wl
        WHERE wl.location_id = $1 AND wl.is_active = true
      `, [locationId]);
      
      if (result.rows.length === 0) {
        return res.status(404).json({
          success: false,
          error: 'Location not found'
        });
      }
      
      res.json({
        success: true,
        data: result.rows[0]
      });
    } catch (error) {
      logger.error(`Failed to get location ${req.params.locationId}:`, error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get all vehicles
  router.get('/vehicles', async (req, res) => {
    try {
      const { type, location } = req.query;
      
      let query = `
        SELECT 
          vehicle_id,
          name,
          type,
          model,
          location_id,
          coordinates,
          rotation,
          storage_capacity,
          fuel_type,
          fuel_level,
          max_speed,
          is_mounted,
          mounted_agent,
          is_locked,
          condition,
          inventory
        FROM world_vehicles
        WHERE is_active = true
      `;
      
      const params: any[] = [];
      let paramIndex = 1;
      
      if (type) {
        query += ` AND type = $${paramIndex++}`;
        params.push(type);
      }
      
      if (location) {
        query += ` AND location_id = $${paramIndex++}`;
        params.push(location);
      }
      
      query += ' ORDER BY vehicle_id';
      
      const result = await db.query(query, params);
      
      res.json({
        success: true,
        data: result.rows,
        count: result.rows.length
      });
    } catch (error) {
      logger.error('Failed to get vehicles:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get specific vehicle
  router.get('/vehicles/:vehicleId', async (req, res) => {
    try {
      const { vehicleId } = req.params;
      
      const result = await db.query(`
        SELECT * FROM world_vehicles
        WHERE vehicle_id = $1 AND is_active = true
      `, [vehicleId]);
      
      if (result.rows.length === 0) {
        return res.status(404).json({
          success: false,
          error: 'Vehicle not found'
        });
      }
      
      res.json({
        success: true,
        data: result.rows[0]
      });
    } catch (error) {
      logger.error(`Failed to get vehicle ${req.params.vehicleId}:`, error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get all resources
  router.get('/resources', async (req, res) => {
    try {
      const { type, location, depleted } = req.query;
      
      let query = `
        SELECT 
          wr.*,
          wl.name as location_name
        FROM world_resources wr
        LEFT JOIN world_locations wl ON wl.location_id = wr.location_id
        WHERE 1=1
      `;
      
      const params: any[] = [];
      let paramIndex = 1;
      
      if (type) {
        query += ` AND wr.type = $${paramIndex++}`;
        params.push(type);
      }
      
      if (location) {
        query += ` AND wr.location_id = $${paramIndex++}`;
        params.push(location);
      }
      
      if (depleted !== undefined) {
        query += ` AND wr.is_depleted = $${paramIndex++}`;
        params.push(depleted === 'true');
      }
      
      query += ' ORDER BY wr.type, wr.resource_id';
      
      const result = await db.query(query, params);
      
      res.json({
        success: true,
        data: result.rows,
        count: result.rows.length
      });
    } catch (error) {
      logger.error('Failed to get resources:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get agent positions
  router.get('/agents/positions', async (req, res) => {
    try {
      const agents = await spawnResolver.getAllAgentPositions();
      
      res.json({
        success: true,
        data: agents,
        count: agents.length
      });
    } catch (error) {
      logger.error('Failed to get agent positions:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get specific agent position
  router.get('/agents/:agentId/position', async (req, res) => {
    try {
      const { agentId } = req.params;
      
      const position = await spawnResolver.getAgentWorldPosition(agentId);
      
      if (!position) {
        return res.status(404).json({
          success: false,
          error: 'Agent position not found'
        });
      }
      
      res.json({
        success: true,
        data: position
      });
    } catch (error) {
      logger.error(`Failed to get agent position ${req.params.agentId}:`, error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get sleeping agents
  router.get('/agents/sleeping', async (req, res) => {
    try {
      const sleepingAgents = await sleepStateService.getSleepingAgents();
      
      res.json({
        success: true,
        data: sleepingAgents,
        count: sleepingAgents.length
      });
    } catch (error) {
      logger.error('Failed to get sleeping agents:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get specific agent sleep state
  router.get('/agents/:agentId/sleeping', async (req, res) => {
    try {
      const { agentId } = req.params;
      
      const sleepState = await sleepStateService.getSleepState(agentId);
      
      if (!sleepState) {
        return res.json({
          success: true,
          data: null,
          message: 'Agent is not sleeping'
        });
      }
      
      res.json({
        success: true,
        data: sleepState
      });
    } catch (error) {
      logger.error(`Failed to get sleep state ${req.params.agentId}:`, error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get bed sharing info
  router.get('/beds/:objectId/sharing', async (req, res) => {
    try {
      const { objectId } = req.params;
      
      const sharingInfo = await sleepStateService.canShareBed(objectId);
      
      res.json({
        success: true,
        data: sharingInfo
      });
    } catch (error) {
      logger.error(`Failed to get bed sharing info ${req.params.objectId}:`, error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get world statistics
  router.get('/statistics', async (req, res) => {
    try {
      const [
        worldStats,
        sleepStats,
        vehicleStats,
        resourceStats
      ] = await Promise.all([
        db.query('SELECT * FROM current_world_state'),
        sleepStateService.getSleepStatistics(),
        db.query('SELECT type, COUNT(*) as count FROM world_vehicles WHERE is_active = true GROUP BY type'),
        db.query('SELECT type, SUM(quantity) as total_quantity FROM world_resources WHERE is_depleted = false GROUP BY type')
      ]);
      
      res.json({
        success: true,
        data: {
          world: worldStats.rows[0] || {},
          sleep: sleepStats,
          vehicles: vehicleStats.rows,
          resources: resourceStats.rows
        }
      });
    } catch (error) {
      logger.error('Failed to get world statistics:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Get save history
  router.get('/saves/history', async (req, res) => {
    try {
      const { limit = 10 } = req.query;
      
      const history = await worldStatePersistence.getSaveHistory(Number(limit));
      
      res.json({
        success: true,
        data: history,
        count: history.length
      });
    } catch (error) {
      logger.error('Failed to get save history:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  // Manual save trigger (admin only)
  router.post('/saves/trigger', async (req, res) => {
    try {
      const { reason = 'manual_api' } = req.body;
      
      const result = await worldStatePersistence.saveWorldState(reason);
      
      if (result.success) {
        res.json({
          success: true,
          message: result.message,
          timestamp: new Date().toISOString()
        });
      } else {
        res.status(500).json({
          success: false,
          error: result.message
        });
      }
    } catch (error) {
      logger.error('Failed to trigger manual save:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });

  return router;
}
