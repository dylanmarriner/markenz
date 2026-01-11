import { WorldLocation, WorldObject, WorldVehicle, WorldResource } from '../types/world.types';

/**
 * Canonical Island World Definition v3
 * Authoritative world structure that is created once and persisted forever
 */

export const CANONICAL_WORLD_VERSION = '3.0.0';

// ============================================================================
// ISLAND LOCATIONS
// ============================================================================

export const ISLAND_LOCATIONS: WorldLocation[] = [
  // Primary Structure: HOUSE
  {
    location_id: 'house',
    name: 'Main House',
    type: 'structure',
    coordinates: { x: 500, y: 400, z: 0 },
    dimensions: { width: 30, height: 20, depth: 10 },
    sleep_allowed: false,
    max_occupancy: 20,
    allowed_activities: ['living', 'socializing', 'cooking', 'working'],
    interaction_permissions: { residents: 'full', guests: 'limited' },
    utilities: { power: true, water: true, drainage: true, internet: true },
    base_inventory: [],
    affordances: { shelter: true, comfort: true, privacy: true },
  },

  // Twin Room A - Gem-D's room
  {
    location_id: 'twin_room_a',
    name: "Gem-D's Bedroom",
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 510, y: 410, z: 0 },
    dimensions: { width: 8, height: 6, depth: 4 },
    sleep_allowed: true,
    max_occupancy: 4, // Can sleep both twins in king bed
    allowed_activities: ['sleeping', 'relaxing', 'working', 'dressing'],
    interaction_permissions: { 'gem-d': 'full', 'gem-k': 'invited', others: 'restricted' },
    utilities: { power: true, lighting: true, ventilation: true },
    base_inventory: ['clothes', 'books', 'personal_items'],
    affordances: { privacy: true, comfort: 0.9, storage: true },
  },

  // Twin Room B - Gem-K's room
  {
    location_id: 'twin_room_b',
    name: "Gem-K's Bedroom",
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 520, y: 410, z: 0 },
    dimensions: { width: 8, height: 6, depth: 4 },
    sleep_allowed: true,
    max_occupancy: 4, // Can sleep both twins in king bed
    allowed_activities: ['sleeping', 'relaxing', 'working', 'dressing'],
    interaction_permissions: { 'gem-k': 'full', 'gem-d': 'invited', others: 'restricted' },
    utilities: { power: true, lighting: true, ventilation: true },
    base_inventory: ['clothes', 'books', 'personal_items'],
    affordances: { privacy: true, comfort: 0.9, storage: true },
  },

  // Bathroom
  {
    location_id: 'bathroom',
    name: 'Bathroom',
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 515, y: 420, z: 0 },
    dimensions: { width: 5, height: 4, depth: 3 },
    sleep_allowed: false,
    max_occupancy: 3,
    allowed_activities: ['hygiene', 'grooming'],
    interaction_permissions: { residents: 'full', guests: 'limited' },
    utilities: { power: true, water: true, drainage: true, ventilation: true },
    base_inventory: ['towels', 'soap', 'toiletries'],
    affordances: { hygiene: true, privacy: true },
  },

  // Kitchen
  {
    location_id: 'kitchen',
    name: 'Kitchen',
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 505, y: 405, z: 0 },
    dimensions: { width: 6, height: 5, depth: 4 },
    sleep_allowed: false,
    max_occupancy: 5,
    allowed_activities: ['cooking', 'eating', 'food_storage'],
    interaction_permissions: { residents: 'full', guests: 'supervised' },
    utilities: { power: true, water: true, drainage: true, gas: true },
    base_inventory: ['appliances', 'utensils', 'cookware', 'food'],
    affordances: { cooking: true, food_storage: true, cleaning: true },
  },

  // Lounge
  {
    location_id: 'lounge',
    name: 'Lounge',
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 515, y: 395, z: 0 },
    dimensions: { width: 10, height: 8, depth: 6 },
    sleep_allowed: true, // Non-primary sleeping allowed
    max_occupancy: 10,
    allowed_activities: ['relaxing', 'entertainment', 'socializing', 'sleeping'],
    interaction_permissions: { residents: 'full', guests: 'full' },
    utilities: { power: true, lighting: true, entertainment: true },
    base_inventory: ['furniture', 'electronics', 'decorations'],
    affordances: { comfort: 0.7, entertainment: true, social: true },
  },

  // Computer Room
  {
    location_id: 'computer_room',
    name: 'Computer Room',
    type: 'room',
    parent_location: 'house',
    coordinates: { x: 525, y: 400, z: 0 },
    dimensions: { width: 7, height: 5, depth: 4 },
    sleep_allowed: false,
    max_occupancy: 4,
    allowed_activities: ['working', 'computing', 'research', 'communication'],
    interaction_permissions: { residents: 'full', guests: 'invited' },
    utilities: { power: true, internet: true, networking: true, cooling: true },
    base_inventory: ['computers', 'peripherals', 'equipment'],
    affordances: { computing: true, productivity: true, communication: true },
  },

  // External Structure: Tool Shed
  {
    location_id: 'tool_shed',
    name: 'Tool Shed',
    type: 'structure',
    coordinates: { x: 450, y: 350, z: 0 },
    dimensions: { width: 15, height: 10, depth: 8 },
    sleep_allowed: false,
    max_occupancy: 5,
    allowed_activities: ['crafting', 'repair', 'construction', 'mining'],
    interaction_permissions: { residents: 'full', guests: 'supervised' },
    utilities: { power: true, work_bench: true, storage: true },
    base_inventory: ['tools', 'materials', 'equipment'],
    affordances: { crafting: true, repair: true, construction: true },
  },

  // Garage/Workshop - Large shed with roller doors
  {
    location_id: 'garage',
    name: 'Garage & Workshop',
    type: 'structure',
    coordinates: { x: 480, y: 400, z: 0 },
    dimensions: { width: 20, height: 8, depth: 15 },
    sleep_allowed: false,
    max_occupancy: 10,
    allowed_activities: ['vehicle_maintenance', 'repair', 'construction', 'storage'],
    interaction_permissions: { residents: 'full', guests: 'supervised' },
    utilities: { power: true, work_bench: true, storage: true, compressed_air: true },
    base_inventory: ['tools', 'equipment', 'vehicle_parts'],
    affordances: { repair: true, construction: true, vehicle_storage: true },
  },

  // Island exterior areas
  {
    location_id: 'island_north',
    name: 'Northern Beach',
    type: 'outdoor',
    coordinates: { x: 500, y: 600, z: 0 },
    dimensions: { width: 200, height: 100, depth: 50 },
    sleep_allowed: true, // Outdoor sleeping allowed
    max_occupancy: 20,
    allowed_activities: ['swimming', 'relaxing', 'exploring', 'fishing'],
    interaction_permissions: { all: 'full' },
    utilities: { natural: true },
    base_inventory: [],
    affordances: { nature: true, recreation: true },
  },

  {
    location_id: 'island_east',
    name: 'Mining Hills',
    type: 'outdoor',
    coordinates: { x: 700, y: 400, z: 0 },
    dimensions: { width: 150, height: 100, depth: 100 },
    sleep_allowed: false,
    max_occupancy: 15,
    allowed_activities: ['mining', 'exploration', 'construction'],
    interaction_permissions: { residents: 'full', guests: 'supervised' },
    utilities: { natural_resources: true },
    base_inventory: [],
    affordances: { mining: true, resources: true },
  },

  {
    location_id: 'island_south',
    name: 'Southern Forest',
    type: 'outdoor',
    coordinates: { x: 500, y: 200, z: 0 },
    dimensions: { width: 180, height: 120, depth: 80 },
    sleep_allowed: true, // Camping allowed
    max_occupancy: 25,
    allowed_activities: ['exploring', 'gathering', 'camping', 'hunting'],
    interaction_permissions: { all: 'full' },
    utilities: { natural: true },
    base_inventory: [],
    affordances: { nature: true, resources: true, wildlife: true },
  },
];

// ============================================================================
// WORLD OBJECTS
// ============================================================================

export const WORLD_OBJECTS: WorldObject[] = [
  // Twin Room A Objects
  {
    object_id: 'bed_king_a',
    location_id: 'twin_room_a',
    name: 'King Size Bed A',
    type: 'bed',
    subtype: 'king',
    coordinates: { x: 513, y: 412, z: 0 },
    dimensions: { width: 2, height: 2, depth: 2.2 },
    weight: 100,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 2, // King bed can sleep 2
    current_state: { made: true, clean: true },
    inventory: ['pillows', 'blankets', 'sheets'],
    properties: { comfort: 0.95, size: 'king', material: 'memory_foam' },
    required_permissions: ['sleep_permission'],
  },

  {
    object_id: 'desk_a',
    location_id: 'twin_room_a',
    name: "Gem-D's Desk",
    type: 'desk',
    coordinates: { x: 517, y: 414, z: 0 },
    dimensions: { width: 1.2, height: 0.8, depth: 0.6 },
    weight: 30,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { organized: true },
    inventory: ['laptop', 'notebooks', 'pens'],
    properties: { style: 'modern', material: 'wood' },
    owner_agent: 'gem-d',
  },

  {
    object_id: 'tv_a',
    location_id: 'twin_room_a',
    name: "Gem-D's TV",
    type: 'tv',
    coordinates: { x: 514, y: 409, z: 1 },
    dimensions: { width: 1, height: 0.1, depth: 0.05 },
    weight: 15,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 0,
    current_state: { powered: false, channel: 1 },
    inventory: [],
    properties: { size: '55inch', type: 'smart', resolution: '4K' },
    owner_agent: 'gem-d',
  },

  {
    object_id: 'storage_a',
    location_id: 'twin_room_a',
    name: "Gem-D's Storage",
    type: 'storage',
    coordinates: { x: 511, y: 415, z: 0 },
    dimensions: { width: 2, height: 0.6, depth: 0.4 },
    weight: 40,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 50,
    current_state: { locked: false },
    inventory: ['clothes', 'shoes', 'accessories'],
    properties: { type: 'wardrobe', material: 'wood' },
    owner_agent: 'gem-d',
  },

  // Twin Room B Objects
  {
    object_id: 'bed_king_b',
    location_id: 'twin_room_b',
    name: 'King Size Bed B',
    type: 'bed',
    subtype: 'king',
    coordinates: { x: 523, y: 412, z: 0 },
    dimensions: { width: 2, height: 2, depth: 2.2 },
    weight: 100,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 2, // King bed can sleep 2
    current_state: { made: true, clean: true },
    inventory: ['pillows', 'blankets', 'sheets'],
    properties: { comfort: 0.95, size: 'king', material: 'memory_foam' },
    required_permissions: ['sleep_permission'],
  },

  {
    object_id: 'desk_b',
    location_id: 'twin_room_b',
    name: "Gem-K's Desk",
    type: 'desk',
    coordinates: { x: 527, y: 414, z: 0 },
    dimensions: { width: 1.2, height: 0.8, depth: 0.6 },
    weight: 30,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { organized: true },
    inventory: ['laptop', 'notebooks', 'art_supplies'],
    properties: { style: 'modern', material: 'wood' },
    owner_agent: 'gem-k',
  },

  {
    object_id: 'tv_b',
    location_id: 'twin_room_b',
    name: "Gem-K's TV",
    type: 'tv',
    coordinates: { x: 524, y: 409, z: 1 },
    dimensions: { width: 1, height: 0.1, depth: 0.05 },
    weight: 15,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 0,
    current_state: { powered: false, channel: 1 },
    inventory: [],
    properties: { size: '55inch', type: 'smart', resolution: '4K' },
    owner_agent: 'gem-k',
  },

  {
    object_id: 'storage_b',
    location_id: 'twin_room_b',
    name: "Gem-K's Storage",
    type: 'storage',
    coordinates: { x: 521, y: 415, z: 0 },
    dimensions: { width: 2, height: 0.6, depth: 0.4 },
    weight: 40,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 50,
    current_state: { locked: false },
    inventory: ['clothes', 'shoes', 'accessories'],
    properties: { type: 'wardrobe', material: 'wood' },
    owner_agent: 'gem-k',
  },

  // Bathroom Objects
  {
    object_id: 'bathtub',
    location_id: 'bathroom',
    name: 'Bathtub',
    type: 'bathtub',
    coordinates: { x: 516, y: 421, z: 0 },
    dimensions: { width: 1.8, height: 0.8, depth: 0.6 },
    weight: 80,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { clean: true, drained: true },
    inventory: [],
    properties: { type: 'jacuzzi', jets: true },
  },

  {
    object_id: 'shower',
    location_id: 'bathroom',
    name: 'Shower',
    type: 'shower',
    coordinates: { x: 518, y: 422, z: 0 },
    dimensions: { width: 1, height: 1, depth: 1 },
    weight: 50,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { clean: true },
    inventory: [],
    properties: { type: 'glass_door', temperature_control: true },
  },

  {
    object_id: 'toilet',
    location_id: 'bathroom',
    name: 'Toilet',
    type: 'toilet',
    coordinates: { x: 514, y: 422, z: 0 },
    dimensions: { width: 0.4, height: 0.6, depth: 0.7 },
    weight: 30,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { clean: true },
    inventory: [],
    properties: { type: 'modern', flush_type: 'dual' },
  },

  {
    object_id: 'sink',
    location_id: 'bathroom',
    name: 'Sink',
    type: 'sink',
    coordinates: { x: 517, y: 419, z: 0 },
    dimensions: { width: 0.6, height: 0.4, depth: 0.4 },
    weight: 20,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 0,
    current_state: { clean: true, faucet_open: false },
    inventory: ['soap', 'towels'],
    properties: { type: 'vanity', mirror: true },
  },

  // Kitchen Objects
  {
    object_id: 'stove',
    location_id: 'kitchen',
    name: 'Stove',
    type: 'stove',
    coordinates: { x: 506, y: 406, z: 0.9 },
    dimensions: { width: 0.8, height: 0.6, depth: 0.6 },
    weight: 60,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 4,
    current_state: { powered: false, clean: true },
    inventory: [],
    properties: { type: 'gas', burners: 4, oven: true },
  },

  {
    object_id: 'fridge',
    location_id: 'kitchen',
    name: 'Refrigerator',
    type: 'fridge',
    coordinates: { x: 508, y: 407, z: 0 },
    dimensions: { width: 0.8, height: 1.8, depth: 0.7 },
    weight: 100,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 100,
    current_state: { powered: true, temperature: 4 },
    inventory: ['food', 'drinks', 'ingredients'],
    properties: { type: 'frost_free', ice_maker: true },
  },

  {
    object_id: 'kitchen_table',
    location_id: 'kitchen',
    name: 'Kitchen Table',
    type: 'table',
    coordinates: { x: 507, y: 408, z: 0.75 },
    dimensions: { width: 1.5, height: 0.75, depth: 0.9 },
    weight: 40,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 6,
    current_state: { clean: true },
    inventory: ['chairs', 'place_settings'],
    properties: { material: 'wood', style: 'farmhouse' },
  },

  // Lounge Objects
  {
    object_id: 'lounge_tv',
    location_id: 'lounge',
    name: 'Large TV',
    type: 'tv',
    coordinates: { x: 515, y: 396, z: 1.5 },
    dimensions: { width: 2, height: 0.1, depth: 0.05 },
    weight: 40,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 0,
    current_state: { powered: false, channel: 1 },
    inventory: [],
    properties: { size: '75inch', type: 'smart', resolution: '8K', soundbar: true },
  },

  {
    object_id: 'couch_large',
    location_id: 'lounge',
    name: 'Large Couch',
    type: 'couch',
    coordinates: { x: 518, y: 400, z: 0 },
    dimensions: { width: 3, height: 0.8, depth: 1 },
    weight: 60,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 4,
    current_state: { clean: true },
    inventory: ['cushions', 'blankets'],
    properties: { material: 'leather', reclining: true, cup_holders: true },
  },

  // Computer Room Objects
  {
    object_id: 'computer_d',
    location_id: 'computer_room',
    name: "Gem-D's Computer",
    type: 'computer',
    coordinates: { x: 526, y: 401, z: 0.75 },
    dimensions: { width: 0.8, height: 0.6, depth: 0.4 },
    weight: 15,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { powered: false, logged_in: false },
    inventory: [],
    properties: { type: 'gaming', specs: 'high_end', monitors: 3 },
    owner_agent: 'gem-d',
  },

  {
    object_id: 'computer_k',
    location_id: 'computer_room',
    name: "Gem-K's Computer",
    type: 'computer',
    coordinates: { x: 528, y: 401, z: 0.75 },
    dimensions: { width: 0.8, height: 0.6, depth: 0.4 },
    weight: 15,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 1,
    current_state: { powered: false, logged_in: false },
    inventory: [],
    properties: { type: 'workstation', specs: 'ultra_high_end', monitors: 2 },
    owner_agent: 'gem-k',
  },

  {
    object_id: 'chair_d',
    location_id: 'computer_room',
    name: "Gem-D's Chair",
    type: 'chair',
    coordinates: { x: 526, y: 402, z: 0 },
    dimensions: { width: 0.6, height: 1.2, depth: 0.6 },
    weight: 20,
    is_interactable: true,
    is_usable: true,
    is_movable: true,
    capacity: 1,
    current_state: { height_adjusted: true },
    inventory: [],
    properties: { type: 'gaming', ergonomic: true, massage: true },
    owner_agent: 'gem-d',
  },

  {
    object_id: 'chair_k',
    location_id: 'computer_room',
    name: "Gem-K's Chair",
    type: 'chair',
    coordinates: { x: 528, y: 402, z: 0 },
    dimensions: { width: 0.6, height: 1.2, depth: 0.6 },
    weight: 20,
    is_interactable: true,
    is_usable: true,
    is_movable: true,
    capacity: 1,
    current_state: { height_adjusted: true },
    inventory: [],
    properties: { type: 'executive', ergonomic: true, lumbar_support: true },
    owner_agent: 'gem-k',
  },

  // Tool Shed Objects
  {
    object_id: 'workbench',
    location_id: 'tool_shed',
    name: 'Workbench',
    type: 'workbench',
    coordinates: { x: 455, y: 355, z: 0 },
    dimensions: { width: 2, height: 0.9, depth: 0.8 },
    weight: 80,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 10,
    current_state: { clean: true, organized: true },
    inventory: ['tools', 'materials', 'projects'],
    properties: { type: 'heavy_duty', vise: true, lighting: true },
  },

  {
    object_id: 'furnace',
    location_id: 'tool_shed',
    name: 'Furnace',
    type: 'furnace',
    coordinates: { x: 460, y: 360, z: 0 },
    dimensions: { width: 1.5, height: 2, depth: 1.5 },
    weight: 200,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 5,
    current_state: { temperature: 20, fueled: false },
    inventory: [],
    properties: { type: 'electric', max_temp: 1500, crucible: true },
  },

  {
    object_id: 'anvil',
    location_id: 'tool_shed',
    name: 'Anvil',
    type: 'anvil',
    coordinates: { x: 462, y: 358, z: 0 },
    dimensions: { width: 0.5, height: 0.8, depth: 0.3 },
    weight: 150,
    is_interactable: true,
    is_usable: true,
    is_movable: false,
    capacity: 0,
    current_state: { surface_condition: 'good' },
    inventory: [],
    properties: { type: 'steel', weight: '100kg', horned: true },
  },
];

// ============================================================================
// VEHICLES
// ============================================================================

export const WORLD_VEHICLES: WorldVehicle[] = [
  // Off-road vehicles
  {
    vehicle_id: 'rzr_turbo',
    name: 'Polaris RZR 1000 Turbo',
    type: 'rzr',
    model: 'RZR 1000 Turbo',
    location_id: 'garage',
    coordinates: { x: 485, y: 395, z: 0 },
    rotation: 0,
    storage_capacity: 20,
    fuel_type: 'gasoline',
    fuel_level: 75,
    max_speed: 80,
    is_mounted: false,
    is_locked: false,
    inventory: ['tool_kit', 'first_aid', 'rope'],
    condition: 0.95,
  },

  {
    vehicle_id: 'rzr_pro_r',
    name: 'Polaris RZR 2000 PRO R',
    type: 'rzr',
    model: 'RZR 2000 PRO R',
    location_id: 'garage',
    coordinates: { x: 490, y: 395, z: 0 },
    rotation: 0,
    storage_capacity: 25,
    fuel_type: 'gasoline',
    fuel_level: 80,
    max_speed: 100,
    is_mounted: false,
    is_locked: false,
    inventory: ['tool_kit', 'first_aid', 'rope', 'gps'],
    condition: 0.98,
  },

  {
    vehicle_id: 'ford_raptor',
    name: 'Ford Raptor 4x4 Ute',
    type: 'truck',
    model: 'F-150 Raptor',
    location_id: 'garage',
    coordinates: { x: 495, y: 395, z: 0 },
    rotation: 90,
    storage_capacity: 100,
    fuel_type: 'gasoline',
    fuel_level: 60,
    max_speed: 120,
    is_mounted: false,
    is_locked: false,
    inventory: ['tool_kit', 'first_aid', 'tow_strap', 'jumper_cables'],
    condition: 0.90,
  },

  {
    vehicle_id: 'golf_cart',
    name: 'Golf Cart',
    type: 'golf_cart',
    model: 'Club Car Electric',
    location_id: 'garage',
    coordinates: { x: 485, y: 405, z: 0 },
    rotation: 180,
    storage_capacity: 15,
    fuel_type: 'electric',
    fuel_level: 90,
    max_speed: 25,
    is_mounted: false,
    is_locked: false,
    inventory: [],
    condition: 0.85,
  },

  // Agricultural vehicles
  {
    vehicle_id: 'fendt_900',
    name: 'Fendt 900 Vario',
    type: 'tractor',
    model: 'Fendt 900 Vario',
    location_id: 'garage',
    coordinates: { x: 470, y: 395, z: 0 },
    rotation: 270,
    storage_capacity: 50,
    fuel_type: 'diesel',
    fuel_level: 70,
    max_speed: 50,
    is_mounted: false,
    is_locked: false,
    inventory: ['stoll_loader_900', 'plow', 'mower', 'forklift'],
    condition: 0.95,
  },

  {
    vehicle_id: 'fendt_1000',
    name: 'Fendt 1000 Vario',
    type: 'tractor',
    model: 'Fendt 1000 Vario',
    location_id: 'garage',
    coordinates: { x: 470, y: 405, z: 0 },
    rotation: 270,
    storage_capacity: 60,
    fuel_type: 'diesel',
    fuel_level: 65,
    max_speed: 60,
    is_mounted: false,
    is_locked: false,
    inventory: ['stoll_loader_1000', 'plow', 'mower', 'forklift', 'baler'],
    condition: 0.92,
  },

  // Heavy equipment
  {
    vehicle_id: 'digger_30ton',
    name: '30-Ton Excavator',
    type: 'excavator',
    model: 'CAT 330',
    location_id: 'garage',
    coordinates: { x: 460, y: 400, z: 0 },
    rotation: 0,
    storage_capacity: 5,
    fuel_type: 'diesel',
    fuel_level: 80,
    max_speed: 5,
    is_mounted: false,
    is_locked: false,
    inventory: [],
    condition: 0.88,
  },

  // Trailers
  {
    vehicle_id: 'double_axle_trailer',
    name: 'Double Axle Trailer',
    type: 'trailer',
    model: 'Heavy Duty 20ft',
    location_id: 'garage',
    coordinates: { x: 500, y: 390, z: 0 },
    rotation: 0,
    storage_capacity: 200,
    fuel_type: 'none',
    fuel_level: 0,
    max_speed: 0,
    is_mounted: false,
    is_locked: false,
    inventory: [],
    condition: 0.95,
  },

  {
    vehicle_id: 'tipper_trailer',
    name: 'Tractor Tipper Trailer',
    type: 'tipper_trailer',
    model: 'Agricultural 15ft',
    location_id: 'garage',
    coordinates: { x: 460, y: 410, z: 0 },
    rotation: 0,
    storage_capacity: 150,
    fuel_type: 'none',
    fuel_level: 0,
    max_speed: 0,
    is_mounted: false,
    is_locked: false,
    inventory: [],
    condition: 0.90,
  },
];

// ============================================================================
// MINEABLE RESOURCES
// ============================================================================

export const WORLD_RESOURCES: WorldResource[] = [
  // Iron deposits
  {
    resource_id: 'iron_vein_1',
    type: 'iron',
    grade: 'medium',
    location_id: 'island_east',
    coordinates: { x: 720, y: 420, z: -10 },
    quantity: 500,
    max_quantity: 500,
    difficulty: 3,
    tool_required: 'pickaxe',
    regeneration_rate: 0.1, // 0.1 units per day
  },

  {
    resource_id: 'iron_vein_2',
    type: 'iron',
    grade: 'low',
    location_id: 'island_east',
    coordinates: { x: 750, y: 450, z: -15 },
    quantity: 300,
    max_quantity: 300,
    difficulty: 2,
    tool_required: 'pickaxe',
    regeneration_rate: 0.05,
  },

  // Copper deposits
  {
    resource_id: 'copper_vein_1',
    type: 'copper',
    grade: 'high',
    location_id: 'island_east',
    coordinates: { x: 780, y: 380, z: -20 },
    quantity: 200,
    max_quantity: 200,
    difficulty: 4,
    tool_required: 'pickaxe',
    regeneration_rate: 0.02,
  },

  {
    resource_id: 'copper_vein_2',
    type: 'copper',
    grade: 'medium',
    location_id: 'island_east',
    coordinates: { x: 730, y: 410, z: -12 },
    quantity: 350,
    max_quantity: 350,
    difficulty: 3,
    tool_required: 'pickaxe',
    regeneration_rate: 0.08,
  },

  // Coal deposits
  {
    resource_id: 'coal_deposit_1',
    type: 'coal',
    grade: 'high',
    location_id: 'island_east',
    coordinates: { x: 700, y: 430, z: -8 },
    quantity: 800,
    max_quantity: 800,
    difficulty: 2,
    tool_required: 'shovel',
    regeneration_rate: 0.15,
  },

  {
    resource_id: 'coal_deposit_2',
    type: 'coal',
    grade: 'medium',
    location_id: 'island_east',
    coordinates: { x: 760, y: 400, z: -10 },
    quantity: 600,
    max_quantity: 600,
    difficulty: 2,
    tool_required: 'shovel',
    regeneration_rate: 0.1,
  },

  // Rare ore deposits
  {
    resource_id: 'rare_ore_1',
    type: 'rare',
    grade: 'ultra',
    location_id: 'island_east',
    coordinates: { x: 800, y: 420, z: -25 },
    quantity: 50,
    max_quantity: 50,
    difficulty: 8,
    tool_required: 'advanced_pickaxe',
    regeneration_rate: 0.001, // Very slow regeneration
  },

  {
    resource_id: 'rare_ore_2',
    type: 'rare',
    grade: 'high',
    location_id: 'island_east',
    coordinates: { x: 770, y: 460, z: -18 },
    quantity: 100,
    max_quantity: 100,
    difficulty: 6,
    tool_required: 'advanced_pickaxe',
    regeneration_rate: 0.005,
  },
];

// ============================================================================
// TOOL SHED INVENTORY
// ============================================================================

export const TOOL_SHED_INVENTORY = {
  construction_tools: [
    'hammer_set',
    'screwdriver_set',
    'power_drill',
    'circular_saw',
    'measuring_tape',
    'level',
    'wrench_set',
    'pliers_set',
  ],
  digging_tools: [
    'shovel',
    'spade',
    'pickaxe',
    'post_hole_digger',
    'rake',
    'hoe',
  ],
  repair_tools: [
    'welding_torch',
    'soldering_iron',
    'multimeter',
    'wire_stripper',
    'socket_set',
    'torque_wrench',
  ],
  exploration_tools: [
    'compass',
    'binoculars',
    'flashlight_set',
    'rope_50ft',
    'climbing_gear',
    'gps_device',
  ],
  smithing_tools: [
    'blacksmith_hammer',
    'tongs_set',
    'chisel_set',
    'grinder',
    'forge_tools',
    'quench_tank',
  ],
};

// ============================================================================
// WORLD VALIDATION
// ============================================================================

export const validateWorldDefinition = () => {
  const errors: string[] = [];

  // Check for king beds in both twin rooms
  const kingBeds = WORLD_OBJECTS.filter(obj => 
    obj.type === 'bed' && obj.subtype === 'king'
  );
  
  if (kingBeds.length !== 2) {
    errors.push(`Expected 2 king beds, found ${kingBeds.length}`);
  }

  const bedLocations = kingBeds.map(bed => bed.location_id);
  if (!bedLocations.includes('twin_room_a') || !bedLocations.includes('twin_room_b')) {
    errors.push('King beds must be in both twin rooms');
  }

  // Check bed capacity
  kingBeds.forEach(bed => {
    if (bed.capacity !== 2) {
      errors.push(`King bed ${bed.object_id} must have capacity 2, has ${bed.capacity}`);
    }
  });

  // Check sleep permissions
  const twinRooms = ISLAND_LOCATIONS.filter(loc => 
    loc.location_id.startsWith('twin_room')
  );
  
  twinRooms.forEach(room => {
    if (!room.sleep_allowed) {
      errors.push(`Twin room ${room.location_id} must allow sleeping`);
    }
    if (room.max_occupancy < 4) {
      errors.push(`Twin room ${room.location_id} must support at least 4 occupants`);
    }
  });

  // Check vehicles
  if (WORLD_VEHICLES.length !== 3) {
    errors.push(`Expected 3 vehicles, found ${WORLD_VEHICLES.length}`);
  }

  // Check resources
  const resourceTypes = [...new Set(WORLD_RESOURCES.map(r => r.type))];
  const requiredTypes = ['iron', 'copper', 'coal', 'rare'];
  
  requiredTypes.forEach(type => {
    if (!resourceTypes.includes(type)) {
      errors.push(`Missing resource type: ${type}`);
    }
  });

  return {
    isValid: errors.length === 0,
    errors,
    summary: {
      locations: ISLAND_LOCATIONS.length,
      objects: WORLD_OBJECTS.length,
      vehicles: WORLD_VEHICLES.length,
      resources: WORLD_RESOURCES.length,
      kingBeds: kingBeds.length,
    }
  };
};
