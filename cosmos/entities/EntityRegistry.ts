
/**
 * ENTITY REGISTRY
 * 
 * Manages all entities in the planetary system
 * Provides tracking, lifecycle management, and spatial indexing
 */

export interface Entity {
  id: string;
  type: EntityType;
  name: string;
  location: { x: number; y: number; z: number };
  velocity: { x: number; y: number; z: number };
  mass: number;
  properties: Map<string, any>;
  created: Date;
  lastUpdated: Date;
  active: boolean;
}

export enum EntityType {
  // Physical entities
  AGENT = 'agent',
  OBJECT = 'object',
  STRUCTURE = 'structure',
  RESOURCE = 'resource',
  
  // Environmental entities
  PLANT = 'plant',
  ANIMAL = 'animal',
  WEATHER = 'weather',
  TERRAIN = 'terrain',
  
  // Abstract entities
  LOCATION = 'location',
  ZONE = 'zone',
  EVENT = 'event',
  DATA = 'data'
}

export interface SpatialIndex {
  x: number;
  y: number;
  z: number;
  radius: number;
  entities: Set<string>;
}

export interface EntityQuery {
  type?: EntityType;
  location?: { x: number; y: number; z: number };
  radius?: number;
  properties?: { [key: string]: any };
  active?: boolean;
}

export interface EntityEvent {
  id: string;
  entityId: string;
  type: 'created' | 'updated' | 'deleted' | 'moved' | 'interacted';
  timestamp: Date;
  data?: any;
}

export class EntityRegistry {
  private entities: Map<string, Entity> = new Map();
  private spatialIndex: Map<string, SpatialIndex> = new Map();
  private events: EntityEvent[] = [];
  private nextId: number = 1;
  private spatialGridSize: number = 100; // Grid cell size in meters

  constructor() {
    this.initializeSpatialIndex();
  }

  /**
   * Initialize spatial indexing grid
   */
  private initializeSpatialIndex(): void {
    // Create a basic spatial grid
    for (let x = 0; x < 10; x++) {
      for (let y = 0; y < 10; y++) {
        for (let z = 0; z < 5; z++) {
          const indexKey = `${x}_${y}_${z}`;
          this.spatialIndex.set(indexKey, {
            x: x * this.spatialGridSize,
            y: y * this.spatialGridSize,
            z: z * this.spatialGridSize,
            radius: this.spatialGridSize,
            entities: new Set()
          });
        }
      }
    }
  }

  /**
   * Create a new entity
   */
  createEntity(
    type: EntityType,
    name: string,
    location: { x: number; y: number; z: number },
    properties: { [key: string]: any } = {}
  ): Entity {
    const id = this.generateEntityId();
    
    const entity: Entity = {
      id,
      type,
      name,
      location: { ...location },
      velocity: { x: 0, y: 0, z: 0 },
      mass: 1,
      properties: new Map(Object.entries(properties)),
      created: new Date(),
      lastUpdated: new Date(),
      active: true
    };

    this.entities.set(id, entity);
    this.addToSpatialIndex(entity);
    this.recordEvent({
      id: this.generateEventId(),
      entityId: id,
      type: 'created',
      timestamp: new Date(),
      data: { type, name, location }
    });

    return entity;
  }

  /**
   * Get entity by ID
   */
  getEntity(id: string): Entity | undefined {
    return this.entities.get(id);
  }

  /**
   * Update entity
   */
  updateEntity(id: string, updates: Partial<Entity>): boolean {
    const entity = this.entities.get(id);
    if (!entity) return false;

    const oldLocation = { ...entity.location };
    
    // Apply updates
    if (updates.location) {
      this.removeFromSpatialIndex(entity);
      entity.location = { ...updates.location };
      this.addToSpatialIndex(entity);
    }
    
    if (updates.velocity) {
      entity.velocity = { ...updates.velocity };
    }
    
    if (updates.properties) {
      Object.entries(updates.properties).forEach(([key, value]) => {
        if (value === undefined) {
          entity.properties.delete(key);
        } else {
          entity.properties.set(key, value);
        }
      });
    }
    
    if (updates.mass !== undefined) {
      entity.mass = updates.mass;
    }
    
    if (updates.active !== undefined) {
      entity.active = updates.active;
    }
    
    entity.lastUpdated = new Date();

    // Record movement event if location changed
    if (updates.location && (oldLocation.x !== updates.location.x || 
                           oldLocation.y !== updates.location.y || 
                           oldLocation.z !== updates.location.z)) {
      this.recordEvent({
        id: this.generateEventId(),
        entityId: id,
        type: 'moved',
        timestamp: new Date(),
        data: { from: oldLocation, to: updates.location }
      });
    } else {
      this.recordEvent({
        id: this.generateEventId(),
        entityId: id,
        type: 'updated',
        timestamp: new Date()
      });
    }

    return true;
  }

  /**
   * Delete entity
   */
  deleteEntity(id: string): boolean {
    const entity = this.entities.get(id);
    if (!entity) return false;

    this.removeFromSpatialIndex(entity);
    this.entities.delete(id);
    
    this.recordEvent({
      id: this.generateEventId(),
      entityId: id,
      type: 'deleted',
      timestamp: new Date()
    });

    return true;
  }

  /**
   * Query entities
   */
  queryEntities(query: EntityQuery): Entity[] {
    let results: Entity[] = Array.from(this.entities.values());

    // Filter by type
    if (query.type) {
      results = results.filter(entity => entity.type === query.type);
    }

    // Filter by location/radius
    if (query.location && query.radius) {
      results = results.filter(entity => {
        const distance = this.calculateDistance(query.location!, entity.location);
        return distance <= query.radius!;
      });
    }

    // Filter by properties
    if (query.properties) {
      results = results.filter(entity => {
        return Object.entries(query.properties!).every(([key, value]) => {
          const entityValue = entity.properties.get(key);
          return entityValue === value;
        });
      });
    }

    // Filter by active status
    if (query.active !== undefined) {
      results = results.filter(entity => entity.active === query.active);
    }

    return results;
  }

  /**
   * Get entities near a location
   */
  getNearbyEntities(
    location: { x: number; y: number; z: number },
    radius: number,
    type?: EntityType
  ): Entity[] {
    const nearbyEntities: Entity[] = [];
    
    // Check spatial grid cells that intersect with the search radius
    const gridCells = this.getGridCellsInRadius(location, radius);
    
    gridCells.forEach(cellKey => {
      const cell = this.spatialIndex.get(cellKey);
      if (cell) {
        cell.entities.forEach(entityId => {
          const entity = this.entities.get(entityId);
          if (entity && entity.active) {
            const distance = this.calculateDistance(location, entity.location);
            if (distance <= radius && (!type || entity.type === type)) {
              nearbyEntities.push(entity);
            }
          }
        });
      }
    });

    return nearbyEntities;
  }

  /**
   * Move entity
   */
  moveEntity(id: string, newLocation: { x: number; y: number; z: number }): boolean {
    return this.updateEntity(id, { location: newLocation });
  }

  /**
   * Apply physics to all entities
   */
  updatePhysics(deltaTime: number): void {
    this.entities.forEach(entity => {
      if (!entity.active) return;

      // Update position based on velocity
      if (entity.velocity.x !== 0 || entity.velocity.y !== 0 || entity.velocity.z !== 0) {
        const newLocation = {
          x: entity.location.x + entity.velocity.x * deltaTime,
          y: entity.location.y + entity.velocity.y * deltaTime,
          z: entity.location.z + entity.velocity.z * deltaTime
        };

        this.removeFromSpatialIndex(entity);
        entity.location = newLocation;
        this.addToSpatialIndex(entity);
        entity.lastUpdated = new Date();
      }

      // Apply friction/damping
      entity.velocity.x *= 0.99;
      entity.velocity.y *= 0.99;
      entity.velocity.z *= 0.99;

      // Stop very small velocities
      if (Math.abs(entity.velocity.x) < 0.01) entity.velocity.x = 0;
      if (Math.abs(entity.velocity.y) < 0.01) entity.velocity.y = 0;
      if (Math.abs(entity.velocity.z) < 0.01) entity.velocity.z = 0;
    });
  }

  /**
   * Apply force to entity
   */
  applyForce(id: string, force: { x: number; y: number; z: number }): boolean {
    const entity = this.entities.get(id);
    if (!entity || !entity.active) return false;

    // F = ma, so a = F/m
    const acceleration = {
      x: force.x / entity.mass,
      y: force.y / entity.mass,
      z: force.z / entity.mass
    };

    // Update velocity (assuming 1 second time step for simplicity)
    entity.velocity.x += acceleration.x;
    entity.velocity.y += acceleration.y;
    entity.velocity.z += acceleration.z;

    entity.lastUpdated = new Date();
    return true;
  }

  /**
   * Check collision between two entities
   */
  checkCollision(id1: string, id2: string): boolean {
    const entity1 = this.entities.get(id1);
    const entity2 = this.entities.get(id2);
    
    if (!entity1 || !entity2 || !entity1.active || !entity2.active) {
      return false;
    }

    const distance = this.calculateDistance(entity1.location, entity2.location);
    const radius1 = (entity1.properties.get('radius') as number) || 1;
    const radius2 = (entity2.properties.get('radius') as number) || 1;

    return distance < (radius1 + radius2);
  }

  /**
   * Get all entities
   */
  getAllEntities(): Entity[] {
    return Array.from(this.entities.values());
  }

  /**
   * Get entities by type
   */
  getEntitiesByType(type: EntityType): Entity[] {
    return this.queryEntities({ type });
  }

  /**
   * Get recent events
   */
  getRecentEvents(count: number = 100): EntityEvent[] {
    return this.events.slice(-count);
  }

  /**
   * Get events for entity
   */
  getEventsForEntity(entityId: string, count: number = 50): EntityEvent[] {
    return this.events
      .filter(event => event.entityId === entityId)
      .slice(-count);
  }

  /**
   * Generate unique entity ID
   */
  private generateEntityId(): string {
    return `entity_${this.nextId++}_${Date.now()}`;
  }

  /**
   * Generate unique event ID
   */
  private generateEventId(): string {
    return `event_${Date.now()}_${ChaosSys.getInstance().next().toString(36).substr(2, 9)}`;
  }

  /**
   * Add entity to spatial index
   */
  private addToSpatialIndex(entity: Entity): void {
    const cellKey = this.getSpatialCellKey(entity.location);
    const cell = this.spatialIndex.get(cellKey);
    if (cell) {
      cell.entities.add(entity.id);
    }
  }

  /**
   * Remove entity from spatial index
   */
  private removeFromSpatialIndex(entity: Entity): void {
    const cellKey = this.getSpatialCellKey(entity.location);
    const cell = this.spatialIndex.get(cellKey);
    if (cell) {
      cell.entities.delete(entity.id);
    }
  }

  /**
   * Get spatial cell key for location
   */
  private getSpatialCellKey(location: { x: number; y: number; z: number }): string {
    const x = Math.floor(location.x / this.spatialGridSize);
    const y = Math.floor(location.y / this.spatialGridSize);
    const z = Math.floor(location.z / this.spatialGridSize);
    return `${x}_${y}_${z}`;
  }

  /**
   * Get grid cells in radius
   */
  private getGridCellsInRadius(location: { x: number; y: number; z: number }, radius: number): string[] {
    const cells: string[] = [];
    const cellRadius = Math.ceil(radius / this.spatialGridSize);
    
    const centerCell = this.getSpatialCellKey(location);
    const [cx, cy, cz] = centerCell.split('_').map(Number);
    
    for (let dx = -cellRadius; dx <= cellRadius; dx++) {
      for (let dy = -cellRadius; dy <= cellRadius; dy++) {
        for (let dz = -cellRadius; dz <= cellRadius; dz++) {
          const cellKey = `${cx + dx}_${cy + dy}_${cz + dz}`;
          if (this.spatialIndex.has(cellKey)) {
            cells.push(cellKey);
          }
        }
      }
    }
    
    return cells;
  }

  /**
   * Calculate distance between two points
   */
  private calculateDistance(
    point1: { x: number; y: number; z: number },
    point2: { x: number; y: number; z: number }
  ): number {
    const dx = point1.x - point2.x;
    const dy = point1.y - point2.y;
    const dz = point1.z - point2.z;
    return Math.sqrt(dx * dx + dy * dy + dz * dz);
  }

  /**
   * Record event
   */
  private recordEvent(event: EntityEvent): void {
    this.events.push(event);
    
    // Keep only last 10000 events
    if (this.events.length > 10000) {
      this.events = this.events.slice(-10000);
    }
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalEntities: number;
    activeEntities: number;
    entitiesByType: { [key in EntityType]?: number };
    totalEvents: number;
  } {
    const entitiesByType: { [key in EntityType]?: number } = {};
    
    Object.values(EntityType).forEach(type => {
      entitiesByType[type] = 0;
    });
    
    this.entities.forEach(entity => {
      if (entitiesByType[entity.type] !== undefined) {
        entitiesByType[entity.type]++;
      }
    });

    return {
      totalEntities: this.entities.size,
      activeEntities: Array.from(this.entities.values()).filter(e => e.active).length,
      entitiesByType,
      totalEvents: this.events.length
    };
  }

  /**
   * Cleanup inactive entities
   */
  cleanupInactive(maxAge: number = 86400000): number { // Default 24 hours
    const now = new Date();
    const toDelete: string[] = [];
    
    this.entities.forEach((entity, id) => {
      if (!entity.active && (now.getTime() - entity.lastUpdated.getTime()) > maxAge) {
        toDelete.push(id);
      }
    });
    
    toDelete.forEach(id => this.deleteEntity(id));
    return toDelete.length;
  }
}

export default EntityRegistry;
