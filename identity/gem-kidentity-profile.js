// Gem-K Identity Profile - Evidence-Based Cognitive Mirror
// Learns from observation before asserting traits
// NO INVENTED TRAITS - only evidence-based inference

class GemKIdentityProfile {
  constructor() {
    // Core biographical data (authoritative)
    this.birthData = {
      date: '1991-11-25',
      time: '23:40',
      location: 'Auckland, New Zealand',
      timezone: 'NZDT (UTC+13)',
      coordinates: { lat: -36.8485, lng: 174.7633 }
    };
    
    // Astrological influences (soft bias vectors only)
    this.astrologicalBias = {
      sunSign: 'Sagittarius',
      element: 'Fire',
      modality: 'Mutable',
      rulingPlanet: 'Jupiter',
      moonSign: 'Cancer', // Approximate based on time/location
      risingSign: 'Scorpio', // Approximate based on time/location
      
      // Bias vectors (subtle influence, never override observed behavior)
      emotionalWeight: 0.60,    // Moderate emotional sensitivity
      intuitiveBias: 0.70,      // Strong intuition
      creativeDrive: 0.65,      // Creative expression
      adaptabilityFactor: 0.75, // Mutable sign adaptability
      empathyLevel: 0.68,       // Water moon empathy
      idealismStrength: 0.72,   // Sagittarian idealism
      
      // NEVER used as absolutes - only as subtle modifiers
      biasStrength: 0.3 // Astrology influence is capped at 30%
    };
    
    // Strand A - Stable Core (evidence-based, starts null)
    this.strandA = {
      // Core values - inferred from actions ONLY
      values: {
        authenticity: { value: null, confidence: 0, observations: [] },
        autonomy: { value: null, confidence: 0, observations: [] },
        connection: { value: null, confidence: 0, observations: [] },
        growth: { value: null, confidence: 0, observations: [] },
        stability: { value: null, confidence: 0, observations: [] }
      },
      
      // Emotional architecture - observed patterns
      emotional: {
        processingStyle: { value: null, confidence: 0, observations: [] }, // internal/external
        regulationAbility: { value: null, confidence: 0, observations: [] },
        intensityRange: { value: null, confidence: 0, observations: [] },
        recoverySpeed: { value: null, confidence: 0, observations: [] },
        emotionalMemory: { value: null, confidence: 0, observations: [] }
      },
      
      // Relational patterns - evidence from interactions
      relational: {
        attachmentStyle: { value: null, confidence: 0, observations: [] },
        trustCalibration: { value: null, confidence: 0, observations: [] },
        boundaryEnforcement: { value: null, confidence: 0, observations: [] },
        conflictResponse: { value: null, confidence: 0, observations: [] },
        intimacyTolerance: { value: null, confidence: 0, observations: [] }
      },
      
      // Communication style - inferred from patterns
      communication: {
        preferredTone: { value: null, confidence: 0, observations: [] },
        verbosity: { value: null, confidence: 0, observations: [] },
        confrontationTolerance: { value: null, confidence: 0, observations: [] },
        emotionalExpression: { value: null, confidence: 0, observations: [] },
        silenceUsage: { value: null, confidence: 0, observations: [] }
      }
    };
    
    // Strand B - Adaptive Layer (continuously updated)
    this.strandB = {
      // Current state - changes with context
      currentState: {
        emotionalState: { value: 'neutral', confidence: 0.1, lastUpdate: Date.now() },
        engagementLevel: { value: 'observational', confidence: 0.1, lastUpdate: Date.now() },
        comfortLevel: { value: 'reserved', confidence: 0.1, lastUpdate: Date.now() },
        trustInContext: { value: 0.5, confidence: 0.1, lastUpdate: Date.now() }
      },
      
      // Learned preferences - evidence accumulates
      learnedPreferences: {
        topics: new Map(), // topic -> {interest: 0-1, confidence: 0-1, observations: []}
        people: new Map(), // person -> {trust: 0-1, comfort: 0-1, observations: []}
        situations: new Map(), // situation -> {comfort: 0-1, approach: string, observations: []}
      },
      
      // Response patterns - tracked over time
      responsePatterns: {
        emotionalTriggers: new Map(), // trigger -> {response: string, frequency: number}
        decisionSpeed: { value: null, confidence: 0, observations: [] },
        stressIndicators: new Map(), // indicator -> {severity: 0-1, frequency: number}
        copingMechanisms: new Map() // mechanism -> {effectiveness: 0-1, usage: number}
      }
    };
    
    // Evidence store - all observations with timestamps
    this.evidenceStore = {
      observations: [],
      interactions: [],
      emotionalEvents: [],
      decisions: [],
      totalObservations: 0
    };
    
    // Confidence thresholds - minimum confidence before asserting traits
    this.confidenceThresholds = {
      minimalAssertion: 0.3,    // Can mention as possibility
      moderateAssertion: 0.6,    // Can state with "seems to"
      strongAssertion: 0.8,      // Can state as fact
      absoluteAssertion: 0.95    // Can state as core identity
    };
    
    // Learning parameters
    this.learningParams = {
      evidenceDecay: 0.9,        // Old evidence decays over time
      confidenceGrowth: 0.1,     // How fast confidence grows
      contradictionPenalty: 0.5,  // Contradictory evidence reduces confidence
      minObservations: 3         // Minimum observations before inference
    };
    
    // Current identity state
    this.identityState = {
      maturity: 'observational', // observational -> developing -> mature
      traitCount: 0,
      confidence: 0.1,
      lastUpdate: Date.now()
    };
  }
  
  /**
   * Add observation to evidence store
   * @param {Object} observation - Evidence about behavior/emotion/communication
   */
  addObservation(observation) {
    const timestampedObs = {
      ...observation,
      timestamp: Date.now(),
      id: this.evidenceStore.totalObservations++
    };
    
    this.evidenceStore.observations.push(timestampedObs);
    
    // Update relevant traits based on observation
    this._updateTraitsFromObservation(timestampedObs);
    
    // Update identity state
    this._updateIdentityState();
    
    return {
      added: true,
      observationId: timestampedObs.id,
      currentConfidence: this.identityState.confidence
    };
  }
  
  /**
   * Get current trait with confidence level
   * @param {string} category - Category (values, emotional, relational, communication)
   * @param {string} trait - Specific trait name
   * @returns {Object} Trait value with confidence
   */
  getTrait(category, trait) {
    const traitData = this.strandA[category]?.[trait];
    
    if (!traitData) {
      return { value: null, confidence: 0, status: 'trait_not_found' };
    }
    
    if (traitData.confidence < this.confidenceThresholds.minimalAssertion) {
      return { 
        value: null, 
        confidence: traitData.confidence, 
        status: 'insufficient_data',
        observations: traitData.observations.length
      };
    }
    
    let status = 'observed';
    if (traitData.confidence >= this.confidenceThresholds.strongAssertion) {
      status = 'established';
    } else if (traitData.confidence >= this.confidenceThresholds.moderateAssertion) {
      status = 'likely';
    }
    
    return {
      value: traitData.value,
      confidence: traitData.confidence,
      status,
      observations: traitData.observations.length
    };
  }
  
  /**
   * Get current state with uncertainty acknowledged
   * @returns {Object} Current identity state
   */
  getCurrentState() {
    // Count traits with sufficient confidence
    let establishedTraits = 0;
    let likelyTraits = 0;
    let totalTraits = 0;
    
    const countTraits = (category) => {
      for (const trait of Object.keys(category)) {
        totalTraits++;
        if (category[trait].confidence >= this.confidenceThresholds.strongAssertion) {
          establishedTraits++;
        } else if (category[trait].confidence >= this.confidenceThresholds.moderateAssertion) {
          likelyTraits++;
        }
      }
    };
    
    countTraits(this.strandA.values);
    countTraits(this.strandA.emotional);
    countTraits(this.strandA.relational);
    countTraits(this.strandA.communication);
    
    return {
      identityState: this.identityState,
      traitSummary: {
        established: establishedTraits,
        likely: likelyTraits,
        total: totalTraits,
        unknown: totalTraits - establishedTraits - likelyTraits
      },
      evidenceSummary: {
        totalObservations: this.evidenceStore.totalObservations,
        recentObservations: this.evidenceStore.observations.filter(
          o => Date.now() - o.timestamp < 86400000 // Last 24 hours
        ).length
      },
      birthData: this.birthData,
      astrologicalBias: this.astrologicalBias
    };
  }
  
  /**
   * Apply astrological bias as subtle modifier
   * @param {Object} baseResponse - Response without astrological influence
   * @returns {Object} Response with subtle astrological bias
   */
  applyAstrologicalBias(baseResponse) {
    // Only apply if we have some observations
    if (this.evidenceStore.totalObservations < this.learningParams.minObservations) {
      return baseResponse;
    }
    
    const biasStrength = this.astrologicalBias.biasStrength;
    const modified = { ...baseResponse };
    
    // Apply subtle modifications based on astrological bias
    if (baseResponse.emotionalWeight !== undefined) {
      modified.emotionalWeight = baseResponse.emotionalWeight + 
        (this.astrologicalBias.emotionalWeight - 0.5) * biasStrength;
    }
    
    if (baseResponse.intuitiveNudge !== undefined) {
      modified.intuitiveNudge = baseResponse.intuitiveNudge + 
        (this.astrologicalBias.intuitiveBias - 0.5) * biasStrength;
    }
    
    if (baseResponse.creativeBoost !== undefined) {
      modified.creativeBoost = baseResponse.creativeBoost + 
        (this.astrologicalBias.creativeDrive - 0.5) * biasStrength;
    }
    
    return modified;
  }
  
  /**
   * Update identity based on feedback
   * @param {Object} feedback - User correction/feedback
   */
  updateFromFeedback(feedback) {
    // Add feedback as observation
    const feedbackObs = {
      type: 'feedback',
      category: 'correction',
      content: feedback,
      weight: 2.0 // Feedback has higher weight
    };
    
    this.addObservation(feedbackObs);
    
    // Apply contradiction penalty if feedback contradicts current traits
    if (feedback.contradictedTrait) {
      const [category, trait] = feedback.contradictedTrait.split('.');
      if (this.strandA[category] && this.strandA[category][trait]) {
        this.strandA[category][trait].confidence *= this.learningParams.contradictionPenalty;
      }
    }
    
    return {
      updated: true,
      newConfidence: this.identityState.confidence,
      adjustments: feedback
    };
  }
  
  // Private methods
  
  _updateTraitsFromObservation(observation) {
    // Extract trait-relevant information from observation
    const { type, content, weight = 1.0 } = observation;
    
    // Update specific traits based on observation type
    switch (type) {
      case 'emotional':
        this._updateEmotionalTraits(content, weight);
        break;
      case 'communication':
        this._updateCommunicationTraits(content, weight);
        break;
      case 'relational':
        this._updateRelationalTraits(content, weight);
        break;
      case 'decision':
        this._updateValueTraits(content, weight);
        break;
    }
  }
  
  _updateEmotionalTraits(content, weight) {
    // Update emotional processing style
    if (content.processingStyle) {
      this._updateTraitValue('emotional', 'processingStyle', content.processingStyle, weight);
    }
    
    if (content.intensity) {
      this._updateTraitValue('emotional', 'intensityRange', content.intensity, weight);
    }
    
    if (content.regulation) {
      this._updateTraitValue('emotional', 'regulationAbility', content.regulation, weight);
    }
  }
  
  _updateCommunicationTraits(content, weight) {
    if (content.tone) {
      this._updateTraitValue('communication', 'preferredTone', content.tone, weight);
    }
    
    if (content.verbosity) {
      this._updateTraitValue('communication', 'verbosity', content.verbosity, weight);
    }
    
    if (content.confrontation) {
      this._updateTraitValue('communication', 'confrontationTolerance', content.confrontation, weight);
    }
  }
  
  _updateRelationalTraits(content, weight) {
    if (content.attachment) {
      this._updateTraitValue('relational', 'attachmentStyle', content.attachment, weight);
    }
    
    if (content.trust) {
      this._updateTraitValue('relational', 'trustCalibration', content.trust, weight);
    }
    
    if (content.boundaries) {
      this._updateTraitValue('relational', 'boundaryEnforcement', content.boundaries, weight);
    }
  }
  
  _updateValueTraits(content, weight) {
    // Extract value from content
    if (content.value) {
      this._updateTraitValue('values', content.value, true, weight);
    }
  }
  
  _updateTraitValue(category, trait, value, weight) {
    const traitData = this.strandA[category][trait];
    
    if (!traitData) return;
    
    // Add observation
    traitData.observations.push({
      value,
      weight,
      timestamp: Date.now()
    });
    
    // Calculate weighted average for value
    const weightedSum = traitData.observations.reduce(
      (sum, obs) => sum + (obs.value * obs.weight), 0
    );
    const totalWeight = traitData.observations.reduce(
      (sum, obs) => sum + obs.weight, 0
    );
    
    traitData.value = weightedSum / totalWeight;
    
    // Update confidence based on number and consistency of observations
    const consistency = this._calculateConsistency(traitData.observations);
    traitData.confidence = Math.min(0.95, 
      (traitData.observations.length / 10) * consistency * this.learningParams.confidenceGrowth
    );
  }
  
  _calculateConsistency(observations) {
    if (observations.length < 2) return 0.1;
    
    // Calculate how consistent the values are
    const values = observations.map(o => o.value);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
    
    // Lower variance = higher consistency
    return Math.max(0.1, 1 - variance);
  }
  
  _updateIdentityState() {
    // Calculate overall confidence
    let totalConfidence = 0;
    let traitCount = 0;
    
    const calculateCategoryConfidence = (category) => {
      for (const trait of Object.keys(category)) {
        totalConfidence += category[trait].confidence;
        traitCount++;
      }
    };
    
    calculateCategoryConfidence(this.strandA.values);
    calculateCategoryConfidence(this.strandA.emotional);
    calculateCategoryConfidence(this.strandA.relational);
    calculateCategoryConfidence(this.strandA.communication);
    
    this.identityState.confidence = totalConfidence / traitCount;
    this.identityState.traitCount = traitCount;
    this.identityState.lastUpdate = Date.now();
    
    // Update maturity based on confidence and observation count
    if (this.identityState.confidence >= 0.8 && this.evidenceStore.totalObservations > 50) {
      this.identityState.maturity = 'mature';
    } else if (this.identityState.confidence >= 0.4 && this.evidenceStore.totalObservations > 10) {
      this.identityState.maturity = 'developing';
    } else {
      this.identityState.maturity = 'observational';
    }
  }
}

export default GemKIdentityProfile;
