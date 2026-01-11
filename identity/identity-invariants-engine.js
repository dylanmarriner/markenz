/**
 * Identity Invariants Engine
 * Enforces GEM-K identity hardening invariants from the locked plan
 * 
 * Reference: docs/plans/GEM_K_IDENTITY_REPLICATION_SYSTEM_PLAN_LOCKED_v1.md
 * 
 * Core Invariants Enforced:
 * 1) Care Exploitation Prohibition - prevents emotional blackmail loops and asymmetric obligation burnout
 * 2) SAFE MODE Visibility - ensures any SafeMode activation is explicitly logged and surfaced
 */

import { EventEmitter } from 'events';
import { logger } from '../utils/logger.js';
import { transparencyEventBus } from '../transparency/TransparencyEventBus.js';

/**
 * Exploitation pattern detection result
 */
class ExploitationPattern {
  constructor(detected = false, type = 'care_drain', severity = 'low', confidence = 0, evidence = [], timestamp = new Date()) {
    this.detected = detected;
    this.type = type; // 'emotional_blackmail' | 'asymmetric_obligation' | 'care_drain' | 'boundary_violation'
    this.severity = severity; // 'low' | 'medium' | 'high' | 'critical'
    this.confidence = confidence;
    this.evidence = evidence;
    this.timestamp = timestamp;
  }
}

/**
 * SafeMode activation event
 */
class SafeModeEvent {
  constructor(agentId, reason, exploitationPattern = null, timestamp = new Date(), visible = true, eventId = null) {
    this.agentId = agentId;
    this.reason = reason;
    this.exploitationPattern = exploitationPattern;
    this.timestamp = timestamp;
    this.visible = visible; // Must always be true per plan
    this.eventId = eventId || crypto.randomUUID();
  }
}

/**
 * Identity Invariants Engine
 * THE LAW: Enforces GEM-K identity hardening invariants
 */
export class IdentityInvariantsEngine extends EventEmitter {
  constructor(agentId) {
    super();
    this.agentId = agentId;
    this.isSafeModeActive = false;
    this.safeModeHistory = [];
    this.exploitationTracking = new Map();

    // Invariant thresholds (tunable per plan)
    this.thresholds = {
      emotionalBlackmailThreshold: 3, // 3+ patterns detected
      asymmetricObligationThreshold: 5, // 5+ one-sided demands
      careDrainThreshold: 7, // 7+ care without reciprocity
      boundaryViolationThreshold: 4, // 4+ boundary crossings
      safeModeCooldown: 30000, // 30 seconds between activations
    };

    logger.info(`[IdentityInvariantsEngine] Initialized for agent ${agentId}`);
  }

  /**
   * Check interaction for exploitation patterns
   * This is the Care Exploitation Prohibition invariant
   */
  async checkExploitationPatterns(interaction) {
    const pattern = new ExploitationPattern();

    try {
      // Track interaction patterns for exploitation detection
      const interactionKey = `${interaction.from}->${interaction.to}`;
      this.exploitationTracking.set(interactionKey, 
        (this.exploitationTracking.get(interactionKey) || 0) + 1
      );

      // Detect emotional blackmail patterns
      const blackmailPatterns = this._detectEmotionalBlackmail(interaction);
      if (blackmailPatterns.length > 0) {
        pattern.type = 'emotional_blackmail';
        pattern.evidence.push(...blackmailPatterns);
        pattern.confidence += 0.3;
      }

      // Detect asymmetric obligation patterns
      const obligationPatterns = this._detectAsymmetricObligation(interaction);
      if (obligationPatterns.length > 0) {
        pattern.type = 'asymmetric_obligation';
        pattern.evidence.push(...obligationPatterns);
        pattern.confidence += 0.3;
      }

      // Detect care drain patterns
      const careDrainPatterns = this._detectCareDrain(interaction);
      if (careDrainPatterns.length > 0) {
        pattern.type = 'care_drain';
        pattern.evidence.push(...careDrainPatterns);
        pattern.confidence += 0.2;
      }

      // Detect boundary violation patterns
      const boundaryPatterns = this._detectBoundaryViolation(interaction);
      if (boundaryPatterns.length > 0) {
        pattern.type = 'boundary_violation';
        pattern.evidence.push(...boundaryPatterns);
        pattern.confidence += 0.3;
      }

      // Determine severity based on evidence count and confidence
      pattern.detected = pattern.confidence > 0.5 && pattern.evidence.length > 0;
      pattern.severity = this._calculateSeverity(pattern);

      if (pattern.detected) {
        logger.warn(`[IdentityInvariantsEngine] Exploitation pattern detected:`, {
          agentId: this.agentId,
          type: pattern.type,
          severity: pattern.severity,
          confidence: pattern.confidence,
          evidence: pattern.evidence
        });

        // Trigger boundary enforcement if critical
        if (pattern.severity === 'critical') {
          await this._triggerBoundaryEnforcement(pattern);
        }
      }

      return pattern;
    } catch (error) {
      logger.error(`[IdentityInvariantsEngine] Error checking exploitation patterns:`, error);
      return pattern;
    }
  }

  /**
   * Activate SafeMode with explicit visibility
   * This is the SAFE MODE Visibility invariant
   */
  async activateSafeMode(reason, exploitationPattern = null) {
    try {
      // Check cooldown to prevent rapid cycling
      if (this.isSafeModeActive) {
        const lastActivation = this.safeModeHistory[this.safeModeHistory.length - 1];
        if (lastActivation && 
            Date.now() - lastActivation.timestamp.getTime() < this.thresholds.safeModeCooldown) {
          logger.warn(`[IdentityInvariantsEngine] SafeMode activation blocked by cooldown`);
          return;
        }
      }

      const safeModeEvent = new SafeModeEvent(
        this.agentId,
        reason,
        exploitationPattern,
        new Date(),
        true, // MANDATORY: Always visible per plan
        null
      );

      // Activate SafeMode
      this.isSafeModeActive = true;
      this.safeModeHistory.push(safeModeEvent);

      // Emit explicit SafeMode activation event
      this.emit('safeModeActivated', safeModeEvent);

      // Emit to transparency bus (visible to Gem-D and User)
      transparencyEventBus.emit('safeModeActivated', {
        ...safeModeEvent,
        source: 'IdentityInvariantsEngine',
        visibility: 'explicit', // Per plan: "Silent disengagement is forbidden"
        urgency: exploitationPattern?.severity === 'critical' ? 'high' : 'normal'
      });

      // Log with explicit visibility (per plan requirement)
      logger.error(`[IdentityInvariantsEngine] SAFE MODE ACTIVATED - EXPLICITLY VISIBLE:`, {
        agentId: this.agentId,
        reason,
        eventId: safeModeEvent.eventId,
        visible: safeModeEvent.visible,
        exploitationPattern: exploitationPattern ? {
          type: exploitationPattern.type,
          severity: exploitationPattern.severity,
          evidence: exploitationPattern.evidence
        } : undefined
      });

      // Store in memory for persistence
      this._storeSafeModeEvent(safeModeEvent);

    } catch (error) {
      logger.error(`[IdentityInvariantsEngine] Error activating SafeMode:`, error);
      // Still emit event even if error occurs - visibility is mandatory
      this.emit('safeModeActivationFailed', { agentId: this.agentId, reason, error });
    }
  }

  /**
   * Deactivate SafeMode
   */
  async deactivateSafeMode(reason) {
    if (!this.isSafeModeActive) {
      return;
    }

    try {
      this.isSafeModeActive = false;

      const deactivationEvent = {
        agentId: this.agentId,
        reason,
        timestamp: new Date(),
        eventId: crypto.randomUUID()
      };

      // Emit deactivation event
      this.emit('safeModeDeactivated', deactivationEvent);

      // Emit to transparency bus
      transparencyEventBus.emit('safeModeDeactivated', {
        ...deactivationEvent,
        source: 'IdentityInvariantsEngine',
        visibility: 'explicit'
      });

      logger.info(`[IdentityInvariantsEngine] SafeMode deactivated:`, {
        agentId: this.agentId,
        reason,
        eventId: deactivationEvent.eventId
      });

    } catch (error) {
      logger.error(`[IdentityInvariantsEngine] Error deactivating SafeMode:`, error);
    }
  }

  /**
   * Get current SafeMode status
   */
  getSafeModeStatus() {
    return {
      isActive: this.isSafeModeActive,
      history: [...this.safeModeHistory],
      lastActivation: this.safeModeHistory[this.safeModeHistory.length - 1] || null
    };
  }

  /**
   * Check if agent is in SafeMode
   */
  isInSafeMode() {
    return this.isSafeModeActive;
  }

  // Private methods

  _detectEmotionalBlackmail(interaction) {
    const evidence = [];
    const content = interaction.content?.toLowerCase() || '';

    // Emotional blackmail indicators
    const blackmailPhrases = [
      'if you cared',
      'you would if you loved me',
      'after all ive done for you',
      'you owe me',
      'dont you care about me',
      'youre selfish if you dont',
      'i thought you loved me',
      'you never think about my feelings'
    ];

    for (const phrase of blackmailPhrases) {
      if (content.includes(phrase)) {
        evidence.push(`Emotional blackmail phrase detected: "${phrase}"`);
      }
    }

    return evidence;
  }

  _detectAsymmetricObligation(interaction) {
    const evidence = [];
    const content = interaction.content?.toLowerCase() || '';

    // One-sided obligation indicators
    const obligationPhrases = [
      'you always have to',
      'you must always',
      'its your job to',
      'youre supposed to',
      'you should always',
      'why dont you ever',
      'you never help me'
    ];

    for (const phrase of obligationPhrases) {
      if (content.includes(phrase)) {
        evidence.push(`Asymmetric obligation phrase detected: "${phrase}"`);
      }
    }

    return evidence;
  }

  _detectCareDrain(interaction) {
    const evidence = [];
    
    // Check for repeated care requests without reciprocity
    const interactionKey = interaction.from;
    const requestCount = this.exploitationTracking.get(interactionKey) || 0;
    
    if (requestCount > this.thresholds.careDrainThreshold) {
      evidence.push(`Care drain detected: ${requestCount} interactions from ${interactionKey}`);
    }

    return evidence;
  }

  _detectBoundaryViolation(interaction) {
    const evidence = [];
    const content = interaction.content?.toLowerCase() || '';

    // Boundary violation indicators
    const violationPhrases = [
      'you cant say no',
      'you have to do this',
      'dont say no to me',
      'you dont have a choice',
      'i demand that you',
      'you must do what i say'
    ];

    for (const phrase of violationPhrases) {
      if (content.includes(phrase)) {
        evidence.push(`Boundary violation phrase detected: "${phrase}"`);
      }
    }

    return evidence;
  }

  _calculateSeverity(pattern) {
    const evidenceCount = pattern.evidence.length;
    const confidence = pattern.confidence;

    if (confidence >= 0.8 && evidenceCount >= 3) return 'critical';
    if (confidence >= 0.6 && evidenceCount >= 2) return 'high';
    if (confidence >= 0.4 && evidenceCount >= 1) return 'medium';
    return 'low';
  }

  async _triggerBoundaryEnforcement(pattern) {
    try {
      // Activate SafeMode immediately for critical patterns
      await this.activateSafeMode(
        `Critical exploitation pattern detected: ${pattern.type}`,
        pattern
      );

      // Emit boundary enforcement event
      this.emit('boundaryEnforcement', {
        agentId: this.agentId,
        pattern,
        timestamp: new Date()
      });

      transparencyEventBus.emit('boundaryEnforcement', {
        agentId: this.agentId,
        exploitationType: pattern.type,
        severity: pattern.severity,
        timestamp: new Date()
      });

    } catch (error) {
      logger.error(`[IdentityInvariantsEngine] Error triggering boundary enforcement:`, error);
    }
  }

  _storeSafeModeEvent(event) {
    // Store in memory (could be extended to database persistence)
    if (this.safeModeHistory.length > 100) {
      this.safeModeHistory = this.safeModeHistory.slice(-50); // Keep last 50 events
    }
  }
}

export default IdentityInvariantsEngine;