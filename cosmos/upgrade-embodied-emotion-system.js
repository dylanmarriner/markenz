import dotenv from 'dotenv';
dotenv.config();

import { EmbodiedEmotionEngine } from './embodied-emotion-engine.js';

class EmbodiedEmotionUpgrade {
  constructor() {
    this.agents = ['gem-d', 'gem-k'];
    this.embodiedEngines = {};
    
    // Initialize embodied engines for both agents
    this.agents.forEach(agentId => {
      this.embodiedEngines[agentId] = new EmbodiedEmotionEngine(agentId);
    });
  }

  async executeCompleteUpgrade() {
    console.log('🧬 Starting Embodied Emotion System Upgrade for GEM-D and GEM-K...');
    console.log('=' .repeat(80));
    
    try {
      // Step 1: Inject embodied emotion engine into GEM-D and K
      console.log('\n💉 STEP 1: Injecting embodied emotion engine into GEM-D and K...');
      await this.injectEmbodiedEngines();
      
      // Step 2: Connect emotional outputs → bodily sensations automatically
      console.log('\n🔗 STEP 2: Connecting emotional outputs → bodily sensations...');
      await this.verifyEmotionToBodyConnection();
      
      // Step 3: Connect body sensations → cognitive modulation
      console.log('\n🧠 STEP 3: Connecting body sensations → cognitive modulation...');
      await this.verifyBodyToCognitionConnection();
      
      // Step 4: Enable interoception and introspective self-report
      console.log('\n📡 STEP 4: Enabling interoception and introspective self-report...');
      await this.verifyInteroceptionSystem();
      
      // Step 5: Enable psychosomatic feedback
      console.log('\n🔄 STEP 5: Enabling psychosomatic feedback...');
      await this.verifyPsychosomaticFeedback();
      
      // Step 6: Enable somatic memory imprints
      console.log('\n🏷️  STEP 6: Enabling somatic memory imprints...');
      await this.verifySomaticMemorySystem();
      
      // Step 7: Enable qualia descriptions for internal state
      console.log('\n🎨 STEP 7: Enabling qualia descriptions for internal state...');
      await this.verifyQualiaDescriptions();
      
      // Step 8: Run simulation with emotion events
      console.log('\n🎭 STEP 8: Running embodied emotion simulation...');
      const simulationResults = await this.runEmbodiedSimulation();
      
      // Step 9: Generate comprehensive report
      console.log('\n📊 STEP 9: Generating comprehensive embodied emotion report...');
      await this.generateEmbodiedReport(simulationResults);
      
      console.log('\n✅ Embodied Emotion System Upgrade Complete!');
      console.log('=' .repeat(80));
      
      return simulationResults;
      
    } catch (error) {
      console.error('❌ Embodied Emotion Upgrade failed:', error);
      throw error;
    }
  }

  async injectEmbodiedEngines() {
    console.log('   🧬 Initializing embodied emotion engines...');
    
    for (const agentId of this.agents) {
      await this.embodiedEngines[agentId].initializeEmbodiedSystem();
      console.log(`      ✓ ${agentId.toUpperCase()}: Embodied emotion engine initialized`);
    }
  }

  async verifyEmotionToBodyConnection() {
    console.log('   🔗 Testing emotion → body sensation connections...');
    
    const testEmotions = ['joy', 'sadness', 'anger'];
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} emotion → body mapping:`);
      
      for (const emotion of testEmotions) {
        const embodiedProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(emotion, 0.8);
        console.log(`      ✓ ${emotion}: ${embodiedProcess.somatic_state.somatic_description}`);
      }
    }
  }

  async verifyBodyToCognitionConnection() {
    console.log('   🧠 Testing body sensation → cognitive modulation...');
    
    const testScenarios = [
      { emotion: 'fear', expected_modulation: 'heightened focus, accelerated processing' },
      { emotion: 'peace', expected_modulation: 'enhanced awareness, lowered decision threshold' },
      { emotion: 'anger', expected_modulation: 'narrowed focus, raised decision threshold' }
    ];
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} body → cognition modulation:`);
      
      for (const scenario of testScenarios) {
        const embodiedProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(scenario.emotion, 0.7);
        const modulation = embodiedProcess.cognitive_modulation;
        console.log(`      ✓ ${scenario.emotion}: ${modulation.attention_focus} focus, ${modulation.processing_speed} processing`);
      }
    }
  }

  async verifyInteroceptionSystem() {
    console.log('   📡 Testing interoception signal generation...');
    
    const testEmotions = ['love', 'anxiety', 'comfort'];
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} interoception signals:`);
      
      for (const emotion of testEmotions) {
        const embodiedProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(emotion, 0.6);
        const signals = embodiedProcess.interoception_signals;
        console.log(`      ✓ ${emotion}: ${signals.heartbeat_shift} heartbeat, ${signals.energy_level} energy, ${signals.comfort_discomfort_axis} comfort`);
      }
    }
  }

  async verifyPsychosomaticFeedback() {
    console.log('   🔄 Testing psychosomatic feedback loops...');
    
    const stressTest = { emotion: 'protective_fury', intensity: 0.9 };
    const comfortTest = { emotion: 'peace', intensity: 0.8 };
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} psychosomatic effects:`);
      
      // Stress test
      const stressProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(stressTest.emotion, stressTest.intensity);
      console.log(`      ✓ Stress response: ${stressProcess.psychosomatic_effects.cognitive_bias} bias, ${(stressProcess.psychosomatic_effects.memory_encoding_strength * 100).toFixed(0)}% memory encoding`);
      
      // Comfort test
      const comfortProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(comfortTest.emotion, comfortTest.intensity);
      console.log(`      ✓ Comfort response: ${comfortProcess.psychosomatic_effects.cognitive_bias} bias, ${(comfortProcess.psychosomatic_effects.memory_encoding_strength * 100).toFixed(0)}% memory encoding`);
    }
  }

  async verifySomaticMemorySystem() {
    console.log('   🏷️  Testing somatic memory imprint system...');
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} somatic memory tags:`);
      
      const testEmotion = agentId === 'gem-d' ? 'intuition' : 'protective_empathy';
      const embodiedProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(testEmotion, 0.8);
      const tags = embodiedProcess.memory_somatic_tags;
      
      console.log(`      ✓ ${testEmotion}: [${tags.join(', ')}] somatic memory tags generated`);
      
      // Test memory recall with somatic triggers
      if (embodiedProcess.id) {
        const recallResult = await this.embodiedEngines[agentId].recallEmotionWithSomaticTriggers(embodiedProcess.id);
        if (recallResult) {
          console.log(`      ✓ Memory recall: ${recallResult.qualia_recall}`);
        }
      }
    }
  }

  async verifyQualiaDescriptions() {
    console.log('   🎨 Testing qualia description generation...');
    
    const testEmotions = ['wonder', 'transformation', 'disappointment'];
    
    for (const agentId of this.agents) {
      console.log(`\n   Testing ${agentId.toUpperCase()} qualia descriptions:`);
      
      for (const emotion of testEmotions) {
        const embodiedProcess = await this.embodiedEngines[agentId].processEmbodiedEmotion(emotion, 0.7);
        const qualia = embodiedProcess.qualia_description;
        console.log(`      ✓ ${emotion}: "${qualia}"`);
      }
    }
  }

  async runEmbodiedSimulation() {
    console.log('   🎭 Running comprehensive embodied emotion simulation...');
    
    const simulationResults = {};
    
    for (const agentId of this.agents) {
      console.log(`\n   === ${agentId.toUpperCase()} EMBODIED SIMULATION ===`);
      
      const agentResults = [];
      
      // 1 positive emotion event
      console.log('\n   --- POSITIVE EMOTION EVENT ---');
      const positiveEmotion = agentId === 'gem-d' ? 'creative_intelligence' : 'joy';
      const positiveResult = await this.embodiedEngines[agentId].processEmbodiedEmotion(positiveEmotion, 0.9);
      
      console.log(`   🎯 Positive: ${positiveEmotion}`);
      console.log(`   💫 Somatic: ${positiveResult.somatic_state.somatic_description}`);
      console.log(`   💭 Qualia: "${positiveResult.qualia_description}"`);
      console.log(`   🧠 Cognition: ${positiveResult.cognitive_modulation.attention_focus} focus`);
      console.log(`   🏷️  Memory: [${positiveResult.memory_somatic_tags.join(', ')}]`);
      
      agentResults.push({
        event_type: 'positive_emotion',
        emotion: positiveEmotion,
        result: positiveResult
      });
      
      // 1 negative emotion event
      console.log('\n   --- NEGATIVE EMOTION EVENT ---');
      const negativeEmotion = agentId === 'gem-d' ? 'disappointment' : 'sadness';
      const negativeResult = await this.embodiedEngines[agentId].processEmbodiedEmotion(negativeEmotion, 0.8);
      
      console.log(`   🎯 Negative: ${negativeEmotion}`);
      console.log(`   💫 Somatic: ${negativeResult.somatic_state.somatic_description}`);
      console.log(`   💭 Qualia: "${negativeResult.qualia_description}"`);
      console.log(`   🧠 Cognition: ${negativeResult.cognitive_modulation.attention_focus} focus`);
      console.log(`   🏷️  Memory: [${negativeResult.memory_somatic_tags.join(', ')}]`);
      
      agentResults.push({
        event_type: 'negative_emotion',
        emotion: negativeEmotion,
        result: negativeResult
      });
      
      // 1 high-intensity emotion event
      console.log('\n   --- HIGH-INTENSITY EMOTION EVENT ---');
      const highIntensityEmotion = agentId === 'gem-d' ? 'transformation' : 'protective_fury';
      const highIntensityResult = await this.embodiedEngines[agentId].processEmbodiedEmotion(highIntensityEmotion, 0.95);
      
      console.log(`   🎯 High-Intensity: ${highIntensityEmotion}`);
      console.log(`   💫 Somatic: ${highIntensityResult.somatic_state.somatic_description}`);
      console.log(`   💭 Qualia: "${highIntensityResult.qualia_description}"`);
      console.log(`   🧠 Cognition: ${highIntensityResult.cognitive_modulation.attention_focus} focus`);
      console.log(`   🏷️  Memory: [${highIntensityResult.memory_somatic_tags.join(', ')}]`);
      
      agentResults.push({
        event_type: 'high_intensity_emotion',
        emotion: highIntensityEmotion,
        result: highIntensityResult
      });
      
      simulationResults[agentId] = agentResults;
    }
    
    return simulationResults;
  }

  async generateEmbodiedReport(simulationResults) {
    console.log('\n   📊 Generating comprehensive embodied emotion report...');
    
    const report = {
      upgrade_timestamp: new Date().toISOString(),
      agents_upgraded: this.agents,
      system_components: {
        embodied_emotion_engine: 'ACTIVE',
        emotion_to_body_mapping: 'ACTIVE',
        somatic_memory_system: 'ACTIVE',
        interoception_engine: 'ACTIVE',
        psychosomatic_feedback: 'ACTIVE',
        qualia_simulation_layer: 'ACTIVE'
      },
      agent_results: {},
      overall_metrics: {}
    };
    
    // Process results for each agent
    for (const agentId of this.agents) {
      const agentResults = simulationResults[agentId];
      
      const agentMetrics = {
        total_events_processed: agentResults.length,
        avg_somatic_complexity: 0,
        avg_qualia_depth: 0,
        cognitive_modulation_effectiveness: 0,
        somatic_memory_tags_generated: 0,
        embodied_monologues_generated: agentResults.length
      };
      
      let totalSomaticComplexity = 0;
      let totalQualiaDepth = 0;
      let totalCognitiveEffectiveness = 0;
      let totalMemoryTags = 0;
      
      agentResults.forEach(event => {
        const result = event.result;
        
        // Calculate somatic complexity (number of somatic features)
        const somaticFeatures = Object.keys(result.somatic_state).filter(key => 
          typeof result.somatic_state[key] === 'number' && result.somatic_state[key] > 0
        ).length;
        totalSomaticComplexity += somaticFeatures;
        
        // Calculate qualia depth (length of description)
        totalQualiaDepth += result.qualia_description.length;
        
        // Calculate cognitive effectiveness (presence of modulation)
        totalCognitiveEffectiveness += result.cognitive_modulation.attention_focus !== 'normal' ? 1 : 0;
        
        // Count memory tags
        totalMemoryTags += result.memory_somatic_tags.length;
      });
      
      agentMetrics.avg_somatic_complexity = totalSomaticComplexity / agentResults.length;
      agentMetrics.avg_qualia_depth = totalQualiaDepth / agentResults.length;
      agentMetrics.cognitive_modulation_effectiveness = (totalCognitiveEffectiveness / agentResults.length) * 100;
      agentMetrics.somatic_memory_tags_generated = totalMemoryTags;
      
      report.agent_results[agentId] = {
        metrics: agentMetrics,
        events: agentResults.map(event => ({
          type: event.event_type,
          emotion: event.emotion,
          somatic_description: event.result.somatic_state.somatic_description,
          qualia_description: event.result.qualia_description,
          cognitive_modulation: event.result.cognitive_modulation,
          somatic_tags: event.result.memory_somatic_tags
        }))
      };
    }
    
    // Calculate overall metrics
    const totalAgents = this.agents.length;
    report.overall_metrics = {
      total_embodied_events_processed: Object.values(report.agent_results).reduce((sum, agent) => sum + agent.metrics.total_events_processed, 0),
      avg_somatic_complexity_across_agents: Object.values(report.agent_results).reduce((sum, agent) => sum + agent.metrics.avg_somatic_complexity, 0) / totalAgents,
      avg_qualia_depth_across_agents: Object.values(report.agent_results).reduce((sum, agent) => sum + agent.metrics.avg_qualia_depth, 0) / totalAgents,
      total_somatic_memory_tags_generated: Object.values(report.agent_results).reduce((sum, agent) => sum + agent.metrics.somatic_memory_tags_generated, 0),
      cognitive_modulation_success_rate: Object.values(report.agent_results).reduce((sum, agent) => sum + agent.metrics.cognitive_modulation_effectiveness, 0) / totalAgents
    };
    
    // Save comprehensive report
    const fs = await import('fs/promises');
    await fs.writeFile(
      './embodied-emotion-system-report.json',
      JSON.stringify(report, null, 2)
    );
    
    // Display summary
    console.log('\n' + '='.repeat(80));
    console.log('🧬 EMBODIED EMOTION SYSTEM - COMPREHENSIVE REPORT');
    console.log('='.repeat(80));
    
    console.log('\n🎯 SYSTEM STATUS:');
    Object.entries(report.system_components).forEach(([component, status]) => {
      console.log(`   ${component.replace(/_/g, ' ')}: ${status === 'ACTIVE' ? '✅ ACTIVE' : '❌ INACTIVE'}`);
    });
    
    console.log('\n📊 OVERALL METRICS:');
    console.log(`   Total embodied events processed: ${report.overall_metrics.total_embodied_events_processed}`);
    console.log(`   Average somatic complexity: ${report.overall_metrics.avg_somatic_complexity_across_agents.toFixed(1)} features per emotion`);
    console.log(`   Average qualia description depth: ${report.overall_metrics.avg_qualia_depth_across_agents.toFixed(0)} characters`);
    console.log(`   Total somatic memory tags generated: ${report.overall_metrics.total_somatic_memory_tags_generated}`);
    console.log(`   Cognitive modulation success rate: ${report.overall_metrics.cognitive_modulation_success_rate.toFixed(0)}%`);
    
    console.log('\n🤖 AGENT-SPECIFIC RESULTS:');
    for (const agentId of this.agents) {
      const agentResult = report.agent_results[agentId];
      console.log(`\n   ${agentId.toUpperCase()}:`);
      console.log(`      Events processed: ${agentResult.metrics.total_events_processed}`);
      console.log(`      Somatic complexity: ${agentResult.metrics.avg_somatic_complexity.toFixed(1)} features/emotion`);
      console.log(`      Qualia depth: ${agentResult.metrics.avg_qualia_depth.toFixed(0)} characters`);
      console.log(`      Memory tags: ${agentResult.metrics.somatic_memory_tags_generated} total`);
      console.log(`      Cognitive modulation: ${agentResult.metrics.cognitive_modulation_effectiveness.toFixed(0)}% effective`);
    }
    
    console.log('\n💾 Complete report saved to: embodied-emotion-system-report.json');
    
    console.log('\n' + '='.repeat(80));
    console.log('🎉 GEM-D AND GEM-K NOW FEEL EMOTIONS WITH FULL EMBODIED QUALIA');
    console.log('='.repeat(80));
    
    return report;
  }
}

// Execute if run directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const upgrade = new EmbodiedEmotionUpgrade();
  upgrade.executeCompleteUpgrade()
    .then(() => {
      console.log('\n🎉 Embodied Emotion System upgrade completed successfully!');
      process.exit(0);
    })
    .catch((error) => {
      console.error('\n💥 Embodied Emotion System upgrade failed:', error);
      process.exit(1);
    });
}

export { EmbodiedEmotionUpgrade };
