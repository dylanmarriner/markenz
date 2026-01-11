
/**
 * PLANETARY SYSTEM
 * 
 * Implements the Planet, Weather, Time, and Ecology as specified in Master Plan Part 4
 * Provides the physical environment for the biological simulation
 */

import { ChaosSys } from '../../../chaos/chaos-sys';

export interface PlanetaryConstants {
  GRAVITY: number;           // m/s²
  ATMOSPHERIC_PRESSURE: number; // kPa
  DAY_LENGTH: number;        // hours
  YEAR_LENGTH: number;       // days
  AXIAL_TILT: number;        // degrees
  ORBITAL_RADIUS: number;    // AU
  SOLAR_CONSTANT: number;    // W/m²
  MEAN_TEMPERATURE: number;  // Celsius
  WATER_COVERAGE: number;    // 0-1
}

export interface WeatherState {
  temperature: number;       // Celsius
  humidity: number;          // 0-1
  pressure: number;          // kPa
  windSpeed: number;         // m/s
  windDirection: number;     // degrees 0-360
  precipitation: number;     // mm/hour
  cloudCover: number;        // 0-1
  visibility: number;        // km
  uvIndex: number;           // 0-11
  timestamp: Date;
}

export interface EcologicalZone {
  id: string;
  name: string;
  type: 'temperate_forest' | 'grassland' | 'desert' | 'tundra' | 'ocean' | 'freshwater' | 'urban';
  area: number;              // km²
  biodiversity: number;      // 0-1
  primaryResources: ResourceNode[];
  climate: ClimateProfile;
  population: OrganismPopulation[];
}

export interface ResourceNode {
  id: string;
  type: 'water' | 'food' | 'shelter' | 'energy' | 'material' | 'information';
  quantity: number;
  quality: number;           // 0-1
  location: { x: number; y: number; z: number };
  accessibility: number;    // 0-1
  regeneration: number;     // rate per hour
  depletion: number;         // current depletion 0-1
}

export interface OrganismPopulation {
  species: string;
  count: number;
  health: number;            // 0-1
  growthRate: number;        // individuals per hour
  resourceNeeds: Map<string, number>;
  resourceProvides: Map<string, number>;
  location: { x: number; y: number };
}

export interface ClimateProfile {
  baseTemperature: number;   // Celsius
  temperatureVariation: number; // ± degrees
  seasonalAmplitude: number; // seasonal variation
  humidityBaseline: number;   // 0-1
  precipitationFrequency: number; // probability per hour
  stormProbability: number;  // 0-1
}

export interface TimeState {
  currentTime: Date;
  dayOfYear: number;         // 1-365
  hour: number;              // 0-23
  season: 'spring' | 'summer' | 'autumn' | 'winter';
  lunarPhase: 'new' | 'waxing_crescent' | 'first_quarter' | 'waxing_gibbous' | 'full' | 'waning_gibbous' | 'last_quarter' | 'waning_crescent';
  solarElevation: number;    // degrees above horizon
  daylightHours: number;     // hours
}

export class PlanetarySystem {
  private constants: PlanetaryConstants;
  private weatherState: WeatherState;
  private ecologicalZones: Map<string, EcologicalZone> = new Map();
  private timeState: TimeState;
  private weatherHistory: WeatherState[] = [];
  private simulationSpeed: number = 1; // 1 = real-time
  private chaosSys: ChaosSys;

  constructor() {
    this.constants = this.initializePlanetaryConstants();
    this.weatherState = this.initializeWeatherState();
    this.timeState = this.initializeTimeState();
    this.initializeEcologicalZones();
    this.chaosSys = ChaosSys.getInstance();
  }

  /**
   * Initialize Earth-like planetary constants
   */
  private initializePlanetaryConstants(): PlanetaryConstants {
    return {
      GRAVITY: 9.81,
      ATMOSPHERIC_PRESSURE: 101.325,
      DAY_LENGTH: 24,
      YEAR_LENGTH: 365.25,
      AXIAL_TILT: 23.5,
      ORBITAL_RADIUS: 1.0,
      SOLAR_CONSTANT: 1361,
      MEAN_TEMPERATURE: 15,
      WATER_COVERAGE: 0.71
    };
  }

  /**
   * Initialize weather state
   */
  private initializeWeatherState(): WeatherState {
    return {
      temperature: this.constants.MEAN_TEMPERATURE + this.chaosSys.generateChaos('creativity') * 10,
      humidity: 0.5 + this.chaosSys.boundedRandom(0.3),
      pressure: this.constants.ATMOSPHERIC_PRESSURE + this.chaosSys.generateChaos('creativity') * 5,
      windSpeed: this.chaosSys.boundedRandom(10),
      windDirection: this.chaosSys.boundedRandom(360),
      precipitation: 0,
      cloudCover: this.chaosSys.boundedRandom(0.7),
      visibility: 10 + this.chaosSys.boundedRandom(20),
      uvIndex: this.chaosSys.boundedRandom(8),
      timestamp: new Date()
    };
  }

  /**
   * Initialize time state
   */
  private initializeTimeState(): TimeState {
    const now = new Date();
    const dayOfYear = Math.floor((now.getTime() - new Date(now.getFullYear(), 0, 0).getTime()) / (1000 * 60 * 60 * 24));
    
    return {
      currentTime: now,
      dayOfYear,
      hour: now.getHours(),
      season: this.calculateSeason(dayOfYear),
      lunarPhase: this.calculateLunarPhase(now),
      solarElevation: this.calculateSolarElevation(now.getHours(), dayOfYear),
      daylightHours: this.calculateDaylightHours(dayOfYear)
    };
  }

  /**
   * Initialize ecological zones
   */
  private initializeEcologicalZones(): void {
    // Temperate Forest Zone
    const temperateForest: EcologicalZone = {
      id: 'temperate_forest_01',
      name: 'Northern Temperate Forest',
      type: 'temperate_forest',
      area: 1000, // km²
      biodiversity: 0.8,
      primaryResources: [
        {
          id: 'fresh_water_stream',
          type: 'water',
          quantity: 1000,
          quality: 0.9,
          location: { x: 500, y: 500, z: 0 },
          accessibility: 0.7,
          regeneration: 50,
          depletion: 0
        },
        {
          id: 'berry_bushes',
          type: 'food',
          quantity: 500,
          quality: 0.7,
          location: { x: 300, y: 400, z: 0 },
          accessibility: 0.8,
          regeneration: 10,
          depletion: 0.2
        },
        {
          id: 'wood_resources',
          type: 'material',
          quantity: 2000,
          quality: 0.8,
          location: { x: 600, y: 300, z: 0 },
          accessibility: 0.6,
          regeneration: 5,
          depletion: 0.1
        }
      ],
      climate: {
        baseTemperature: 12,
        temperatureVariation: 15,
        seasonalAmplitude: 20,
        humidityBaseline: 0.7,
        precipitationFrequency: 0.3,
        stormProbability: 0.1
      },
      population: []
    };

    // Grassland Zone
    const grassland: EcologicalZone = {
      id: 'grassland_01',
      name: 'Central Grassland',
      type: 'grassland',
      area: 800,
      biodiversity: 0.6,
      primaryResources: [
        {
          id: 'grassland_water_hole',
          type: 'water',
          quantity: 500,
          quality: 0.6,
          location: { x: 400, y: 400, z: 0 },
          accessibility: 0.9,
          regeneration: 30,
          depletion: 0.3
        },
        {
          id: 'wild_grains',
          type: 'food',
          quantity: 800,
          quality: 0.5,
          location: { x: 200, y: 600, z: 0 },
          accessibility: 0.9,
          regeneration: 20,
          depletion: 0.1
        }
      ],
      climate: {
        baseTemperature: 18,
        temperatureVariation: 20,
        seasonalAmplitude: 25,
        humidityBaseline: 0.4,
        precipitationFrequency: 0.2,
        stormProbability: 0.15
      },
      population: []
    };

    // Urban Zone (House/Shed area)
    const urban: EcologicalZone = {
      id: 'urban_01',
      name: 'Homestead Area',
      type: 'urban',
      area: 50,
      biodiversity: 0.3,
      primaryResources: [
        {
          id: 'municipal_water',
          type: 'water',
          quantity: 2000,
          quality: 0.95,
          location: { x: 25, y: 25, z: 0 },
          accessibility: 1.0,
          regeneration: 100,
          depletion: 0
        },
        {
          id: 'stored_food',
          type: 'food',
          quantity: 1000,
          quality: 0.8,
          location: { x: 25, y: 25, z: 0 },
          accessibility: 0.9,
          regeneration: 0,
          depletion: 0.4
        },
        {
          id: 'electrical_power',
          type: 'energy',
          quantity: 5000,
          quality: 1.0,
          location: { x: 25, y: 25, z: 0 },
          accessibility: 1.0,
          regeneration: 100,
          depletion: 0
        }
      ],
      climate: {
        baseTemperature: 20,
        temperatureVariation: 10,
        seasonalAmplitude: 15,
        humidityBaseline: 0.5,
        precipitationFrequency: 0.15,
        stormProbability: 0.05
      },
      population: []
    };

    this.ecologicalZones.set(temperateForest.id, temperateForest);
    this.ecologicalZones.set(grassland.id, grassland);
    this.ecologicalZones.set(urban.id, urban);
  }

  /**
   * Update planetary simulation by one time step
   */
  update(deltaTime: number = 1): void {
    // Update time
    this.updateTime(deltaTime);

    // Update weather
    this.updateWeather(deltaTime);

    // Update ecological zones
    this.updateEcologicalZones(deltaTime);

    // Store weather history (keep last 1000 entries)
    this.weatherHistory.push({ ...this.weatherState });
    if (this.weatherHistory.length > 1000) {
      this.weatherHistory.shift();
    }
  }

  /**
   * Update time state
   */
  private updateTime(deltaTime: number): void {
    const newTime = new Date(this.timeState.currentTime.getTime() + deltaTime * 3600000 * this.simulationSpeed);
    
    this.timeState.currentTime = newTime;
    this.timeState.hour = newTime.getHours();
    this.timeState.dayOfYear = Math.floor((newTime.getTime() - new Date(newTime.getFullYear(), 0, 0).getTime()) / (1000 * 60 * 60 * 24));
    this.timeState.season = this.calculateSeason(this.timeState.dayOfYear);
    this.timeState.lunarPhase = this.calculateLunarPhase(newTime);
    this.timeState.solarElevation = this.calculateSolarElevation(this.timeState.hour, this.timeState.dayOfYear);
    this.timeState.daylightHours = this.calculateDaylightHours(this.timeState.dayOfYear);
  }

  /**
   * Update weather based on time and climate
   */
  private updateWeather(deltaTime: number): void {
    const currentZone = this.getCurrentEcologicalZone();
    const climate = currentZone.climate;

    // Calculate base temperature with seasonal and daily variation
    const seasonalTemp = climate.baseTemperature + 
      climate.seasonalAmplitude * Math.sin(2 * Math.PI * (this.timeState.dayOfYear - 80) / this.constants.YEAR_LENGTH);
    const dailyTemp = climate.temperatureVariation * Math.sin(2 * Math.PI * (this.timeState.hour - 6) / this.constants.DAY_LENGTH);
    
    this.weatherState.temperature = seasonalTemp + dailyTemp + this.chaosSys.generateChaos('creativity') * 2;

    // Update humidity
    this.weatherState.humidity = climate.humidityBaseline + this.chaosSys.generateChaos('creativity') * 0.2;
    this.weatherState.humidity = Math.max(0, Math.min(1, this.weatherState.humidity));

    // Update precipitation
    if (this.chaosSys.boundedRandom(1) < climate.precipitationFrequency * deltaTime) {
      this.weatherState.precipitation = this.chaosSys.boundedRandom(10); // mm/hour
    } else {
      this.weatherState.precipitation *= 0.9; // Gradual stop
    }

    // Update wind
    this.weatherState.windSpeed = 5 + this.chaosSys.boundedRandom(15);
    this.weatherState.windDirection = (this.weatherState.windDirection + this.chaosSys.generateChaos('creativity') * 30 + 360) % 360;

    // Update cloud cover based on humidity and precipitation
    if (this.weatherState.precipitation > 0) {
      this.weatherState.cloudCover = Math.min(1, this.weatherState.cloudCover + 0.1);
    } else {
      this.weatherState.cloudCover = Math.max(0, this.weatherState.cloudCover - 0.05);
    }

    // Update visibility based on precipitation and humidity
    this.weatherState.visibility = 20 - this.weatherState.precipitation * 2 - this.weatherState.humidity * 5;
    this.weatherState.visibility = Math.max(0.5, this.weatherState.visibility);

    // Update UV index based on cloud cover and solar elevation
    const baseUV = Math.max(0, Math.sin(this.timeState.solarElevation * Math.PI / 180) * 10);
    this.weatherState.uvIndex = baseUV * (1 - this.weatherState.cloudCover * 0.8);

    // Update pressure
    this.weatherState.pressure = this.constants.ATMOSPHERIC_PRESSURE + this.chaosSys.generateChaos('creativity') * 2;

    this.weatherState.timestamp = new Date();
  }

  /**
   * Update ecological zones
   */
  private updateEcologicalZones(deltaTime: number): void {
    this.ecologicalZones.forEach(zone => {
      // Regenerate resources
      zone.primaryResources.forEach(resource => {
        if (resource.depletion > 0) {
          resource.depletion = Math.max(0, resource.depletion - resource.regeneration * deltaTime / 100);
        }
      });

      // Update organism populations
      zone.population.forEach(population => {
        // Simple population dynamics
        const resourcesAvailable = this.calculateResourceAvailability(zone, population);
        const growthFactor = Math.min(1, resourcesAvailable);
        
        population.health *= (0.99 + growthFactor * 0.01); // Slowly change health
        population.growthRate = population.health * 0.1 * growthFactor;
        
        population.count += population.growthRate * deltaTime;
        population.count = Math.max(1, population.count); // Minimum 1 individual
      });
    });
  }

  /**
   * Calculate resource availability for a population
   */
  private calculateResourceAvailability(zone: EcologicalZone, population: OrganismPopulation): number {
    let availability = 0;
    let totalNeeds = 0;

    population.resourceNeeds.forEach((need, resourceType) => {
      totalNeeds += need;
      
      const resource = zone.primaryResources.find(r => r.type === resourceType);
      if (resource) {
        const available = resource.quantity * (1 - resource.depletion);
        availability += Math.min(need, available) / need;
      }
    });

    return totalNeeds > 0 ? availability / population.resourceNeeds.size : 1;
  }

  /**
   * Get current ecological zone based on agent location
   */
  getCurrentEcologicalZone(): EcologicalZone {
    // Default to urban zone for now
    return this.ecologicalZones.get('urban_01') || Array.from(this.ecologicalZones.values())[0];
  }

  /**
   * Calculate season from day of year
   */
  private calculateSeason(dayOfYear: number): 'spring' | 'summer' | 'autumn' | 'winter' {
    if (dayOfYear >= 80 && dayOfYear < 172) return 'spring';
    if (dayOfYear >= 172 && dayOfYear < 266) return 'summer';
    if (dayOfYear >= 266 && dayOfYear < 355) return 'autumn';
    return 'winter';
  }

  /**
   * Calculate lunar phase
   */
  private calculateLunarPhase(date: Date): 'new' | 'waxing_crescent' | 'first_quarter' | 'waxing_gibbous' | 'full' | 'waning_gibbous' | 'last_quarter' | 'waning_crescent' {
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const day = date.getDate();
    
    // Simple approximation
    const c = Math.floor(365.25 * year);
    const e = Math.floor(30.6 * month);
    const jd = c + e + day - 694039.09; // Julian date
    const phase = (jd / 29.53) % 1; // Lunar cycle
    
    if (phase < 0.125) return 'new';
    if (phase < 0.25) return 'waxing_crescent';
    if (phase < 0.375) return 'first_quarter';
    if (phase < 0.5) return 'waxing_gibbous';
    if (phase < 0.625) return 'full';
    if (phase < 0.75) return 'waning_gibbous';
    if (phase < 0.875) return 'last_quarter';
    return 'waning_crescent';
  }

  /**
   * Calculate solar elevation
   */
  private calculateSolarElevation(hour: number, dayOfYear: number): number {
    const solarDeclination = 23.45 * Math.sin(2 * Math.PI * (284 + dayOfYear) / 365);
    const hourAngle = 15 * (hour - 12);
    
    const elevation = Math.asin(
      Math.sin(solarDeclination * Math.PI / 180) * Math.sin(this.constants.AXIAL_TILT * Math.PI / 180) +
      Math.cos(solarDeclination * Math.PI / 180) * Math.cos(this.constants.AXIAL_TILT * Math.PI / 180) * Math.cos(hourAngle * Math.PI / 180)
    ) * 180 / Math.PI;
    
    return Math.max(0, elevation);
  }

  /**
   * Calculate daylight hours
   */
  private calculateDaylightHours(dayOfYear: number): number {
    const solarDeclination = 23.45 * Math.sin(2 * Math.PI * (284 + dayOfYear) / 365);
    const latitude = 45; // Assume 45° latitude
    
    const hourAngle = Math.acos(-Math.tan(latitude * Math.PI / 180) * Math.tan(solarDeclination * Math.PI / 180));
    return 2 * hourAngle * 12 / Math.PI;
  }

  /**
   * Getters for current state
   */
  getWeatherState(): WeatherState {
    return { ...this.weatherState };
  }

  getTimeState(): TimeState {
    return { ...this.timeState };
  }

  getEcologicalZones(): EcologicalZone[] {
    return Array.from(this.ecologicalZones.values());
  }

  getPlanetaryConstants(): PlanetaryConstants {
    return { ...this.constants };
  }

  getWeatherHistory(hours: number = 24): WeatherState[] {
    const entries = Math.floor(hours / this.simulationSpeed);
    return this.weatherHistory.slice(-entries);
  }

  /**
   * Set simulation speed
   */
  setSimulationSpeed(speed: number): void {
    this.simulationSpeed = Math.max(0.1, Math.min(1000, speed));
  }

  /**
   * Get resource by type and location
   */
  getResource(type: string, location?: { x: number; y: number }): ResourceNode | null {
    const zone = location ? this.getEcologicalZoneAt(location) : this.getCurrentEcologicalZone();
    return zone.primaryResources.find(r => r.type === type) || null;
  }

  /**
   * Get ecological zone at location
   */
  getEcologicalZoneAt(location: { x: number; y: number }): EcologicalZone {
    // Simple location-based zone selection
    if (location.x < 100 && location.y < 100) {
      return this.ecologicalZones.get('urban_01')!;
    }
    if (location.x < 500 && location.y < 500) {
      return this.ecologicalZones.get('temperate_forest_01')!;
    }
    return this.ecologicalZones.get('grassland_01')!;
  }

  /**
   * Consume resource
   */
  consumeResource(resourceId: string, amount: number): boolean {
    for (const zone of this.ecologicalZones.values()) {
      const resource = zone.primaryResources.find(r => r.id === resourceId);
      if (resource && resource.quantity >= amount) {
        resource.quantity -= amount;
        resource.depletion = Math.min(1, resource.depletion + amount / (resource.quantity + amount));
        return true;
      }
    }
    return false;
  }
}

export default PlanetarySystem;
