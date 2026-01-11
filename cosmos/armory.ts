/**
 * ARMORY
 * 
 * Complete weapons and military equipment system
 * Firearms, artillery, armor, vehicles, and defense items
 */

import { ChaosSys } from '../chaos/chaos-sys';

export interface Firearm {
  id: string;
  name: string;
  type: 'pistol' | 'rifle' | 'shotgun' | 'sniper' | 'machine_gun' | 'submachine_gun';
  caliber: string;
  magazine_capacity: number;
  fire_rate: number; // rounds per minute
  effective_range: number; // meters
  condition: number; // 0-1
  ammunition_loaded: number;
  attachments: string[];
}

export interface Ammunition {
  id: string;
  type: string; // caliber
  quantity: number;
  armor_penetration: number; // 0-1
  damage_type: 'ballistic' | 'explosive' | 'incendiary' | 'armor_piercing';
}

export interface Armor {
  id: string;
  name: string;
  type: 'vest' | 'helmet' | 'plate_carrier' | 'full_body' | 'tactical';
  protection_level: number; // 0-1
  coverage: string[]; // head, torso, limbs
  condition: number;
  weight: number; // kg
}

export interface MeleeWeapon {
  id: string;
  name: string;
  type: 'sword' | 'axe' | 'spear' | 'dagger' | 'mace' | 'halberd';
  era: 'ancient' | 'medieval' | 'modern' | 'ceremonial';
  damage: number;
  reach: number; // meters
  material: string;
  condition: number;
}

export interface MilitaryVehicle {
  id: string;
  name: string;
  type: 'helicopter' | 'jet' | 'tank' | 'apc' | 'drone';
  armament: string[];
  crew_capacity: number;
  max_speed: number; // km/h
  operational_range: number; // km
  fuel_type: string;
  condition: number;
}

export interface DefenseSystem {
  id: string;
  name: string;
  type: 'turret' | 'missile_system' | 'radar' | 'jamming' | 'countermeasure';
  effectiveness: number; // 0-1
  power_requirement: number;
  active: boolean;
}

export class Armory {
  private firearms: Map<string, Firearm>;
  private ammunition: Map<string, Ammunition>;
  private armor: Map<string, Armor>;
  private meleeWeapons: Map<string, MeleeWeapon>;
  private militaryVehicles: Map<string, MilitaryVehicle>;
  private defenseSystems: Map<string, DefenseSystem>;
  private firingRange: FiringRange;

  constructor() {
    this.firearms = new Map();
    this.ammunition = new Map();
    this.armor = new Map();
    this.meleeWeapons = new Map();
    this.militaryVehicles = new Map();
    this.defenseSystems = new Map();
    this.firingRange = new FiringRange();

    this._initializeArmory();
  }

  update(dtSeconds: number): void {
    // Update weapon conditions
    this._updateWeaponConditions(dtSeconds);

    // Update ammunition storage
    this._updateAmmunitionStorage(dtSeconds);

    // Update armor integrity
    this._updateArmorIntegrity(dtSeconds);

    // Update vehicle maintenance
    this._updateVehicleMaintenance(dtSeconds);

    // Update defense systems
    this._updateDefenseSystems(dtSeconds);

    // Update firing range
    this.firingRange.update(dtSeconds);
  }

  addFirearm(firearm: Omit<Firearm, 'id'>): string {
    const id = `firearm_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullFirearm: Firearm = {
      id,
      ...firearm,
      condition: 1.0,
      ammunition_loaded: 0
    };

    this.firearms.set(id, fullFirearm);
    return id;
  }

  addAmmunition(ammo: Omit<Ammunition, 'id'>): string {
    const id = `ammo_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullAmmo: Ammunition = {
      id,
      ...ammo
    };

    this.ammunition.set(id, fullAmmo);
    return id;
  }

  loadFirearm(firearmId: string, ammoId: string, rounds: number): boolean {
    const firearm = this.firearms.get(firearmId);
    const ammo = this.ammunition.get(ammoId);

    if (!firearm || !ammo) return false;
    if (ammo.type !== firearm.caliber) return false;
    if (ammo.quantity < rounds) return false;
    if (rounds > firearm.magazine_capacity) return false;

    firearm.ammunition_loaded = rounds;
    ammo.quantity -= rounds;

    if (ammo.quantity <= 0) {
      this.ammunition.delete(ammoId);
    }

    return true;
  }

  fireWeapon(firearmId: string, rounds: number): boolean {
    const firearm = this.firearms.get(firearmId);
    if (!firearm) return false;
    if (firearm.ammunition_loaded < rounds) return false;

    firearm.ammunition_loaded -= rounds;
    
    // Weapon wear from firing
    firearm.condition = Math.max(0, firearm.condition - rounds * 0.0001);

    return true;
  }

  getArmoryReport(): {
    firearms: any;
    ammunition: any;
    armor: any;
    melee_weapons: any;
    vehicles: any;
    defense_systems: any;
    firing_range: any;
  } {
    return {
      firearms: this.getFirearmsStatus(),
      ammunition: this.getAmmunitionStatus(),
      armor: this.getArmorStatus(),
      melee_weapons: this.getMeleeWeaponsStatus(),
      vehicles: this.getVehiclesStatus(),
      defense_systems: this.getDefenseSystemsStatus(),
      firing_range: this.firingRange.getRangeStatus()
    };
  }

  private _initializeArmory(): void {
    // Initialize firearms - multiple of every caliber
    this._initializeFirearms();
    
    // Initialize ammunition - thousands of rounds
    this._initializeAmmunition();
    
    // Initialize armor and defense items
    this._initializeArmor();
    
    // Initialize medieval weapons
    this._initializeMeleeWeapons();
    
    // Initialize military vehicles
    this._initializeMilitaryVehicles();
    
    // Initialize defense systems
    this._initializeDefenseSystems();
  }

  private _initializeFirearms(): void {
    // Pistols
    this.addFirearm({
      name: 'Glock 17',
      type: 'pistol',
      caliber: '9mm',
      magazine_capacity: 17,
      fire_rate: 1200,
      effective_range: 50,
      ammunition_loaded: 0,
      attachments: ['flashlight', 'laser_sight']
    });

    this.addFirearm({
      name: 'Beretta M9',
      type: 'pistol',
      caliber: '9mm',
      magazine_capacity: 15,
      fire_rate: 1200,
      effective_range: 50,
      ammunition_loaded: 0,
      attachments: ['flashlight']
    });

    // Rifles
    this.addFirearm({
      name: 'M4 Carbine',
      type: 'rifle',
      caliber: '5.56mm',
      magazine_capacity: 30,
      fire_rate: 700,
      effective_range: 500,
      ammunition_loaded: 0,
      attachments: ['scope', 'foregrip', 'flashlight']
    });

    this.addFirearm({
      name: 'AK-47',
      type: 'rifle',
      caliber: '7.62mm',
      magazine_capacity: 30,
      fire_rate: 600,
      effective_range: 300,
      ammunition_loaded: 0,
      attachments: ['bayonet']
    });

    // Sniper rifles
    this.addFirearm({
      name: 'Barrett M82',
      type: 'sniper',
      caliber: '.50 BMG',
      magazine_capacity: 10,
      fire_rate: 10,
      effective_range: 1800,
      ammunition_loaded: 0,
      attachments: ['thermal_scope', 'bipod']
    });

    // Shotguns
    this.addFirearm({
      name: 'Remington 870',
      type: 'shotgun',
      caliber: '12 gauge',
      magazine_capacity: 8,
      fire_rate: 30,
      effective_range: 50,
      ammunition_loaded: 0,
      attachments: ['flashlight']
    });

    // Machine guns
    this.addFirearm({
      name: 'M240B',
      type: 'machine_gun',
      caliber: '7.62mm',
      magazine_capacity: 100,
      fire_rate: 650,
      effective_range: 1100,
      ammunition_loaded: 0,
      attachments: ['bipod', 'heat_shield']
    });
  }

  private _initializeAmmunition(): void {
    // Thousands of rounds for each caliber
    this.addAmmunition({
      type: '9mm',
      quantity: 10000,
      armor_penetration: 0.3,
      damage_type: 'ballistic'
    });

    this.addAmmunition({
      type: '5.56mm',
      quantity: 15000,
      armor_penetration: 0.4,
      damage_type: 'ballistic'
    });

    this.addAmmunition({
      type: '7.62mm',
      quantity: 12000,
      armor_penetration: 0.5,
      damage_type: 'ballistic'
    });

    this.addAmmunition({
      type: '.50 BMG',
      quantity: 500,
      armor_penetration: 0.9,
      damage_type: 'armor_piercing'
    });

    this.addAmmunition({
      type: '12 gauge',
      quantity: 2000,
      armor_penetration: 0.2,
      damage_type: 'ballistic'
    });
  }

  private _initializeArmor(): void {
    this.armor.set('vest_1', {
      id: 'vest_1',
      name: 'Level IV Body Armor',
      type: 'vest',
      protection_level: 0.9,
      coverage: ['torso'],
      condition: 1.0,
      weight: 8.0
    });

    this.armor.set('helmet_1', {
      id: 'helmet_1',
      name: 'Advanced Combat Helmet',
      type: 'helmet',
      protection_level: 0.7,
      coverage: ['head'],
      condition: 1.0,
      weight: 2.5
    });
  }

  private _initializeMeleeWeapons(): void {
    this.meleeWeapons.set('sword_1', {
      id: 'sword_1',
      name: 'Excalibur Replica',
      type: 'sword',
      era: 'medieval',
      damage: 85,
      reach: 1.2,
      material: 'high_carbon_steel',
      condition: 1.0
    });

    this.meleeWeapons.set('axe_1', {
      id: 'axe_1',
      name: 'Viking Battle Axe',
      type: 'axe',
      era: 'medieval',
      damage: 75,
      reach: 0.8,
      material: 'iron_wood',
      condition: 1.0
    });
  }

  private _initializeMilitaryVehicles(): void {
    this.militaryVehicles.set('apache_1', {
      id: 'apache_1',
      name: 'AH-64 Apache',
      type: 'helicopter',
      armament: ['30mm_chain_gun', 'hellfire_missiles', 'hydra_rockets'],
      crew_capacity: 2,
      max_speed: 284,
      operational_range: 480,
      fuel_type: 'jet_fuel',
      condition: 1.0
    });

    this.militaryVehicles.set('f35_1', {
      id: 'f35_1',
      name: 'F-35 Lightning II',
      type: 'jet',
      armament: ['25mm_cannon', 'aim_120_missiles', 'jdam_bombs'],
      crew_capacity: 1,
      max_speed: 1975,
      operational_range: 2220,
      fuel_type: 'jet_fuel',
      condition: 1.0
    });
  }

  private _initializeDefenseSystems(): void {
    this.defenseSystems.set('turret_1', {
      id: 'turret_1',
      name: 'Automated Defense Turret',
      type: 'turret',
      effectiveness: 0.85,
      power_requirement: 5000,
      active: false
    });
  }

  private _updateWeaponConditions(dtSeconds: number): void {
    for (const firearm of this.firearms.values()) {
      // Natural decay
      firearm.condition = Math.max(0, firearm.condition - dtSeconds * 0.000001);
    }
  }

  private _updateAmmunitionStorage(dtSeconds: number): void {
    // Ammunition degradation minimal
  }

  private _updateArmorIntegrity(dtSeconds: number): void {
    for (const armor of this.armor.values()) {
      armor.condition = Math.max(0, armor.condition - dtSeconds * 0.0000005);
    }
  }

  private _updateVehicleMaintenance(dtSeconds: number): void {
    for (const vehicle of this.militaryVehicles.values()) {
      vehicle.condition = Math.max(0, vehicle.condition - dtSeconds * 0.000002);
    }
  }

  private _updateDefenseSystems(dtSeconds: number): void {
    for (const defense of this.defenseSystems.values()) {
      if (defense.active) {
        defense.effectiveness = Math.max(0, defense.effectiveness - dtSeconds * 0.00001);
      }
    }
  }

  getFirearmsStatus(): any {
    return {
      total_firearms: this.firearms.size,
      by_type: this._groupByType(this.firearms),
      average_condition: this._averageCondition(Array.from(this.firearms.values()))
    };
  }

  getAmmunitionStatus(): any {
    const totalRounds = Array.from(this.ammunition.values())
      .reduce((sum, ammo) => sum + ammo.quantity, 0);
    
    return {
      total_rounds: totalRounds,
      by_caliber: this._groupByCaliber(this.ammunition),
      types_available: new Set(Array.from(this.ammunition.values()).map(a => a.type)).size
    };
  }

  getArmorStatus(): any {
    return {
      total_items: this.armor.size,
      average_protection: this._averageProtection(Array.from(this.armor.values())),
      total_weight: Array.from(this.armor.values()).reduce((sum, a) => sum + a.weight, 0)
    };
  }

  getMeleeWeaponsStatus(): any {
    return {
      total_weapons: this.meleeWeapons.size,
      by_era: this._groupByEra(this.meleeWeapons),
      average_damage: this._averageDamage(Array.from(this.meleeWeapons.values()))
    };
  }

  getVehiclesStatus(): any {
    return {
      total_vehicles: this.militaryVehicles.size,
      by_type: this._groupByVehicleType(this.militaryVehicles),
      operational_count: Array.from(this.militaryVehicles.values()).filter(v => v.condition > 0.5).length
    };
  }

  getDefenseSystemsStatus(): any {
    return {
      total_systems: this.defenseSystems.size,
      active_systems: Array.from(this.defenseSystems.values()).filter(d => d.active).length,
      average_effectiveness: this._averageEffectiveness(Array.from(this.defenseSystems.values()))
    };
  }

  private _groupByType(items: Map<string, any>): any {
    const groups = {};
    for (const item of items.values()) {
      groups[item.type] = (groups[item.type] || 0) + 1;
    }
    return groups;
  }

  private _groupByCaliber(items: Map<string, Ammunition>): any {
    const groups = {};
    for (const ammo of items.values()) {
      groups[ammo.type] = (groups[ammo.type] || 0) + ammo.quantity;
    }
    return groups;
  }

  private _groupByEra(items: Map<string, MeleeWeapon>): any {
    const groups = {};
    for (const weapon of items.values()) {
      groups[weapon.era] = (groups[weapon.era] || 0) + 1;
    }
    return groups;
  }

  private _groupByVehicleType(items: Map<string, MilitaryVehicle>): any {
    const groups = {};
    for (const vehicle of items.values()) {
      groups[vehicle.type] = (groups[vehicle.type] || 0) + 1;
    }
    return groups;
  }

  private _averageCondition(items: any[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.condition, 0) / items.length;
  }

  private _averageProtection(items: Armor[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.protection_level, 0) / items.length;
  }

  private _averageDamage(items: MeleeWeapon[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.damage, 0) / items.length;
  }

  private _averageEffectiveness(items: DefenseSystem[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.effectiveness, 0) / items.length;
  }
}

export class FiringRange {
  private lanes: Map<string, FiringRangeLane>;
  private targets: Map<string, Target>;
  private safetySystems: boolean;

  constructor() {
    this.lanes = new Map();
    this.targets = new Map();
    this.safetySystems = true;

    this._initializeFiringRange();
  }

  update(dtSeconds: number): void {
    // Update target conditions
    for (const target of this.targets.values()) {
      target.update(dtSeconds);
    }

    // Update lane conditions
    for (const lane of this.lanes.values()) {
      lane.update(dtSeconds);
    }
  }

  getRangeStatus(): any {
    return {
      total_lanes: this.lanes.size,
      active_lanes: Array.from(this.lanes.values()).filter(l => l.active).length,
      targets_available: this.targets.size,
      safety_systems_active: this.safetySystems
    };
  }

  private _initializeFiringRange(): void {
    // Initialize firing range lanes
    for (let i = 1; i <= 10; i++) {
      this.lanes.set(`lane_${i}`, {
        id: `lane_${i}`,
        number: i,
        distance: 100, // 100m lanes
        active: false,
        condition: 1.0,
        last_maintenance: Date.now()
      });
    }

    // Initialize targets
    for (let i = 1; i <= 20; i++) {
      this.targets.set(`target_${i}`, {
        id: `target_${i}`,
        type: i % 2 === 0 ? 'human_silhouette' : 'steel_plate',
        distance: 50 + (i * 10),
        condition: 1.0,
        hit_count: 0
      });
    }
  }
}

interface FiringRangeLane {
  id: string;
  number: number;
  distance: number;
  active: boolean;
  condition: number;
  last_maintenance: number;

  update(dtSeconds: number): void;
}

interface Target {
  id: string;
  type: 'human_silhouette' | 'steel_plate' | 'clay_pigeon';
  distance: number;
  condition: number;
  hit_count: number;

  update(dtSeconds: number): void;
}
