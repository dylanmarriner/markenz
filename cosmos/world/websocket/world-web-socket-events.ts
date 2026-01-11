import { Server as SocketIOServer } from 'socket.io';
import { Pool } from 'pg';
import { logger } from '../../../utils/logger';
import { SpawnResolver } from '../services/SpawnResolver';
import { SleepStateService } from '../sleeping/SleepStateService';

export class WorldWebSocketEvents {
  private io: SocketIOServer;
  private db: Pool;
  private spawnResolver: SpawnResolver;
  private sleepStateService: SleepStateService;

  constructor(
    io: SocketIOServer,
    db: Pool,
    spawnResolver: SpawnResolver,
    sleepStateService: SleepStateService
  ) {
    this.io = io;
    this.db = db;
    this.spawnResolver = spawnResolver;
    this.sleepStateService = sleepStateService;
  }

  /**
   * Register all world-related WebSocket events
   */
  registerEvents(): void {
    const worldNamespace = this.io.of('/world');
    
    worldNamespace.on('connection', (socket) => {
      logger.info(`World WebSocket connected: ${socket.id}`);
      
      // Send initial world snapshot
      this.sendWorldSnapshot(socket);
      
      // Join agent-specific room if provided
      const agentId = socket.handshake.query.agentId as string;
      if (agentId) {
        socket.join(`agent:${agentId}`);
        logger.debug(`Agent ${agentId} joined world room`);
      }
      
      // Handle subscribe to events
      socket.on('subscribe', (data) => {
        this.handleSubscribe(socket, data);
      });
      
      // Handle unsubscribe from events
      socket.on('unsubscribe', (data) => {
        this.handleUnsubscribe(socket, data);
      });
      
      // Handle agent movement requests
      socket.on('move_agent', async (data) => {
        await this.handleMoveAgent(socket, data);
      });
      
      // Handle object interaction
      socket.on('use_object', async (data) => {
        await this.handleUseObject(socket, data);
      });
      
      // Handle sleep requests
      socket.on('start_sleep', async (data) => {
        await this.handleStartSleep(socket, data);
      });
      
      socket.on('stop_sleep', async (data) => {
        await this.handleStopSleep(socket, data);
      });
      
      // Handle vehicle interactions
      socket.on('mount_vehicle', async (data) => {
        await this.handleMountVehicle(socket, data);
      });
      
      socket.on('dismount_vehicle', async (data) => {
        await this.handleDismountVehicle(socket, data);
      });
      
      // Handle resource mining
      socket.on('mine_resource', async (data) => {
        await this.handleMineResource(socket, data);
      });
      
      socket.on('disconnect', () => {
        logger.info(`World WebSocket disconnected: ${socket.id}`);
      });
    });
  }

  /**
   * Send complete world snapshot to client
   */
  private async sendWorldSnapshot(socket: any): Promise<void> {
    try {
      const [
        locations,
        objects,
        vehicles,
        resources,
        agentPositions,
        sleepingAgents
      ] = await Promise.all([
        this.db.query('SELECT * FROM world_locations WHERE is_active = true'),
        this.db.query('SELECT * FROM world_objects WHERE is_active = true'),
        this.db.query('SELECT * FROM world_vehicles WHERE is_active = true'),
        this.db.query('SELECT * FROM world_resources'),
        this.spawnResolver.getAllAgentPositions(),
        this.sleepStateService.getSleepingAgents()
      ]);
      
      socket.emit('world:snapshot', {
        timestamp: new Date().toISOString(),
        locations: locations.rows,
        objects: objects.rows,
        vehicles: vehicles.rows,
        resources: resources.rows,
        agents: agentPositions,
        sleepingAgents: sleepingAgents
      });
      
    } catch (error) {
      logger.error('Failed to send world snapshot:', error);
      socket.emit('error', { message: 'Failed to load world data' });
    }
  }

  /**
   * Handle subscription to specific events
   */
  private handleSubscribe(socket: any, data: { events: string[] }): void {
    const events = data.events || [];
    
    events.forEach(event => {
      socket.join(event);
    });
    
    socket.emit('subscribed', { events });
    logger.debug(`Socket ${socket.id} subscribed to events: ${events.join(', ')}`);
  }

  /**
   * Handle unsubscribe from events
   */
  private handleUnsubscribe(socket: any, data: { events: string[] }): void {
    const events = data.events || [];
    
    events.forEach(event => {
      socket.leave(event);
    });
    
    socket.emit('unsubscribed', { events });
    logger.debug(`Socket ${socket.id} unsubscribed from events: ${events.join(', ')}`);
  }

  /**
   * Handle agent movement
   */
  private async handleMoveAgent(socket: any, data: {
    agentId: string;
    toLocation: string;
    coordinates?: { x: number; y: number; z: number };
  }): Promise<void> {
    try {
      const { agentId, toLocation, coordinates } = data;
      
      // Get current position
      const currentPos = await this.spawnResolver.getAgentWorldPosition(agentId);
      if (!currentPos) {
        socket.emit('error', { message: 'Agent not found' });
        return;
      }
      
      // Update position
      const targetCoords = coordinates || { x: 0, y: 0, z: 0 };
      await this.spawnResolver.updateAgentPosition(
        agentId,
        toLocation,
        targetCoords,
        true, // is_moving
        toLocation,
        targetCoords
      );
      
      // Broadcast movement
      this.io.of('/world').emit('world:agent_moved', {
        agentId,
        fromLocation: currentPos.location_id,
        toLocation,
        coordinates: targetCoords,
        timestamp: new Date().toISOString()
      });
      
      // Simulate arrival after delay
      setTimeout(async () => {
        await this.spawnResolver.updateAgentPosition(
          agentId,
          toLocation,
          targetCoords,
          false // is_moving = false
        );
        
        this.io.of('/world').emit('world:agent_arrived', {
          agentId,
          location: toLocation,
          coordinates: targetCoords,
          timestamp: new Date().toISOString()
        });
      }, 2000); // 2 second movement
      
    } catch (error) {
      logger.error('Failed to move agent:', error);
      socket.emit('error', { message: 'Failed to move agent' });
    }
  }

  /**
   * Handle object interaction
   */
  private async handleUseObject(socket: any, data: {
    agentId: string;
    objectId: string;
    action: string;
  }): Promise<void> {
    try {
      const { agentId, objectId, action } = data;
      
      // Get object details
      const objectResult = await this.db.query(`
        SELECT wo.*, wl.location_id
        FROM world_objects wo
        JOIN world_locations wl ON wl.location_id = wo.location_id
        WHERE wo.object_id = $1 AND wo.is_active = true
      `, [objectId]);
      
      if (objectResult.rows.length === 0) {
        socket.emit('error', { message: 'Object not found' });
        return;
      }
      
      const object = objectResult.rows[0];
      
      // Handle bed usage (sleeping)
      if (object.type === 'bed' && action === 'sleep') {
        const sleepResult = await this.sleepStateService.startSleeping(
          agentId,
          object.location_id,
          objectId,
          true,
          'voluntary_sleep'
        );
        
        if (sleepResult.success) {
          this.io.of('/world').emit('world:agent_sleep_started', {
            agentId,
            locationId: object.location_id,
            objectId,
            timestamp: new Date().toISOString()
          });
        } else {
          socket.emit('error', { message: sleepResult.message });
        }
      }
      
      // Handle other object types
      else {
        // Update object state
        await this.db.query(`
          UPDATE world_objects SET
            last_interaction = NOW(),
            interaction_count = COALESCE(interaction_count, 0) + 1
          WHERE object_id = $1
        `, [objectId]);
        
        this.io.of('/world').emit('world:object_used', {
          agentId,
          objectId,
          action,
          timestamp: new Date().toISOString()
        });
      }
      
    } catch (error) {
      logger.error('Failed to use object:', error);
      socket.emit('error', { message: 'Failed to use object' });
    }
  }

  /**
   * Handle start sleep
   */
  private async handleStartSleep(socket: any, data: {
    agentId: string;
    locationId: string;
    objectId?: string;
  }): Promise<void> {
    try {
      const { agentId, locationId, objectId } = data;
      
      const result = await this.sleepStateService.startSleeping(
        agentId,
        locationId,
        objectId,
        true,
        'websocket_request'
      );
      
      if (result.success) {
        this.io.of('/world').emit('world:agent_sleep_started', {
          agentId,
          locationId,
          objectId,
          timestamp: new Date().toISOString()
        });
      } else {
        socket.emit('error', { message: result.message });
      }
      
    } catch (error) {
      logger.error('Failed to start sleep:', error);
      socket.emit('error', { message: 'Failed to start sleep' });
    }
  }

  /**
   * Handle stop sleep
   */
  private async handleStopSleep(socket: any, data: { agentId: string }): Promise<void> {
    try {
      const { agentId } = data;
      
      const result = await this.sleepStateService.stopSleeping(agentId);
      
      if (result.success) {
        this.io.of('/world').emit('world:agent_sleep_ended', {
          agentId,
          timestamp: new Date().toISOString()
        });
      } else {
        socket.emit('error', { message: result.message });
      }
      
    } catch (error) {
      logger.error('Failed to stop sleep:', error);
      socket.emit('error', { message: 'Failed to stop sleep' });
    }
  }

  /**
   * Handle vehicle mounting
   */
  private async handleMountVehicle(socket: any, data: {
    agentId: string;
    vehicleId: string;
    position?: string;
  }): Promise<void> {
    try {
      const { agentId, vehicleId, position = 'driver' } = data;
      
      // Update vehicle state
      await this.db.query(`
        UPDATE world_vehicles SET
          is_mounted = true,
          mounted_agent = $1
        WHERE vehicle_id = $2
      `, [agentId, vehicleId]);
      
      // Update agent position
      await this.spawnResolver.updateAgentPosition(
        agentId,
        'vehicle', // Special location for mounted state
        { x: 0, y: 0, z: 0 },
        false,
        undefined,
        undefined,
        vehicleId,
        true
      );
      
      this.io.of('/world').emit('world:vehicle_mounted', {
        agentId,
        vehicleId,
        position,
        timestamp: new Date().toISOString()
      });
      
    } catch (error) {
      logger.error('Failed to mount vehicle:', error);
      socket.emit('error', { message: 'Failed to mount vehicle' });
    }
  }

  /**
   * Handle vehicle dismounting
   */
  private async handleDismountVehicle(socket: any, data: {
    agentId: string;
    locationId: string;
    coordinates: { x: number; y: number; z: number };
  }): Promise<void> {
    try {
      const { agentId, locationId, coordinates } = data;
      
      // Get current vehicle
      const agentPos = await this.spawnResolver.getAgentWorldPosition(agentId);
      const vehicleId = agentPos?.vehicle_id;
      
      if (vehicleId) {
        // Update vehicle state
        await this.db.query(`
          UPDATE world_vehicles SET
            is_mounted = false,
            mounted_agent = NULL
          WHERE vehicle_id = $1
        `, [vehicleId]);
      }
      
      // Update agent position
      await this.spawnResolver.updateAgentPosition(
        agentId,
        locationId,
        coordinates,
        false,
        undefined,
        undefined,
        undefined,
        false
      );
      
      this.io.of('/world').emit('world:vehicle_dismounted', {
        agentId,
        vehicleId,
        locationId,
        coordinates,
        timestamp: new Date().toISOString()
      });
      
    } catch (error) {
      logger.error('Failed to dismount vehicle:', error);
      socket.emit('error', { message: 'Failed to dismount vehicle' });
    }
  }

  /**
   * Handle resource mining
   */
  private async handleMineResource(socket: any, data: {
    agentId: string;
    resourceId: string;
    tool: string;
  }): Promise<void> {
    try {
      const { agentId, resourceId, tool } = data;
      
      // Get resource details
      const resourceResult = await this.db.query(`
        SELECT * FROM world_resources
        WHERE resource_id = $1 AND NOT is_depleted
      `, [resourceId]);
      
      if (resourceResult.rows.length === 0) {
        socket.emit('error', { message: 'Resource not found or depleted' });
        return;
      }
      
      const resource = resourceResult.rows[0];
      
      // Check if tool is correct
      if (resource.tool_required && resource.tool_required !== tool) {
        socket.emit('error', { message: `Wrong tool. Required: ${resource.tool_required}` });
        return;
      }
      
      // Mine resource (simple implementation)
      const minedAmount = Math.min(10, resource.quantity);
      
      await this.db.query(`
        UPDATE world_resources SET
          quantity = quantity - $1,
          is_depleted = CASE WHEN quantity - $1 <= 0 THEN true ELSE false END,
          last_mined = NOW()
        WHERE resource_id = $2
      `, [minedAmount, resourceId]);
      
      // Add to agent inventory (simplified)
      await this.db.query(`
        UPDATE agent_world_positions SET
          last_activity = 'mining',
          last_state_change = NOW()
        WHERE agent_id = $1
      `, [agentId]);
      
      this.io.of('/world').emit('world:resource_mined', {
        agentId,
        resourceId,
        amount: minedAmount,
        remaining: Math.max(0, resource.quantity - minedAmount),
        timestamp: new Date().toISOString()
      });
      
    } catch (error) {
      logger.error('Failed to mine resource:', error);
      socket.emit('error', { message: 'Failed to mine resource' });
    }
  }

  /**
   * Broadcast agent spawned event
   */
  broadcastAgentSpawned(agentId: string, location: string, reason: string): void {
    this.io.of('/world').emit('world:agent_spawned', {
      agentId,
      location,
      reason,
      timestamp: new Date().toISOString()
    });
  }

  /**
   * Broadcast world state saved
   */
  broadcastWorldSaved(reason: string): void {
    this.io.of('/world').emit('world:state_saved', {
      reason,
      timestamp: new Date().toISOString()
    });
  }
}
