
/**
 * ECOSYSTEM
 * 
 * Environmental simulation with flora, fauna, and climate
 * Resource regeneration, population dynamics, seasonal cycles
 */

export interface Organism {
  id: string;
  species: string;
  type: 'plant' | 'animal' | 'fungus' | 'bacteria';
  population: number;
  health: number; // 0-1
  growth_rate: number; // individuals per time unit
  resource_needs: Map<string, number>;
  resource_provides: Map<string, number>;
}

export interface ClimateState {
  temperature: number; // Celsius
  humidity: number; // 0-1
  precipitation: number; // mm/hour
  wind_speed: number; // m/s
  season: 'spring' | 'summer' | 'autumn' | 'winter';
  day_length: number; // hours
}

export interface EcosystemBalance {
  biodiversity: number; // 0-1, species variety
  resource_abundance: number; // 0-1, available resources
  population_stability: number; // 0-1, ecosystem health
  pollution_level: number; // 0-1, contamination
}

export class Ecosystem {
  private organisms: Map<string, Organism>;
  private climate: ClimateState;
  private balance: EcosystemBalance;
  private time: number;
  private resourcePools: Map<string, number>;

  constructor() {
    this.organisms = new Map();
    this.resourcePools = new Map();
    this.time = 0;

    this.climate = {
      temperature: 15.0,
      humidity: 0.6,
      precipitation: 0.1,
      wind_speed: 2.0,
      season: 'spring',
      day_length: 12
    };

    this.balance = {
      biodiversity: 0.7,
      resource_abundance: 0.6,
      population_stability: 0.8,
      pollution_level: 0.1
    };

    this._initializeEcosystem();
  }

  update(dtSeconds: number): void {
    this.time += dtSeconds;

    // Update climate
    this._updateClimate(dtSeconds);

    // Update organism populations
    this._updateOrganisms(dtSeconds);

    // Process resource cycles
    this._processResourceCycles(dtSeconds);

    // Calculate ecosystem balance
    this._calculateEcosystemBalance();

    // Seasonal changes
    this._processSeasonalChanges();
  }

  addOrganism(organism: Omit<Organism, 'id'>): string {
    const id = `organism_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullOrganism: Organism = {
      id,
      ...organism
    };

    this.organisms.set(id, fullOrganism);
    return id;
  }

  harvestResource(resourceType: string, amount: number): boolean {
    const available = this.resourcePools.get(resourceType) || 0;
    if (available < amount) return false;

    this.resourcePools.set(resourceType, available - amount);

    // Affect organisms that provide this resource
    for (const organism of this.organisms.values()) {
      if (organism.resource_provides.has(resourceType)) {
        organism.health = Math.max(0, organism.health - amount * 0.001);
        organism.population = Math.max(0, organism.population - amount * 0.01);
      }
    }

    return true;
  }

  getEcosystemState(): {
    climate: ClimateState;
    balance: EcosystemBalance;
    species_count: number;
    total_population: number;
  } {
    const speciesCount = new Set(Array.from(this.organisms.values()).map(o => o.species)).size;
    const totalPopulation = Array.from(this.organisms.values())
      .reduce((sum, o) => sum + o.population, 0);

    return {
      climate: { ...this.climate },
      balance: { ...this.balance },
      species_count: speciesCount,
      total_population: totalPopulation
    };
  }

  getAvailableResources(): Map<string, number> {
    return new Map(this.resourcePools);
  }

  private _initializeEcosystem(): void {
    // Add initial organisms
    this.addOrganism({
      species: 'Oak Tree',
      type: 'plant',
      population: 50,
      health: 0.8,
      growth_rate: 0.1,
      resource_needs: new Map([['water', 1], ['nutrients', 0.5]]),
      resource_provides: new Map([['wood', 2], ['oxygen', 5]])
    });

    this.addOrganism({
      species: 'Rabbit',
      type: 'animal',
      population: 20,
      health: 0.9,
      growth_rate: 0.2,
      resource_needs: new Map([['plants', 0.3], ['water', 0.2]]),
      resource_provides: new Map([['food', 1], ['fur', 0.5]])
    });

    // Initialize resource pools
    this.resourcePools.set('water', 1000);
    this.resourcePools.set('nutrients', 500);
    this.resourcePools.set('oxygen', 2000);
  }

  private _updateClimate(dtSeconds: number): void {
    // Simple climate model
    const dayInYear = (this.time / 86400) % 365; // Day of year
    
    // Determine season
    if (dayInYear < 90) this.climate.season = 'spring';
    else if (dayInYear < 180) this.climate.season = 'summer';
    else if (dayInYear < 270) this.climate.season = 'autumn';
    else this.climate.season = 'winter';

    // Temperature variation
    const seasonalTemp = {
      spring: 15,
      summer: 25,
      autumn: 10,
      winter: 0
    };

    this.climate.temperature = seasonalTemp[this.climate.season] + Math.sin(this.time / 43200) * 5;

    // Day length
    this.climate.day_length = 12 + Math.sin((dayInYear - 80) / 365 * Math.PI) * 4;
  }

  private _updateOrganisms(dtSeconds: number): void {
    for (const organism of this.organisms.values()) {
      // Resource consumption
      for (const [resource, need] of organism.resource_provides) {
        const available = this.resourcePools.get(resource) || 0;
        const consumption = need * organism.population * dtSeconds * 0.001;
        
        if (available >= consumption) {
          this.resourcePools.set(resource, available - consumption);
        } else {
          // Resource scarcity affects health and population
          organism.health = Math.max(0, organism.health - (available - consumption) * 0.01);
          organism.population = Math.max(0, organism.population - (available - consumption) * 0.005);
        }
      }

      // Population dynamics
      const growthFactor = organism.health * this._getEnvironmentalSuitability(organism);
      const populationChange = organism.growth_rate * growthFactor * organism.population * dtSeconds * 0.0001;
      organism.population = Math.max(0, organism.population + populationChange);

      // Health regeneration
      if (organism.health < 1) {
        organism.health = Math.min(1, organism.health + dtSeconds * 0.0001);
      }
    }
  }

  private _processResourceCycles(dtSeconds: number): void {
    // Natural resource regeneration
    const regenerationRates = {
      water: 0.1,
      nutrients: 0.05,
      oxygen: 0.2
    };

    for (const [resource, rate] of Object.entries(regenerationRates)) {
      const current = this.resourcePools.get(resource) || 0;
      const maxPool = { water: 2000, nutrients: 1000, oxygen: 5000 }[resource] || 1000;
      const regeneration = rate * dtSeconds * (1 - this.balance.pollution_level);
      
      this.resourcePools.set(resource, Math.min(maxPool, current + regeneration));
    }
  }

  private _calculateEcosystemBalance(): void {
    const species = Array.from(this.organisms.values());
    
    // Biodiversity
    this.balance.biodiversity = Math.min(1, species.length / 20); // Normalized to 20 species

    // Resource abundance
    const totalResources = Array.from(this.resourcePools.values())
      .reduce((sum, amount) => sum + amount, 0);
    this.balance.resource_abundance = Math.min(1, totalResources / 10000);

    // Population stability
    const healthVariance = this._calculatePopulationVariance(species);
    this.balance.population_stability = Math.max(0, 1 - healthVariance);

    // Pollution (simplified)
    this.balance.pollution_level = Math.max(0, this.balance.pollution_level - 0.0001); // Natural decay
  }

  private _getEnvironmentalSuitability(organism: Organism): number {
    let suitability = 1.0;

    // Temperature suitability
    const optimalTemp = { plant: 20, animal: 15, fungus: 10, bacteria: 25 }[organism.type];
    const tempDiff = Math.abs(this.climate.temperature - optimalTemp);
    suitability *= Math.max(0, 1 - tempDiff / 20);

    // Seasonal suitability
    const seasonalBonus = {
      spring: organism.type === 'plant' ? 1.2 : 1.0,
      summer: organism.type === 'animal' ? 1.1 : 0.9,
      autumn: organism.type === 'fungus' ? 1.3 : 1.0,
      winter: organism.type === 'bacteria' ? 0.8 : 0.7
    };

    suitability *= seasonalBonus[this.climate.season];

    return suitability;
  }

  private _calculatePopulationVariance(organisms: Organism[]): number {
    if (organisms.length === 0) return 1;

    const populations = organisms.map(o => o.health);
    const mean = populations.reduce((sum, p) => sum + p, 0) / populations.length;
    const variance = populations.reduce((sum, p) => sum + Math.pow(p - mean, 2), 0) / populations.length;
    
    return Math.min(1, Math.sqrt(variance));
  }

  private _processSeasonalChanges(): void {
    // Seasonal effects on different organism types
    for (const organism of this.organisms.values()) {
      switch (this.climate.season) {
        case 'spring':
          if (organism.type === 'plant') {
            organism.growth_rate *= 1.5;
          }
          break;
        case 'summer':
          if (organism.type === 'animal') {
            organism.growth_rate *= 1.2;
          }
          break;
        case 'autumn':
          if (organism.type === 'fungus') {
            organism.growth_rate *= 1.8;
          }
          break;
        case 'winter':
          if (organism.type === 'plant') {
            organism.growth_rate *= 0.3;
          }
          break;
      }
    }
  }

  getEcosystemReport(): {
    state: any;
    organisms: any;
    resources: any;
  } {
    return {
      state: this.getEcosystemState(),
      organisms: {
        total_species: new Set(Array.from(this.organisms.values()).map(o => o.species)).size,
        average_health: Array.from(this.organisms.values()).reduce((sum, o) => sum + o.health, 0) / this.organisms.size,
        total_population: Array.from(this.organisms.values()).reduce((sum, o) => sum + o.population, 0)
      },
      resources: Object.fromEntries(this.resourcePools)
    };
  }

  resetEcosystem(): void {
    this.organisms.clear();
    this.resourcePools.clear();
    this.time = 0;
    this.balance = {
      biodiversity: 0.7,
      resource_abundance: 0.6,
      population_stability: 0.8,
      pollution_level: 0.1
    };
    this._initializeEcosystem();
  }
}
