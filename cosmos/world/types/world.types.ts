
// World type definitions for canonical world v3

export interface Coordinates {
  x: number;
  y: number;
  z: number;
}

export interface Dimensions {
  width: number;
  height: number;
  depth: number;
}

export interface WorldLocation {
  location_id: string;
  name: string;
  type: 'room' | 'outdoor' | 'structure';
  parent_location?: string;
  coordinates: Coordinates;
  dimensions: Dimensions;
  sleep_allowed: boolean;
  max_occupancy: number;
  allowed_activities: string[];
  interaction_permissions: Record<string, string>;
  utilities: Record<string, any>;
  base_inventory: string[];
  affordances: Record<string, any>;
  is_active?: boolean;
}

export interface WorldObject {
  object_id: string;
  location_id: string;
  name: string;
  type: string;
  subtype?: string;
  coordinates: Coordinates;
  dimensions: Dimensions;
  weight?: number;
  is_interactable: boolean;
  is_usable: boolean;
  is_movable: boolean;
  capacity?: number;
  current_state?: Record<string, any>;
  inventory?: any[];
  properties?: Record<string, any>;
  owner_agent?: string;
  required_permissions?: string[];
  is_active?: boolean;
  condition?: number;
}

export interface WorldVehicle {
  vehicle_id: string;
  name: string;
  type: string;
  model: string;
  location_id: string;
  coordinates: Coordinates;
  rotation: number;
  storage_capacity: number;
  fuel_type?: string;
  fuel_level: number;
  max_speed?: number;
  is_mounted: boolean;
  mounted_agent?: string;
  is_locked: boolean;
  inventory?: any[];
  condition?: number;
  is_active?: boolean;
}

export interface WorldResource {
  resource_id: string;
  type: string;
  grade: string;
  location_id: string;
  coordinates: Coordinates;
  quantity: number;
  max_quantity?: number;
  difficulty: number;
  tool_required?: string;
  regeneration_rate?: number;
  is_depleted?: boolean;
  last_mined?: Date;
}

export interface AgentSleepingState {
  id?: number;
  agent_id: string;
  location_id: string;
  object_id?: string;
  sleep_start: Date;
  sleep_end?: Date;
  is_sleeping: boolean;
  is_voluntary: boolean;
  comfort_level?: number;
  noise_level?: number;
  temperature_comfort?: number;
  reason?: string;
  last_position?: Coordinates;
  created_at?: Date;
  updated_at?: Date;
}

export interface AgentWorldPosition {
  id?: number;
  agent_id: string;
  location_id: string;
  coordinates: Coordinates;
  rotation: number;
  is_moving: boolean;
  target_location?: string;
  target_coordinates?: Coordinates;
  move_start_time?: Date;
  estimated_arrival?: Date;
  vehicle_id?: string;
  is_mounted: boolean;
  mount_position?: string;
  last_activity?: string;
  last_state_change: Date;
  created_at?: Date;
  updated_at?: Date;
}

export interface SpawnReason {
  type: 'GENESIS' | 'REHYDRATED';
  description: string;
}

export interface SpawnLocation {
  location_id: string;
  coordinates: Coordinates;
  rotation: number;
  reason: SpawnReason;
}
