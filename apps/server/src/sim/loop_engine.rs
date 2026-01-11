/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Fixed timestep simulation loop
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use deterministic::DeterministicMap;
use crate::sim::{Event, EventKind, InputEvent, SimTime, ChaosStream};

pub struct SimLoop {
    current_time: SimTime,
    accumulator: f64,
    dt: f64,
    speed_multiplier: f64,
    paused: bool,
    events: DeterministicMap<u64, Event>,
    next_seq: u64,
    rng: ChaosStream,
}

#[derive(Debug, Clone)]
pub enum TickResult {
    Ticked { events: Vec<Event> },
    Paused,
    NoEvents,
}

impl SimLoop {
    pub fn new(seed: u64, dt: f64) -> Self {
        Self {
            current_time: SimTime::zero(),
            accumulator: 0.0,
            dt,
            speed_multiplier: 1.0,
            paused: false,
            events: DeterministicMap::new(),
            next_seq: 0,
            rng: ChaosStream::from_global_seed(seed),
        }
    }
    
    pub fn add_input(&mut self, input: InputEvent) {
        let event = Event::input(self.next_seq, self.current_time, input);
        self.events.insert(self.next_seq, event);
        self.next_seq += 1;
    }
    
    pub fn accumulate_time(&mut self, delta_time: f64) {
        if !self.paused {
            self.accumulator += delta_time * self.speed_multiplier;
        }
    }
    
    pub fn tick(&mut self) -> TickResult {
        if self.paused {
            // Still process events at current time even when paused
            let events_at_tick: Vec<Event> = self.events
                .values()
                .filter(|e| e.tick == self.current_time)
                .cloned()
                .collect();
            
            if !events_at_tick.is_empty() {
                for event in &events_at_tick {
                    self.process_event(event);
                }
                return TickResult::Ticked { events: events_at_tick };
            }
            
            return TickResult::Paused;
        }
        
        if self.accumulator < self.dt {
            // Still process events at current time even if not enough time for tick
            let events_at_tick: Vec<Event> = self.events
                .values()
                .filter(|e| e.tick == self.current_time)
                .cloned()
                .collect();
            
            if !events_at_tick.is_empty() {
                for event in &events_at_tick {
                    self.process_event(event);
                }
                return TickResult::Ticked { events: events_at_tick };
            }
            
            return TickResult::NoEvents;
        }
        
        let mut tick_events = Vec::new();
        
        // Execute tick event
        let tick_event = Event::new(self.next_seq, self.current_time, EventKind::Tick);
        tick_events.push(tick_event.clone());
        self.events.insert(self.next_seq, tick_event);
        self.next_seq += 1;
        
        // Process all events at current time
        let events_at_tick: Vec<Event> = self.events
            .values()
            .filter(|e| e.tick == self.current_time)
            .cloned()
            .collect();
        
        for event in &events_at_tick {
            self.process_event(event);
        }
        
        tick_events.extend(events_at_tick);
        
        // Advance time
        self.current_time.tick();
        self.accumulator -= self.dt;
        
        TickResult::Ticked { events: tick_events }
    }
    
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
    
    pub fn set_speed(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier as f64;
    }
    
    pub fn current_time(&self) -> SimTime {
        self.current_time
    }
    
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    
    /// Get next random value for testing purposes
    pub fn next_test_random(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    fn process_event(&mut self, event: &Event) {
        match &event.kind {
            EventKind::Input(input) => {
                self.process_input(input);
            }
            EventKind::Tick => {
                // Process deterministic tick logic
                let _random_value = self.rng.next_f32();
            }
            EventKind::AgentSpawn { agent_id } => {
                // Handle agent spawning
                tracing::debug!("Spawning agent: {}", agent_id);
            }
            EventKind::AgentMove { agent_id, x, y, z } => {
                // Handle agent movement
                tracing::debug!("Agent {} moved to ({}, {}, {})", agent_id, x, y, z);
            }
            EventKind::Chat { agent_id, message } => {
                // Handle chat
                tracing::debug!("Chat from {}: {}", agent_id, message);
            }
        }
    }
    
    fn process_input(&mut self, input: &InputEvent) {
        match input {
            InputEvent::AdminCommand { command } => {
                self.process_admin_command(command);
            }
            InputEvent::Chat { user_id, message } => {
                tracing::debug!("Chat from {}: {}", user_id, message);
            }
            InputEvent::Move { user_id, direction } => {
                tracing::debug!("Move from {}: {:?}", user_id, direction);
            }
        }
    }
    
    fn process_admin_command(&mut self, command: &crate::sim::events::AdminCommand) {
        use crate::sim::events::AdminCommand;
        
        match command {
            AdminCommand::Pause => self.paused = true,
            AdminCommand::Resume => self.paused = false,
            AdminCommand::Step => {
                // Force one tick
                self.accumulator = self.dt;
            }
            AdminCommand::SetSpeed { multiplier } => {
                self.speed_multiplier = *multiplier as f64;
            }
            AdminCommand::SpawnAgent { agent_id } => {
                let spawn_event = Event::new(
                    self.next_seq,
                    self.current_time,
                    EventKind::AgentSpawn { agent_id: agent_id.clone() }
                );
                self.events.insert(self.next_seq, spawn_event);
                self.next_seq += 1;
            }
        }
    }
}

