
/**
 * Test script for Phase 0a - World Clock & Determinism
 * 
 * Verifies:
 * - Tick count increases deterministically
 * - simTime = tickCount × dt
 * - Running twice produces identical logs
 * - Headless run for 10s without crash
 */

import { World } from './World';

function runDeterministicTest(): void {
  console.log('=== PHASE 0a DETERMINISM TEST ===');
  
  // Test 1: Basic tick behavior
  console.log('\n--- Test 1: Basic Tick Behavior ---');
  const world1 = new World();
  
  const initialState = world1.getState();
  console.log('Initial state:', initialState);
  
  // Run 5 steps
  for (let i = 0; i < 5; i++) {
    world1.step();
    const state = world1.getState();
    console.log(`Step ${i + 1}: tick=${state.clock.tickCount}, simTime=${state.clock.simTime}ms`);
  }
  
  // Verify simTime = tickCount × dt
  const finalState = world1.getState();
  const expectedSimTime = finalState.clock.tickCount * finalState.clock.dt;
  console.log(`Expected simTime: ${expectedSimTime}ms`);
  console.log(`Actual simTime: ${finalState.clock.simTime}ms`);
  console.log(`Match: ${expectedSimTime === finalState.clock.simTime}`);
  
  // Test 2: Deterministic run (same results)
  console.log('\n--- Test 2: Deterministic Run Test ---');
  const world2 = new World();
  const world3 = new World();
  
  // Run both worlds for 10 steps
  const states2: any[] = [];
  const states3: any[] = [];
  
  for (let i = 0; i < 10; i++) {
    world2.step();
    world3.step();
    
    const state2 = world2.getState();
    const state3 = world3.getState();
    
    states2.push({ tick: state2.clock.tickCount, time: state2.clock.simTime });
    states3.push({ tick: state3.clock.tickCount, time: state3.clock.simTime });
  }
  
  console.log('World 2 states:', states2);
  console.log('World 3 states:', states3);
  console.log('Identical timing results:', JSON.stringify(states2) === JSON.stringify(states3));
  
  // Test 3: Headless run for 10 seconds
  console.log('\n--- Test 3: Headless 10s Run ---');
  const world4 = new World();
  
  const startTime = Date.now();
  world4.runHeadless(10000); // 10 seconds
  const endTime = Date.now();
  
  const finalState4 = world4.getState();
  console.log(`Headless run completed in ${endTime - startTime}ms`);
  console.log(`Final tick count: ${finalState4.clock.tickCount}`);
  console.log(`Final sim time: ${finalState4.clock.simTime}ms`);
  console.log(`Expected ticks: ${Math.ceil(10000 / 100)}`); // 100ms dt
  
  // Test 4: Clock state persistence
  console.log('\n--- Test 4: State Persistence ---');
  const world5 = new World();
  
  // Run some steps
  for (let i = 0; i < 7; i++) {
    world5.step();
  }
  
  const savedState = world5.getState();
  console.log('Saved state:', savedState);
  
  // Create new world and verify it starts from zero
  const world6 = new World();
  const freshState = world6.getState();
  console.log('Fresh world state:', freshState);
  console.log('Fresh world starts at zero:', freshState.clock.tickCount === 0);
  
  console.log('\n=== PHASE 0a TEST COMPLETE ===');
}

// Run the test
if (import.meta.url === `file://${process.argv[1]}`) {
  runDeterministicTest();
}

export { runDeterministicTest };
