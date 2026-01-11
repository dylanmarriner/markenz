import { Pool } from 'pg';
import { 
  ISLAND_LOCATIONS, 
  WORLD_OBJECTS, 
  WORLD_VEHICLES, 
  WORLD_RESOURCES,
  TOOL_SHED_INVENTORY,
  CANONICAL_WORLD_VERSION,
  validateWorldDefinition
} from '../definitions/initial_island_world';
import { logger } from '../../../utils/logger';

export class WorldSeederService {
  constructor(private db: Pool) {}

  /**
   * Idempotent world seeding - creates world once and never resets
   */
  async seedWorld(): Promise<{ success: boolean; message: string; wasCreated: boolean }> {
    try {
      // Check if world already exists
      const worldExists = await this.checkWorldExists();
      
      if (worldExists) {
        logger.info('World already exists - skipping seeding');
        return {
          success: true,
          message: 'World already initialized',
          wasCreated: false
        };
      }

      // Validate world definition before seeding
      const validation = validateWorldDefinition();
      if (!validation.isValid) {
        const errorMsg = `World definition validation failed: ${validation.errors.join(', ')}`;
        logger.error(errorMsg);
        return {
          success: false,
          message: errorMsg,
          wasCreated: false
        };
      }

      // Begin transaction for atomic world creation
      const client = await this.db.connect();
      
      try {
        await client.query('BEGIN');
        
        // Initialize world config
        await this.initializeWorldConfig(client);
        
        // Seed locations
        await this.seedLocations(client);
        
        // Seed objects
        await this.seedObjects(client);
        
        // Seed vehicles
        await this.seedVehicles(client);
        
        // Seed resources
        await this.seedResources(client);
        
        // Initialize tool shed inventory
        await this.initializeToolShed(client);
        
        // Mark world as initialized
        await this.markWorldInitialized(client);
        
        await client.query('COMMIT');
        
        logger.info(`World v${CANONICAL_WORLD_VERSION} seeded successfully`, validation.summary);
        
        return {
          success: true,
          message: `World v${CANONICAL_WORLD_VERSION} created successfully`,
          wasCreated: true
        };
        
      } catch (error) {
        await client.query('ROLLBACK');
        throw error;
      } finally {
        client.release();
      }
      
    } catch (error) {
      logger.error('Failed to seed world:', error);
      return {
        success: false,
        message: `World seeding failed: ${error.message}`,
        wasCreated: false
      };
    }
  }

  /**
   * Check if world has already been initialized
   */
  private async checkWorldExists(): Promise<boolean> {
    const result = await this.db.query(`
      SELECT is_initialized 
      FROM world_config 
      WHERE id = 1
    `);
    
    return result.rows.length > 0 && result.rows[0].is_initialized;
  }

  /**
   * Initialize world configuration
   */
  private async initializeWorldConfig(client: any): Promise<void> {
    await client.query(`
      INSERT INTO world_config (
        id, 
        world_name, 
        world_version, 
        is_initialized, 
        initialization_timestamp
      ) VALUES (
        1, 
        $1, 
        $2, 
        false, 
        NOW()
      ) ON CONFLICT (id) DO UPDATE SET
        world_name = EXCLUDED.world_name,
        world_version = EXCLUDED.world_version
    `, [`${CANONICAL_WORLD_VERSION}_island`, CANONICAL_WORLD_VERSION]);
  }

  /**
   * Seed all world locations
   */
  private async seedLocations(client: any): Promise<void> {
    logger.info(`Seeding ${ISLAND_LOCATIONS.length} locations`);
    
    for (const location of ISLAND_LOCATIONS) {
      await client.query(`
        INSERT INTO world_locations (
          location_id,
          name,
          type,
          parent_location,
          coordinates,
          dimensions,
          sleep_allowed,
          max_occupancy,
          allowed_activities,
          interaction_permissions,
          utilities,
          base_inventory,
          affordances,
          is_active
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        ) ON CONFLICT (location_id) DO NOTHING
      `, [
        location.location_id,
        location.name,
        location.type,
        location.parent_location || null,
        JSON.stringify(location.coordinates),
        JSON.stringify(location.dimensions),
        location.sleep_allowed,
        location.max_occupancy,
        JSON.stringify(location.allowed_activities),
        JSON.stringify(location.interaction_permissions),
        JSON.stringify(location.utilities),
        JSON.stringify(location.base_inventory),
        JSON.stringify(location.affordances),
        location.is_active
      ]);
    }
  }

  /**
   * Seed all world objects
   */
  private async seedObjects(client: any): Promise<void> {
    logger.info(`Seeding ${WORLD_OBJECTS.length} objects`);
    
    for (const object of WORLD_OBJECTS) {
      await client.query(`
        INSERT INTO world_objects (
          object_id,
          location_id,
          name,
          type,
          subtype,
          coordinates,
          dimensions,
          weight,
          is_interactable,
          is_usable,
          is_movable,
          capacity,
          current_state,
          inventory,
          properties,
          owner_agent,
          required_permissions,
          is_active,
          condition
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
        ) ON CONFLICT (object_id) DO NOTHING
      `, [
        object.object_id,
        object.location_id,
        object.name,
        object.type,
        object.subtype || null,
        JSON.stringify(object.coordinates),
        JSON.stringify(object.dimensions),
        object.weight || null,
        object.is_interactable,
        object.is_usable,
        object.is_movable,
        object.capacity || 0,
        JSON.stringify(object.current_state || {}),
        JSON.stringify(object.inventory || []),
        JSON.stringify(object.properties || {}),
        object.owner_agent || null,
        JSON.stringify(object.required_permissions || []),
        object.is_active,
        object.condition || 1.0
      ]);
    }
  }

  /**
   * Seed all vehicles
   */
  private async seedVehicles(client: any): Promise<void> {
    logger.info(`Seeding ${WORLD_VEHICLES.length} vehicles`);
    
    for (const vehicle of WORLD_VEHICLES) {
      await client.query(`
        INSERT INTO world_vehicles (
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
          inventory,
          condition,
          is_active
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
        ) ON CONFLICT (vehicle_id) DO NOTHING
      `, [
        vehicle.vehicle_id,
        vehicle.name,
        vehicle.type,
        vehicle.model,
        vehicle.location_id,
        JSON.stringify(vehicle.coordinates),
        vehicle.rotation,
        vehicle.storage_capacity,
        vehicle.fuel_type,
        vehicle.fuel_level,
        vehicle.max_speed,
        vehicle.is_mounted,
        vehicle.mounted_agent || null,
        vehicle.is_locked,
        JSON.stringify(vehicle.inventory || []),
        vehicle.condition,
        vehicle.is_active
      ]);
    }
  }

  /**
   * Seed all resources
   */
  private async seedResources(client: any): Promise<void> {
    logger.info(`Seeding ${WORLD_RESOURCES.length} resources`);
    
    for (const resource of WORLD_RESOURCES) {
      await client.query(`
        INSERT INTO world_resources (
          resource_id,
          type,
          grade,
          location_id,
          coordinates,
          quantity,
          max_quantity,
          difficulty,
          tool_required,
          regeneration_rate,
          is_depleted
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
        ) ON CONFLICT (resource_id) DO NOTHING
      `, [
        resource.resource_id,
        resource.type,
        resource.grade,
        resource.location_id,
        JSON.stringify(resource.coordinates),
        resource.quantity,
        resource.max_quantity,
        resource.difficulty,
        resource.tool_required || null,
        resource.regeneration_rate || 0,
        resource.is_depleted || false
      ]);
    }
  }

  /**
   * Initialize tool shed with inventory
   */
  private async initializeToolShed(client: any): Promise<void> {
    logger.info('Initializing tool shed inventory');
    
    // Add tools to workbench inventory
    const allTools = Object.values(TOOL_SHED_INVENTORY).flat();
    
    await client.query(`
      UPDATE world_objects 
      SET inventory = $1 
      WHERE object_id = 'workbench'
    `, [JSON.stringify(allTools)]);
  }

  /**
   * Mark world as fully initialized
   */
  private async markWorldInitialized(client: any): Promise<void> {
    await client.query(`
      UPDATE world_config 
      SET is_initialized = true, 
          initialization_timestamp = NOW()
      WHERE id = 1
    `);
  }

  /**
   * Get world initialization status
   */
  async getWorldStatus(): Promise<{
    isInitialized: boolean;
    version: string;
    initializedAt?: Date;
    summary?: any;
  }> {
    const result = await this.db.query(`
      SELECT 
        is_initialized,
        world_version,
        initialization_timestamp
      FROM world_config 
      WHERE id = 1
    `);
    
    if (result.rows.length === 0) {
      return {
        isInitialized: false,
        version: 'none'
      };
    }
    
    const row = result.rows[0];
    
    // Get world summary if initialized
    let summary = null;
    if (row.is_initialized) {
      const summaryResult = await this.db.query(`
        SELECT * FROM current_world_state
      `);
      summary = summaryResult.rows[0];
    }
    
    return {
      isInitialized: row.is_initialized,
      version: row.world_version,
      initializedAt: row.initialization_timestamp,
      summary
    };
  }

  /**
   * Force re-initialization (DEV ONLY)
   * WARNING: This will reset the entire world
   */
  async forceReinitialize(): Promise<{ success: boolean; message: string }> {
    if (process.env.NODE_ENV === 'production') {
      return {
        success: false,
        message: 'Force re-initialization not allowed in production'
      };
    }
    
    const client = await this.db.connect();
    
    try {
      await client.query('BEGIN');
      
      // Clear all world tables
      await client.query('DELETE FROM agent_sleeping_states');
      await client.query('DELETE FROM agent_world_positions');
      await client.query('DELETE FROM agent_sleep_history');
      await client.query('DELETE FROM world_resources');
      await client.query('DELETE FROM world_vehicles');
      await client.query('DELETE FROM world_objects');
      await client.query('DELETE FROM world_locations');
      await client.query('DELETE FROM world_config');
      
      await client.query('COMMIT');
      
      logger.warn('World force reset completed');
      
      // Re-seed the world
      return await this.seedWorld();
      
    } catch (error) {
      await client.query('ROLLBACK');
      throw error;
    } finally {
      client.release();
    }
  }
}
