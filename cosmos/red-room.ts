/**
 * RED ROOM
 * 
 * Adult entertainment and pleasure systems
 * Toys, outfits, and pleasure objects
 */

import { ChaosSys } from '../chaos/chaos-sys';

export interface Toy {
  id: string;
  name: string;
  type: 'vibrator' | 'dildo' | 'bondage' | 'roleplay' | 'sensory' | 'luxury';
  material: string;
  condition: number; // 0-1
  cleanliness: number; // 0-1
  power_requirement?: number; // watts
  features: string[];
}

export interface Outfit {
  id: string;
  name: string;
  style: 'lingerie' | 'latex' | 'leather' | 'roleplay' | 'casual' | 'formal';
  size: 'xs' | 's' | 'm' | 'l' | 'xl' | 'xxl';
  material: string;
  condition: number;
  cleanliness: number;
  accessories: string[];
}

export interface PleasureObject {
  id: string;
  name: string;
  category: 'furniture' | 'restraint' | 'sensory' | 'atmosphere' | 'luxury';
  type: string;
  material: string;
  condition: number;
  cleanliness: number;
  capacity?: number; // for furniture
}

export interface RedRoomState {
  ambiance_level: number; // 0-1, mood lighting/atmosphere
  temperature: number; // Celsius
  privacy_level: number; // 0-1, soundproofing/security
  cleanliness_level: number; // 0-1, overall hygiene
  entertainment_active: boolean;
}

export class RedRoom {
  private toys: Map<string, Toy>;
  private outfits: Map<string, Outfit>;
  private pleasureObjects: Map<string, PleasureObject>;
  private state: RedRoomState;
  private lastCleaning: number;

  constructor() {
    this.toys = new Map();
    this.outfits = new Map();
    this.pleasureObjects = new Map();
    this.lastCleaning = Date.now();

    this.state = {
      ambiance_level: 0.8,
      temperature: 22.0,
      privacy_level: 0.9,
      cleanliness_level: 0.9,
      entertainment_active: false
    };

    this._initializeRedRoom();
  }

  update(dtSeconds: number): void {
    // Update toy conditions
    this._updateToyConditions(dtSeconds);

    // Update outfit conditions
    this._updateOutfitConditions(dtSeconds);

    // Update pleasure objects
    this._updatePleasureObjects(dtSeconds);

    // Update room state
    this._updateRoomState(dtSeconds);

    // Process cleanliness decay
    this._processCleanlinessDecay(dtSeconds);
  }

  addToy(toy: Omit<Toy, 'id'>): string {
    const id = `toy_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullToy: Toy = {
      id,
      ...toy,
      condition: 1.0,
      cleanliness: 1.0
    };

    this.toys.set(id, fullToy);
    return id;
  }

  addOutfit(outfit: Omit<Outfit, 'id'>): string {
    const id = `outfit_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullOutfit: Outfit = {
      id,
      ...outfit,
      condition: 1.0,
      cleanliness: 1.0
    };

    this.outfits.set(id, fullOutfit);
    return id;
  }

  addPleasureObject(object: Omit<PleasureObject, 'id'>): string {
    const id = `object_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullObject: PleasureObject = {
      id,
      ...object,
      condition: 1.0,
      cleanliness: 1.0
    };

    this.pleasureObjects.set(id, fullObject);
    return id;
  }

  useToy(toyId: string, intensity: number): boolean {
    const toy = this.toys.get(toyId);
    if (!toy) return false;
    if (toy.condition < 0.1) return false;

    // Apply wear and cleanliness reduction
    toy.condition = Math.max(0, toy.condition - intensity * 0.001);
    toy.cleanliness = Math.max(0, toy.cleanliness - intensity * 0.01);

    // Update room state
    this.state.entertainment_active = true;

    return true;
  }

  wearOutfit(outfitId: string, duration: number): boolean {
    const outfit = this.outfits.get(outfitId);
    if (!outfit) return false;
    if (outfit.condition < 0.1) return false;

    // Apply wear and cleanliness reduction
    outfit.condition = Math.max(0, outfit.condition - duration * 0.00001);
    outfit.cleanliness = Math.max(0, outfit.cleanliness - duration * 0.0001);

    return true;
  }

  usePleasureObject(objectId: string, intensity: number): boolean {
    const object = this.pleasureObjects.get(objectId);
    if (!object) return false;
    if (object.condition < 0.1) return false;

    // Apply wear and cleanliness reduction
    object.condition = Math.max(0, object.condition - intensity * 0.0005);
    object.cleanliness = Math.max(0, object.cleanliness - intensity * 0.005);

    // Update room state
    this.state.entertainment_active = true;

    return true;
  }

  cleanRoom(): void {
    this.lastCleaning = Date.now();
    
    // Clean all items
    for (const toy of this.toys.values()) {
      toy.cleanliness = Math.min(1, toy.cleanliness + 0.5);
    }
    
    for (const outfit of this.outfits.values()) {
      outfit.cleanliness = Math.min(1, outfit.cleanliness + 0.3);
    }
    
    for (const object of this.pleasureObjects.values()) {
      object.cleanliness = Math.min(1, object.cleanliness + 0.4);
    }

    // Update room cleanliness
    this.state.cleanliness_level = Math.min(1, this.state.cleanliness_level + 0.3);
  }

  setAmbiance(level: number): void {
    this.state.ambiance_level = Math.max(0, Math.min(1, level));
  }

  setTemperature(temperature: number): void {
    this.state.temperature = Math.max(15, Math.min(30, temperature));
  }

  getRedRoomReport(): {
    state: RedRoomState;
    toys: any;
    outfits: any;
    pleasure_objects: any;
  } {
    return {
      state: { ...this.state },
      toys: this.getToysStatus(),
      outfits: this.getOutfitsStatus(),
      pleasure_objects: this.getPleasureObjectsStatus()
    };
  }

  private _initializeRedRoom(): void {
    // Initialize toys - all types
    this._initializeToys();
    
    // Initialize outfits - different styles
    this._initializeOutfits();
    
    // Initialize pleasure objects
    this._initializePleasureObjects();
  }

  private _initializeToys(): void {
    // Vibrators
    this.addToy({
      name: 'Magic Wand',
      type: 'vibrator',
      material: 'silicone',
      condition: 1.0,
      cleanliness: 1.0,
      power_requirement: 20,
      features: ['multiple_speeds', 'waterproof', 'rechargeable']
    });

    this.addToy({
      name: 'Rabbit Vibrator',
      type: 'vibrator',
      material: 'silicone',
      condition: 1.0,
      cleanliness: 1.0,
      power_requirement: 15,
      features: ['dual_stimulation', 'multiple_patterns', 'rechargeable']
    });

    // Dildos
    this.addToy({
      name: 'Glass Dildo',
      type: 'dildo',
      material: 'borosilicate_glass',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['temperature_play', 'hypoallergenic', 'easy_clean']
    });

    this.addToy({
      name: 'Realistic Dildo',
      type: 'dildo',
      material: 'dual_density_silicone',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['suction_cup', 'realistic_texture', 'body_safe']
    });

    // Bondage equipment
    this.addToy({
      name: 'Silk Rope Set',
      type: 'bondage',
      material: 'silk',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['soft', 'strong', 'washable', 'multiple_lengths']
    });

    this.addToy({
      name: 'Leather Cuffs',
      type: 'bondage',
      material: 'genuine_leather',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['adjustable', 'lined', 'quick_release']
    });

    // Roleplay items
    this.addToy({
      name: 'Blindfold Set',
      type: 'roleplay',
      material: 'satin',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['adjustable', 'comfortable', 'light_blocking']
    });

    // Sensory toys
    this.addToy({
      name: 'Feather Tickler',
      type: 'sensory',
      material: 'ostrich_feathers',
      condition: 1.0,
      cleanliness: 1.0,
      features: ['soft', 'lightweight', 'sensory_enhancement']
    });
  }

  private _initializeOutfits(): void {
    // Lingerie
    this.addOutfit({
      name: 'Black Lace Bodysuit',
      style: 'lingerie',
      size: 'm',
      material: 'lace',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['matching_thong', 'garter_belt']
    });

    this.addOutfit({
      name: 'Red Corset',
      style: 'lingerie',
      size: 'm',
      material: 'satin_boning',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['stockings', 'thong']
    });

    // Latex
    this.addOutfit({
      name: 'Black Catsuit',
      style: 'latex',
      size: 'm',
      material: 'latex',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['gloves', 'zipper_front']
    });

    // Leather
    this.addOutfit({
      name: 'Leather Harness',
      style: 'leather',
      size: 'm',
      material: 'genuine_leather',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['adjustable_straps', 'metal_hardware']
    });

    // Roleplay
    this.addOutfit({
      name: 'School Girl Outfit',
      style: 'roleplay',
      size: 'm',
      material: 'cotton_blend',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['pleated_skirt', 'tie', 'thigh_highs']
    });

    this.addOutfit({
      name: 'Police Officer',
      style: 'roleplay',
      size: 'm',
      material: 'polyester',
      condition: 1.0,
      cleanliness: 1.0,
      accessories: ['badge', 'handcuffs', 'baton']
    });
  }

  private _initializePleasureObjects(): void {
    // Furniture
    this.addPleasureObject({
      name: 'Bondage Bed',
      category: 'furniture',
      type: 'restraint_bed',
      material: 'wood_metal',
      condition: 1.0,
      cleanliness: 1.0,
      capacity: 2
    });

    this.addPleasureObject({
      name: 'Sex Swing',
      category: 'furniture',
      type: 'door_swing',
      material: 'nylon_steel',
      condition: 1.0,
      cleanliness: 1.0,
      capacity: 1
    });

    // Restraints
    this.addPleasureObject({
      name: 'Under-Bed Restraint System',
      category: 'restraint',
      type: 'bed_restraints',
      material: 'leather_neoprene',
      condition: 1.0,
      cleanliness: 1.0
    });

    // Sensory equipment
    this.addPleasureObject({
      name: 'Sensory Deprivation Hood',
      category: 'sensory',
      type: 'full_hood',
      material: 'leather',
      condition: 1.0,
      cleanliness: 1.0
    });

    // Atmosphere
    this.addPleasureObject({
      name: 'LED Mood Lighting',
      category: 'atmosphere',
      type: 'color_changing_lights',
      material: 'plastic_led',
      condition: 1.0,
      cleanliness: 1.0
    });

    this.addPleasureObject({
      name: 'Aromatherapy Diffuser',
      category: 'atmosphere',
      type: 'essential_oil_diffuser',
      material: 'glass_plastic',
      condition: 1.0,
      cleanliness: 1.0
    });
  }

  private _updateToyConditions(dtSeconds: number): void {
    for (const toy of this.toys.values()) {
      // Natural decay
      toy.condition = Math.max(0, toy.condition - dtSeconds * 0.000001);
    }
  }

  private _updateOutfitConditions(dtSeconds: number): void {
    for (const outfit of this.outfits.values()) {
      // Natural decay
      outfit.condition = Math.max(0, outfit.condition - dtSeconds * 0.0000005);
    }
  }

  private _updatePleasureObjects(dtSeconds: number): void {
    for (const object of this.pleasureObjects.values()) {
      // Natural decay
      object.condition = Math.max(0, object.condition - dtSeconds * 0.0000002);
    }
  }

  private _updateRoomState(dtSeconds: number): void {
    // Natural ambiance decay
    this.state.ambiance_level = Math.max(0, this.state.ambiance_level - dtSeconds * 0.00001);

    // Temperature regulation
    const targetTemp = 22.0;
    const tempDiff = targetTemp - this.state.temperature;
    this.state.temperature += tempDiff * dtSeconds * 0.001;
  }

  private _processCleanlinessDecay(dtSeconds: number): void {
    // Cleanliness naturally decreases
    this.state.cleanliness_level = Math.max(0, this.state.cleanliness_level - dtSeconds * 0.000005);

    // Item cleanliness also decays
    for (const toy of this.toys.values()) {
      toy.cleanliness = Math.max(0, toy.cleanliness - dtSeconds * 0.000002);
    }

    for (const outfit of this.outfits.values()) {
      outfit.cleanliness = Math.max(0, outfit.cleanliness - dtSeconds * 0.000001);
    }
  }

  getToysStatus(): any {
    return {
      total_toys: this.toys.size,
      by_type: this._groupByToyType(this.toys),
      average_condition: this._averageCondition(Array.from(this.toys.values())),
      average_cleanliness: this._averageCleanliness(Array.from(this.toys.values()))
    };
  }

  getOutfitsStatus(): any {
    return {
      total_outfits: this.outfits.size,
      by_style: this._groupByOutfitStyle(this.outfits),
      average_condition: this._averageCondition(Array.from(this.outfits.values())),
      average_cleanliness: this._averageCleanliness(Array.from(this.outfits.values()))
    };
  }

  getPleasureObjectsStatus(): any {
    return {
      total_objects: this.pleasureObjects.size,
      by_category: this._groupByObjectCategory(this.pleasureObjects),
      average_condition: this._averageCondition(Array.from(this.pleasureObjects.values())),
      average_cleanliness: this._averageCleanliness(Array.from(this.pleasureObjects.values()))
    };
  }

  private _groupByToyType(items: Map<string, Toy>): any {
    const groups = {};
    for (const toy of items.values()) {
      groups[toy.type] = (groups[toy.type] || 0) + 1;
    }
    return groups;
  }

  private _groupByOutfitStyle(items: Map<string, Outfit>): any {
    const groups = {};
    for (const outfit of items.values()) {
      groups[outfit.style] = (groups[outfit.style] || 0) + 1;
    }
    return groups;
  }

  private _groupByObjectCategory(items: Map<string, PleasureObject>): any {
    const groups = {};
    for (const object of items.values()) {
      groups[object.category] = (groups[object.category] || 0) + 1;
    }
    return groups;
  }

  private _averageCondition(items: any[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.condition, 0) / items.length;
  }

  private _averageCleanliness(items: any[]): number {
    if (items.length === 0) return 0;
    return items.reduce((sum, item) => sum + item.cleanliness, 0) / items.length;
  }
}

export default RedRoom;
