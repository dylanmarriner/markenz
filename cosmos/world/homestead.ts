
/**
 * HOMESTEAD
 * 
 * Home base management, shelter, resources, and infrastructure
 * Building construction, maintenance, and resource management
 */

import { ChaosSys } from '../chaos/chaos-sys';

export interface Building {
  id: string;
  type: 'shelter' | 'workshop' | 'storage' | 'garden' | 'well' | 'fireplace';
  position: { x: number; y: number };
  size: { width: number; height: number; depth: number };
  condition: number; // 0-1, structural integrity
  functionality: number; // 0-1, operational status
  resources: Map<string, number>; // Stored resources
}

export interface Resource {
  type: 'wood' | 'stone' | 'food' | 'water' | 'tools' | 'fuel' | 'materials';
  quantity: number;
  quality: number; // 0-1
  location: string; // storage location
  perishable: boolean;
  expiration?: number; // timestamp
}

export interface HomesteadState {
  security_level: number; // 0-1, protection from threats
  comfort_level: number; // 0-1, living conditions
  self_sufficiency: number; // 0-1, resource independence
  maintenance_needs: number; // 0-1, required repairs
  population_capacity: number;
  current_population: number;
}

export class Homestead {
  private buildings: Map<string, Building>;
  private resources: Map<string, Resource>;
  private state: HomesteadState;
  private lastMaintenance: number;
  private weatherImpact: number;

  constructor() {
    this.buildings = new Map();
    this.resources = new Map();
    this.lastMaintenance = Date.now();
    this.weatherImpact = 0;

    this.state = {
      security_level: 0.5,
      comfort_level: 0.6,
      self_sufficiency: 0.4,
      maintenance_needs: 0.2,
      population_capacity: 4,
      current_population: 2
    };

    this._initializeHomestead();
  }

  update(dtSeconds: number, weather: { temperature: number; precipitation: number; wind: number }): void {
    // Update weather impact
    this._updateWeatherImpact(weather);

    // Process resource consumption
    this._processResourceConsumption(dtSeconds);

    // Update building conditions
    this._updateBuildingConditions(dtSeconds, weather);

    // Calculate maintenance needs
    this._calculateMaintenanceNeeds();

    // Update homestead state
    this._updateHomesteadState();

    // Process perishable resources
    this._processPerishableResources(dtSeconds);
  }

  addBuilding(building: Omit<Building, 'id'>): string {
    const id = `building_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullBuilding: Building = {
      id,
      ...building,
      condition: 1.0,
      functionality: 1.0,
      resources: new Map()
    };

    this.buildings.set(id, fullBuilding);
    this._updateHomesteadState();
    return id;
  }

  addResource(resource: Omit<Resource, 'location'>): void {
    const resourceId = `${resource.type}_${Date.now()}`;
    const fullResource: Resource = {
      ...resource,
      location: this._findOptimalStorage(resource.type)
    };

    this.resources.set(resourceId, fullResource);
  }

  consumeResource(type: Resource['type'], quantity: number): boolean {
    let availableQuantity = 0;
    const resourcesToConsume: string[] = [];

    // Find available resources of this type
    for (const [id, resource] of this.resources) {
      if (resource.type === type && resource.quantity > 0) {
        availableQuantity += resource.quantity;
        resourcesToConsume.push(id);
      }
    }

    if (availableQuantity < quantity) {
      return false; // Insufficient resources
    }

    // Consume resources (FIFO - first in, first out)
    let remainingQuantity = quantity;
    for (const id of resourcesToConsume) {
      const resource = this.resources.get(id)!;
      const consumeAmount = Math.min(resource.quantity, remainingQuantity);
      resource.quantity -= consumeAmount;
      remainingQuantity -= consumeAmount;

      if (resource.quantity <= 0) {
        this.resources.delete(id);
      }

      if (remainingQuantity <= 0) break;
    }

    return true;
  }

  getBuildingStatus(): {
    total_buildings: number;
    average_condition: number;
    functionality_rate: number;
    maintenance_urgency: number;
  } {
    if (this.buildings.size === 0) {
      return {
        total_buildings: 0,
        average_condition: 0,
        functionality_rate: 0,
        maintenance_urgency: 0
      };
    }

    const conditions = Array.from(this.buildings.values()).map(b => b.condition);
    const functionalities = Array.from(this.buildings.values()).map(b => b.functionality);

    return {
      total_buildings: this.buildings.size,
      average_condition: conditions.reduce((sum, c) => sum + c, 0) / conditions.length,
      functionality_rate: functionalities.reduce((sum, f) => sum + f, 0) / functionalities.length,
      maintenance_urgency: this.state.maintenance_needs
    };
  }

  getResourceStatus(): {
    total_types: number;
    critical_resources: string[];
    abundance_level: number;
    storage_efficiency: number;
  } {
    const resourceTypes = new Set(Array.from(this.resources.values()).map(r => r.type));
    const resourceSummary = new Map<Resource['type'], number>();

    // Aggregate resources by type
    for (const resource of this.resources.values()) {
      const current = resourceSummary.get(resource.type) || 0;
      resourceSummary.set(resource.type, current + resource.quantity);
    }

    // Check critical levels
    const criticalResources = [];
    for (const [type, quantity] of resourceSummary) {
      if (this._isResourceCritical(type, quantity)) {
        criticalResources.push(type);
      }
    }

    // Calculate abundance
    const totalResources = Array.from(resourceSummary.values()).reduce((sum, q) => sum + q, 0);
    const abundanceLevel = Math.min(1, totalResources / 1000); // Normalized to 1000 units

    return {
      total_types: resourceTypes.size,
      critical_resources: criticalResources,
      abundance_level: abundanceLevel,
      storage_efficiency: this._calculateStorageEfficiency()
    };
  }

  performMaintenance(buildingId?: string): void {
    if (buildingId) {
      const building = this.buildings.get(buildingId);
      if (building) {
        building.condition = Math.min(1, building.condition + 0.2);
        building.functionality = Math.min(1, building.functionality + 0.3);
      }
    } else {
      // Maintain all buildings
      for (const building of this.buildings.values()) {
        building.condition = Math.min(1, building.condition + 0.1);
        building.functionality = Math.min(1, building.functionality + 0.15);
      }
    }

    this.lastMaintenance = Date.now();
    this.state.maintenance_needs = Math.max(0, this.state.maintenance_needs - 0.3);
  }

  expandHomestead(expansion: { building_type: Building['type']; materials: Resource['type'][] }): boolean {
    // Check if materials are available
    for (const material of expansion.materials) {
      if (!this.consumeResource(material, 10)) {
        return false; // Insufficient materials
      }
    }

    // Add new building
    const position = this._findOptimalBuildingLocation(expansion.building_type);
    this.addBuilding({
      type: expansion.building_type,
      position,
      size: { width: 5, height: 3, depth: 4 },
      resources: new Map(),
      condition: 1.0,
      functionality: 1.0
    });

    return true;
  }

  private _initializeHomestead(): void {
    // Add initial buildings
    this.addBuilding({
      type: 'shelter',
      position: { x: 0, y: 0 },
      size: { width: 8, height: 3, depth: 6 },
      resources: new Map(),
      condition: 1.0,
      functionality: 1.0
    });

    this.addBuilding({
      type: 'storage',
      position: { x: 10, y: 0 },
      size: { width: 4, height: 2, depth: 3 },
      resources: new Map(),
      condition: 1.0,
      functionality: 1.0
    });

    // Add initial resources
    this.addResource({ type: 'wood', quantity: 100, quality: 0.7, perishable: false });
    this.addResource({ type: 'food', quantity: 50, quality: 0.8, perishable: true, expiration: Date.now() + 7 * 24 * 3600000 });
    this.addResource({ type: 'water', quantity: 200, quality: 0.9, perishable: false });
  }

  private _updateWeatherImpact(weather: { temperature: number; precipitation: number; wind: number }): void {
    // Extreme weather increases maintenance needs
    let impact = 0;

    if (weather.temperature < -10 || weather.temperature > 35) {
      impact += 0.2;
    }
    if (weather.precipitation > 10) {
      impact += 0.15;
    }
    if (weather.wind > 15) {
      impact += 0.1;
    }

    this.weatherImpact = Math.min(1, impact);
  }

  private _processResourceConsumption(dtSeconds: number): void {
    // Basic consumption rates
    const consumptionRates = {
      food: 0.1 * this.state.current_population,
      water: 0.2 * this.state.current_population,
      fuel: 0.05,
      wood: 0.02
    };

    for (const [type, rate] of Object.entries(consumptionRates)) {
      this.consumeResource(type as Resource['type'], rate * dtSeconds);
    }
  }

  private _updateBuildingConditions(dtSeconds: number, weather: { temperature: number; precipitation: number; wind: number }): void {
    for (const building of this.buildings.values()) {
      // Natural decay
      building.condition -= dtSeconds * 0.0001;
      building.functionality -= dtSeconds * 0.00005;

      // Weather damage
      if (this.weatherImpact > 0.3) {
        building.condition -= dtSeconds * this.weatherImpact * 0.0002;
        building.functionality -= dtSeconds * this.weatherImpact * 0.0001;
      }

      // Clamp values
      building.condition = Math.max(0, building.condition);
      building.functionality = Math.max(0, building.functionality);
    }
  }

  private _calculateMaintenanceNeeds(): void {
    if (this.buildings.size === 0) return;

    const avgCondition = Array.from(this.buildings.values())
      .reduce((sum, b) => sum + b.condition, 0) / this.buildings.size;

    if (avgCondition < 0.7) {
      this.state.maintenance_needs = Math.min(1, this.state.maintenance_needs + 0.01);
    } else {
      this.state.maintenance_needs = Math.max(0, this.state.maintenance_needs - 0.005);
    }
  }

  private _updateHomesteadState(): void {
    // Calculate security level
    const shelterBuildings = Array.from(this.buildings.values()).filter(b => b.type === 'shelter');
    const avgShelterCondition = shelterBuildings.length > 0 ? 
      shelterBuildings.reduce((sum, b) => sum + b.condition, 0) / shelterBuildings.length : 0;
    this.state.security_level = avgShelterCondition * 0.8;

    // Calculate comfort level
    const totalFunctionality = Array.from(this.buildings.values())
      .reduce((sum, b) => sum + b.functionality, 0);
    this.state.comfort_level = Math.min(1, totalFunctionality / (this.buildings.size * 2));

    // Calculate self-sufficiency
    const resourceTypes = new Set(Array.from(this.resources.values()).map(r => r.type));
    this.state.self_sufficiency = Math.min(1, resourceTypes.size / 7); // 7 basic resource types
  }

  private _processPerishableResources(dtSeconds: number): void {
    const now = Date.now();
    
    for (const [id, resource] of this.resources) {
      if (resource.perishable && resource.expiration) {
        const timeToExpiration = resource.expiration - now;
        if (timeToExpiration <= 0) {
          this.resources.delete(id);
        } else {
          // Gradual quality decay
          const decayRate = 1 - (timeToExpiration / (7 * 24 * 3600000)); // 7 days
          resource.quality = Math.max(0, resource.quality * (1 - decayRate * 0.01));
        }
      }
    }
  }

  private _findOptimalStorage(resourceType: Resource['type']): string {
    // Find appropriate storage building
    const storageMap = {
      food: 'storage',
      water: 'well',
      wood: 'storage',
      stone: 'storage',
      tools: 'workshop',
      fuel: 'storage',
      materials: 'storage'
    };

    const buildingType = storageMap[resourceType] || 'storage';
    const storageBuilding = Array.from(this.buildings.values()).find(b => b.type === buildingType);
    
    return storageBuilding ? storageBuilding.id : 'general';
  }

  private _findOptimalBuildingLocation(buildingType: Building['type']): { x: number; y: number } {
    // Simple placement algorithm
    const existingBuildings = Array.from(this.buildings.values());
    
    let x = 0, y = 0;
    const spacing = 8; // Minimum spacing between buildings

    for (let attempt = 0; attempt < 10; attempt++) {
      x = (ChaosSys.getInstance().next() - 0.5) * 30;
      y = (ChaosSys.getInstance().next() - 0.5) * 30;

      // Check for collisions
      const collision = existingBuildings.some(building => {
        const dx = Math.abs(building.position.x - x);
        const dy = Math.abs(building.position.y - y);
        return dx < spacing && dy < spacing;
      });

      if (!collision) break;
    }

    return { x, y };
  }

  private _isResourceCritical(type: Resource['type'], quantity: number): boolean {
    const criticalLevels = {
      food: 10,
      water: 20,
      fuel: 5,
      wood: 10
    };

    return (criticalLevels[type] || 0) > quantity;
  }

  private _calculateStorageEfficiency(): number {
    let totalCapacity = 0;
    let usedCapacity = 0;

    for (const building of this.buildings.values()) {
      if (building.type === 'storage') {
        const capacity = building.size.width * building.size.height * building.size.depth;
        totalCapacity += capacity;
        usedCapacity += capacity * building.functionality;
      }
    }

    return totalCapacity > 0 ? usedCapacity / totalCapacity : 0;
  }

  getHomesteadReport(): {
    state: HomesteadState;
    buildings: any;
    resources: any;
  } {
    return {
      state: { ...this.state },
      buildings: this.getBuildingStatus(),
      resources: this.getResourceStatus()
    };
  }

  resetHomestead(): void {
    this.buildings.clear();
    this.resources.clear();
    this.state = {
      security_level: 0.5,
      comfort_level: 0.6,
      self_sufficiency: 0.4,
      maintenance_needs: 0.2,
      population_capacity: 4,
      current_population: 2
    };
    this._initializeHomestead();
  }
}
