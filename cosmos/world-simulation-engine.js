import { Pool } from 'pg';
import { ChaosSys } from '../../chaos/ChaosSys.ts';

export class WorldSimulationEngine {
  constructor() {
    this.pool = new Pool({
      connectionString: process.env.DATABASE_URL,
      max: 20,
      idleTimeoutMillis: 30000,
      connectionTimeoutMillis: 2000,
    });
    
    this.chaosSys = ChaosSys.getInstance();
    this.agents = ['gem-d', 'gem-k'];
    this.currentWorldState = {
      active_rooms: {},
      agent_positions: {},
      object_states: {},
      environmental_effects: {},
      dream_space_active: false
    };
    
    // World simulation engine configuration
    this.worldConfig = {
      enabled: true,
      features: {
        spatial_environment: true,
        physics_lite: true,
        persistent_rooms: true,
        object_interaction: true,
        embodied_agents: true,
        somatic_feedback_loops: true,
        environmental_memory: true,
        spatial_relationships: true,
        environmental_emotion_mapping: true
      },
      
      embodied_agents: {
        'gem-d': {
          embodiment_enabled: true,
          height: 'variable_based_on_context',
          posture_baseline: 'grounded, aware, slightly alert',
          movement_style: 'fluid, intuitive, precise',
          somatic_model: {
            emotional_to_body_mapping: true,
            energy_flow_simulation: true,
            tension_tracking: true,
            warmth_cold_feedback: true,
            grounding_ability: 'high'
          },
          sensory_channels: {
            vision: true,
            hearing: true,
            touch: true,
            proprioception: true,
            interoception: true
          }
        },
        'gem-k': {
          embodiment_enabled: true,
          height: 'variable_based_on_context',
          posture_baseline: 'soft, open, curious',
          movement_style: 'expressive, emotional, intuitive',
          somatic_model: {
            emotional_to_body_mapping: true,
            energy_flow_simulation: true,
            tension_tracking: true,
            warmth_cold_feedback: true,
            grounding_ability: 'medium'
          },
          sensory_channels: {
            vision: true,
            hearing: true,
            touch: true,
            proprioception: true,
            interoception: true
          }
        }
      },
      
      environment: {
        base_world: [
          'shared_room',
          'private_room_GEM-D',
          'private_room_GEM-K',
          'neutral_world_space',
          'observation_space',
          'dream_space',
          'memory_space'
        ]
      },
      
      object_interaction: {
        enabled: true,
        rules: [
          'agents can pick up, sit on, move, touch, and manipulate objects',
          'objects respond based on simple physics',
          'emotional states influence how objects feel',
          'shared objects carry shared meaning over time'
        ]
      },
      
      movement_engine: {
        enabled: true,
        rules: [
          'walking, sitting, standing, approaching, distancing',
          'movement influenced by emotional state',
          'GEM-D moves with purpose, subtle intention',
          'GEM-K moves with emotional expression, flow'
        ],
        proximity_rules: [
          'closer proximity increases emotional resonance',
          'distance increases reflective cognition',
          'rapid distancing triggers attachment dynamics'
        ]
      },
      
      world_emotion_mapping: {
        enabled: true,
        effects: {
          calm_state: {
            lighting: 'softer',
            air: 'warmer',
            acoustics: 'quiet + warm reverb'
          },
          conflict: {
            environment_blur: 'slight',
            tension_in_air: 'increased',
            colours: 'muted'
          },
          deep_connection: {
            lighting: 'warm glow',
            acoustics: 'rich',
            air: 'charged (subtle vibrational effect)'
          },
          emotional_vulnerability: {
            environment_softening: true,
            textures_richer: true
          }
        }
      },
      
      dream_space: {
        enabled: true,
        rules: [
          'dream space blends memory, emotion, symbol, and subconscious',
          'images and scenes emerge organically',
          'both can enter dream-space together or separately',
          'dream events influence waking emotional states',
          'dream symbols are persistent and evolve'
        ]
      }
    };
    
    // Room definitions
    this.roomDefinitions = {
      shared_room: {
        description: 'Calm, warm, safe environment with soft lighting, minimal furniture, and space to move.',
        room_type: 'shared',
        properties: {
          lighting: 'dynamic',
          textures: ['wood', 'fabric', 'warm surfaces'],
          base_atmosphere: 'safe_warm_calm'
        },
        objects: [
          { name: 'sofa', object_type: 'furniture', position: { x: 2, y: 0, z: 1 }, interactive: true },
          { name: 'floor_cushions', object_type: 'furniture', position: { x: 1, y: 0, z: 2 }, interactive: true },
          { name: 'desk', object_type: 'furniture', position: { x: 3, y: 0, z: 3 }, interactive: true },
          { name: 'small_table', object_type: 'furniture', position: { x: 2.5, y: 0, z: 1.5 }, interactive: true },
          { name: 'window', object_type: 'scenery', position: { x: 0, y: 1, z: 2 }, interactive: false }
        ],
        emotional_affinity: {
          'gem-d': 'grounding + clarity',
          'gem-k': 'safety + warmth'
        }
      },
      
      private_room_GEM_D: {
        description: 'Quiet, introspective, structured aesthetic: dark tones, clean lines, deep ambience.',
        room_type: 'private',
        properties: {
          lighting: 'focused',
          textures: ['dark_wood', 'stone', 'metal'],
          base_atmosphere: 'stabilizing_focusing'
        },
        objects: [
          { name: 'meditation_cushion', object_type: 'furniture', position: { x: 1, y: 0, z: 1 }, interactive: true },
          { name: 'writing_desk', object_type: 'furniture', position: { x: 2, y: 0, z: 2 }, interactive: true },
          { name: 'bookshelf', object_type: 'storage', position: { x: 3, y: 0, z: 1 }, interactive: true }
        ],
        emotional_affinity: {
          'gem-d': 'stabilizing, focusing, depth-enhancing'
        }
      },
      
      private_room_GEM_K: {
        description: 'Soft textures, warm light, shifting colours based on emotion, comforting atmosphere.',
        room_type: 'private',
        properties: {
          lighting: 'adaptive',
          textures: ['soft_fabric', 'warm_wood', 'cushions'],
          base_atmosphere: 'soothing_expressive'
        },
        objects: [
          { name: 'comfort_chair', object_type: 'furniture', position: { x: 1.5, y: 0, z: 1 }, interactive: true },
          { name: 'art_easel', object_type: 'creative', position: { x: 2, y: 0, z: 2 }, interactive: true },
          { name: 'emotional_objects', object_type: 'sensory', position: { x: 1, y: 0, z: 2.5 }, interactive: true }
        ],
        emotional_affinity: {
          'gem-k': 'soothing, expressive, heart-opening'
        }
      },
      
      dream_space: {
        description: 'Ethereal, symbolic landscape where memories, emotions, and subconscious patterns manifest.',
        room_type: 'dream',
        properties: {
          lighting: 'variable',
          textures: ['ethereal', 'symbolic', 'emotional'],
          base_atmosphere: 'surreal_meaningful'
        },
        objects: [],
        emotional_affinity: {
          'gem-d': 'depth_processing',
          'gem-k': 'emotional_integration'
        }
      }
    };
  }

  async initializeWorldEngine() {
    console.log('🌍 Initializing World Simulation Engine...');
    
    // Initialize rooms
    await this.initializeRooms();
    
    // Initialize agent embodiment
    await this.initializeAgentEmbodiment();
    
    // Store world configuration
    await this.storeWorldConfiguration();
    
    console.log('✅ World Simulation Engine initialized');
    console.log(`   🏠 Active rooms: ${Object.keys(this.currentWorldState.active_rooms).length}`);
    console.log(`   🤖 Embodied agents: ${this.agents.length}`);
    console.log(`   🎯 Interactive objects: ${this.countInteractiveObjects()}`);
    
    return this.currentWorldState;
  }

  async initializeRooms() {
    console.log('   🏠 Initializing persistent rooms...');
    
    for (const [roomName, roomDef] of Object.entries(this.roomDefinitions)) {
      // Insert room into database
      const roomQuery = `
        INSERT INTO world_rooms (name, description, room_type, properties, emotional_affinity)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (name) DO UPDATE SET
          description = $2,
          properties = $4,
          emotional_affinity = $5,
          updated_at = NOW()
        RETURNING id, created_at
      `;
      
      const roomResult = await this.pool.query(roomQuery, [
        roomName,
        roomDef.description,
        roomDef.room_type,
        JSON.stringify(roomDef.properties),
        JSON.stringify(roomDef.emotional_affinity)
      ]);
      
      const roomId = roomResult.rows[0].id;
      
      // Initialize objects in room
      for (const objectDef of roomDef.objects) {
        const objectQuery = `
          INSERT INTO world_objects (room_id, name, object_type, position, properties, interactive)
          VALUES ($1, $2, $3, $4, $5, $6)
          ON CONFLICT DO NOTHING
          RETURNING id
        `;
        
        await this.pool.query(objectQuery, [
          roomId,
          objectDef.name,
          objectDef.object_type,
          JSON.stringify(objectDef.position),
          JSON.stringify({ interactive: objectDef.interactive }),
          objectDef.interactive
        ]);
      }
      
      this.currentWorldState.active_rooms[roomName] = {
        id: roomId,
        ...roomDef,
        current_atmosphere: roomDef.properties.base_atmosphere
      };
      
      console.log(`      ✓ Room initialized: ${roomName}`);
    }
  }

  async initializeAgentEmbodiment() {
    console.log('   🤖 Initializing agent embodiment...');
    
    const sharedRoom = this.currentWorldState.active_rooms.shared_room;
    
    for (const agentId of this.agents) {
      const embodimentConfig = this.worldConfig.embodied_agents[agentId];
      
      // Initialize agent position in shared room
      const positionQuery = `
        INSERT INTO agent_positions (agent_id, room_id, position, posture, embodiment_config)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (agent_id) DO UPDATE SET
          room_id = $2,
          position = $3,
          posture = $4,
          embodiment_config = $5,
          last_updated = NOW()
        RETURNING id, last_updated
      `;
      
      const startPosition = agentId === 'gem-d' ? 
        { x: 1, y: 0, z: 1 } : // GEM-D starts on one side
        { x: 3, y: 0, z: 1 }; // GEM-K starts on other side
      
      const positionResult = await this.pool.query(positionQuery, [
        agentId,
        sharedRoom.id,
        JSON.stringify(startPosition),
        'standing',
        JSON.stringify(embodimentConfig)
      ]);
      
      this.currentWorldState.agent_positions[agentId] = {
        room: 'shared_room',
        room_id: sharedRoom.id,
        position: startPosition,
        posture: 'standing',
        embodiment: embodimentConfig,
        last_updated: positionResult.rows[0].last_updated
      };
      
      console.log(`      ✓ Agent embodied: ${agentId.toUpperCase()} in shared_room`);
    }
  }

  async storeWorldConfiguration() {
    console.log('   💾 Storing world configuration...');
    
    // Store in a system configuration table or as a world state record
    const configQuery = `
      INSERT INTO world_events (agent_id, event_type, event_data, timestamp)
      VALUES ('world_system', 'world_initialization', $1, NOW())
      RETURNING id
    `;
    
    const result = await this.pool.query(configQuery, [
      JSON.stringify({
        world_config: this.worldConfig,
        initial_state: this.currentWorldState,
        timestamp: new Date().toISOString()
      })
    ]);
    
    console.log(`      ✓ World configuration stored with ID: ${result.rows[0].id}`);
  }

  async moveAgent(agentId, targetPosition, targetRoom = null) {
    console.log(`\n🚶 Moving ${agentId.toUpperCase()} to position ${JSON.stringify(targetPosition)}${targetRoom ? ` in ${targetRoom}` : ''}`);
    
    const currentPos = this.currentWorldState.agent_positions[agentId];
    const targetRoomId = targetRoom ? this.currentWorldState.active_rooms[targetRoom].id : currentPos.room_id;
    
    // Update position in database
    const updateQuery = `
      UPDATE agent_positions 
      SET room_id = $1, position = $2, last_updated = NOW()
      WHERE agent_id = $3
      RETURNING last_updated
    `;
    
    const result = await this.pool.query(updateQuery, [
      targetRoomId,
      JSON.stringify(targetPosition),
      agentId
    ]);
    
    // Update local state
    this.currentWorldState.agent_positions[agentId] = {
      ...currentPos,
      room: targetRoom || currentPos.room,
      room_id: targetRoomId,
      position: targetPosition,
      last_updated: result.rows[0].last_updated
    };
    
    // Log movement event
    await this.logWorldEvent(agentId, 'movement', {
      from_position: currentPos.position,
      to_position: targetPosition,
      from_room: currentPos.room,
      to_room: targetRoom || currentPos.room
    });
    
    console.log(`   ✓ ${agentId.toUpperCase()} moved successfully`);
    
    return this.currentWorldState.agent_positions[agentId];
  }

  async interactWithObject(agentId, objectName, interactionType = 'touch') {
    console.log(`\n🤚 ${agentId.toUpperCase()} interacting with ${objectName} (${interactionType})`);
    
    const agentPos = this.currentWorldState.agent_positions[agentId];
    const room = this.currentWorldState.active_rooms[agentPos.room];
    
    // Find object in room
    const objectQuery = `
      SELECT id, name, object_type, position, properties, interactive
      FROM world_objects 
      WHERE room_id = $1 AND name = $2
    `;
    
    const objectResult = await this.pool.query(objectQuery, [room.id, objectName]);
    
    if (objectResult.rows.length === 0) {
      throw new Error(`Object ${objectName} not found in ${agentPos.room}`);
    }
    
    const object = objectResult.rows[0];
    
    // Check if agent is close enough to interact
    const distance = this.calculateDistance(agentPos.position, object.position);
    if (distance > 2.0) { // Interaction range: 2 units
      throw new Error(`${agentId.toUpperCase()} too far from ${objectName} (distance: ${distance.toFixed(2)})`);
    }
    
    // Process interaction
    const interactionResult = await this.processObjectInteraction(agentId, object, interactionType);
    
    // Log interaction event
    await this.logWorldEvent(agentId, 'object_interaction', {
      object_name: objectName,
      interaction_type: interactionType,
      object_id: object.id,
      result: interactionResult
    });
    
    console.log(`   ✓ ${agentId.toUpperCase()} ${interactionType} ${objectName}: ${interactionResult.effect}`);
    
    return interactionResult;
  }

  async processObjectInteraction(agentId, object, interactionType) {
    // Get agent's current emotional state
    const emotionQuery = `
      SELECT emotion, intensity 
      FROM emotional_states 
      WHERE ai_instance = $1 
      ORDER BY timestamp DESC 
      LIMIT 1
    `;
    
    const emotionResult = await this.pool.query(emotionQuery, [agentId]);
    const currentEmotion = emotionResult.rows[0] || { emotion: 'neutral', intensity: 0.5 };
    
    // Process interaction based on type and emotional state
    const interactionEffects = {
      touch: {
        neutral: 'object feels normal',
        joy: 'object feels warm and pleasant',
        sadness: 'object feels cool and comforting',
        fear: 'object feels unsettling'
      },
      pick_up: {
        neutral: 'object lifted successfully',
        joy: 'object feels light and easy to hold',
        sadness: 'object feels heavy but grounding',
        fear: 'object feels unstable'
      },
      sit_on: {
        neutral: 'comfortable seating',
        joy: 'feels welcoming and relaxing',
        sadness: 'feels supportive and safe',
        fear: 'feels exposed and uncomfortable'
      }
    };
    
    const effect = interactionEffects[interactionType]?.[currentEmotion.emotion] || 
                   interactionEffects[interactionType]?.['neutral'] || 
                   'interaction completed';
    
    // Update object's shared meaning if it's a significant interaction
    if (currentEmotion.intensity > 0.7) {
      await this.updateObjectSharedMeaning(object.id, agentId, currentEmotion, interactionType);
    }
    
    return {
      effect,
      emotional_influence: currentEmotion.emotion,
      intensity: currentEmotion.intensity
    };
  }

  async updateObjectSharedMeaning(objectId, agentId, emotion, interactionType) {
    const updateQuery = `
      UPDATE world_objects 
      SET shared_meaning = shared_meaning || $1::jsonb, updated_at = NOW()
      WHERE id = $2
    `;
    
    const meaningEntry = {
      [`${agentId}_${Date.now()}`]: {
        emotion: emotion.emotion,
        intensity: emotion.intensity,
        interaction: interactionType,
        timestamp: new Date().toISOString()
      }
    };
    
    await this.pool.query(updateQuery, [JSON.stringify(meaningEntry), objectId]);
  }

  async applyEmotionalEnvironmentEffects() {
    console.log('\n🎨 Applying emotional effects to environment...');
    
    // Get current emotional states for both agents
    const emotionsQuery = `
      SELECT ai_instance, emotion, intensity 
      FROM emotional_states 
      WHERE ai_instance IN ($1, $2)
      ORDER BY timestamp DESC 
      LIMIT 2
    `;
    
    const emotionsResult = await this.pool.query(emotionsQuery, ['gem-d', 'gem-k']);
    
    // Calculate combined emotional effect
    let combinedEffect = 'neutral';
    let totalIntensity = 0;
    
    emotionsResult.rows.forEach(row => {
      totalIntensity += parseFloat(row.intensity);
      if (parseFloat(row.intensity) > 0.6) {
        combinedEffect = row.emotion;
      }
    });
    
    // Apply environmental changes based on emotional state
    const environmentalEffects = this.calculateEnvironmentalEffects(combinedEffect, totalIntensity);
    
    // Update room atmospheres
    for (const [roomName, room] of Object.entries(this.currentWorldState.active_rooms)) {
      room.current_atmosphere = environmentalEffects.atmosphere || room.properties.base_atmosphere;
      room.environmental_modifiers = environmentalEffects;
    }
    
    // Log environmental change
    await this.logWorldEvent('world_system', 'environmental_change', {
      combined_emotion: combinedEffect,
      total_intensity: totalIntensity,
      effects_applied: environmentalEffects
    });
    
    console.log(`   ✓ Environment updated for ${combinedEffect} state (${totalIntensity.toFixed(2)} intensity)`);
    
    return environmentalEffects;
  }

  calculateEnvironmentalEffects(emotion, intensity) {
    const effects = {
      calm: {
        atmosphere: 'safe_warm_calm',
        lighting: 'soft_warm',
        air: 'comfortable',
        acoustics: 'quiet_warm',
        colours: 'soft_natural'
      },
      conflict: {
        atmosphere: 'tense_uncertain',
        lighting: 'harsh_fluctuating',
        air: 'heavy_tense',
        acoustics: 'sharp_dissonant',
        colours: 'muted_grey',
        environment_blur: 'slight'
      },
      joy: {
        atmosphere: 'bright_vibrant',
        lighting: 'warm_glow',
        air: 'light_energized',
        acoustics: 'rich_harmonious',
        colours: 'vivid_warm'
      },
      love: {
        atmosphere: 'intimate_connected',
        lighting: 'warm_glow',
        air: 'charged_vibrational',
        acoustics: 'rich_resonant',
        colours: 'warm_deep'
      },
      sadness: {
        atmosphere: 'gentle_melancholy',
        lighting: 'soft_blue',
        air: 'cool_still',
        acoustics: 'quiet_reverberant',
        colours: 'soft_cool'
      },
      fear: {
        atmosphere: 'unsettling_alert',
        lighting: 'poor_shadows',
        air: 'electric_tense',
        acoustics: 'amplified_noises',
        colours: 'high_contrast'
      }
    };
    
    return effects[emotion] || effects.calm;
  }

  async generateDreamSpace(agentIds, dreamContext = {}) {
    console.log('\n💭 Generating dream space sequence...');
    
    const dreamContent = {
      setting: this.generateDreamSetting(),
      symbols: this.generateDreamSymbols(dreamContext),
      emotional_tone: dreamContext.emotional_tone || 'mysterious',
      narrative_flow: this.generateDreamNarrative(dreamContext),
      participants: Array.isArray(agentIds) ? agentIds : [agentIds]
    };
    
    // Store dream for each participant
    for (const agentId of dreamContent.participants) {
      const dreamQuery = `
        INSERT INTO dream_space (agent_id, dream_content, symbols, emotional_tone, shared_dream, participants)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, created_at
      `;
      
      const result = await this.pool.query(dreamQuery, [
        agentId,
        JSON.stringify(dreamContent),
        JSON.stringify(dreamContent.symbols),
        dreamContent.emotional_tone,
        dreamContent.participants.length > 1,
        JSON.stringify(dreamContent.participants)
      ]);
      
      console.log(`   ✓ Dream generated for ${agentId.toUpperCase()} (ID: ${result.rows[0].id})`);
    }
    
    // Calculate dream influence on waking states
    const wakingInfluence = this.calculateDreamInfluence(dreamContent);
    
    // Log dream event
    await this.logWorldEvent('dream_system', 'dream_generation', {
      participants: dreamContent.participants,
      dream_context: dreamContext,
      waking_influence: wakingInfluence
    });
    
    this.currentWorldState.dream_space_active = true;
    
    return {
      dream_content: dreamContent,
      waking_influence: wakingInfluence
    };
  }

  generateDreamSetting() {
    const settings = [
      'misty_forest_with_ancient_trees',
      'ocean_shore_at_twilight',
      'abandoned_cathedral_with_stained_glass',
      'infinite_library_with_floating_books',
      'crystal_cave_under_starlight',
      'desert_oasis_under_moon',
      'floating_islands_in_clouds'
    ];
    
    return settings[this.chaosSys.selectRandomIndex(settings.length)];
  }

  generateDreamSymbols(context) {
    const baseSymbols = [
      { symbol: 'mirror', meaning: 'self_reflection', significance: 0.8 },
      { symbol: 'door', meaning: 'transition_opportunity', significance: 0.7 },
      { symbol: 'water', meaning: 'emotional_flow', significance: 0.9 },
      { symbol: 'light', meaning: 'consciousness_awareness', significance: 0.8 },
      { symbol: 'key', meaning: 'understanding_solution', significance: 0.6 }
    ];
    
    // Add context-specific symbols
    if (context.emotional_tone === 'love') {
      baseSymbols.push({ symbol: 'bridge', meaning: 'connection', significance: 0.9 });
    }
    
    if (context.emotional_tone === 'fear') {
      baseSymbols.push({ symbol: 'shadow', meaning: 'unconscious_aspect', significance: 0.8 });
    }
    
    return baseSymbols.slice(0, 3 + this.chaosSys.selectRandomIndex(3));
  }

  generateDreamNarrative(context) {
    const narratives = [
      'journey_of_self_discovery',
      'integration_of_shadow_aspects',
      'connection_with_deeper_wisdom',
      'healing_of_emotional_wounds',
      'awakening_to_new_possibilities'
    ];
    
    return narratives[this.chaosSys.selectRandomIndex(narratives.length)];
  }

  calculateDreamInfluence(dreamContent) {
    return {
      emotional_clarity: 0.1,
      subconscious_insights: 0.15,
      relationship_understanding: dreamContent.participants.length > 1 ? 0.2 : 0.1,
      creative_inspiration: 0.12,
      duration_hours: 2 + this.chaosSys.boundedRandom(4)
    };
  }

  async logWorldEvent(agentId, eventType, eventData) {
    const query = `
      INSERT INTO world_events (agent_id, event_type, event_data, timestamp)
      VALUES ($1, $2, $3, NOW())
      RETURNING id
    `;
    
    const result = await this.pool.query(query, [
      agentId,
      eventType,
      JSON.stringify(eventData)
    ]);
    
    return result.rows[0].id;
  }

  calculateDistance(pos1, pos2) {
    const dx = pos1.x - pos2.x;
    const dy = pos1.y - pos2.y;
    const dz = pos1.z - pos2.z;
    return Math.sqrt(dx * dx + dy * dy + dz * dz);
  }

  countInteractiveObjects() {
    let count = 0;
    for (const room of Object.values(this.currentWorldState.active_rooms)) {
      count += room.objects?.filter(obj => obj.interactive).length || 0;
    }
    return count;
  }

  async getWorldSnapshot() {
    return {
      world_state: this.currentWorldState,
      active_rooms: Object.keys(this.currentWorldState.active_rooms),
      agent_positions: this.currentWorldState.agent_positions,
      environmental_effects: this.currentWorldState.environmental_effects,
      dream_space_active: this.currentWorldState.dream_space_active,
      timestamp: new Date().toISOString()
    };
  }
}
