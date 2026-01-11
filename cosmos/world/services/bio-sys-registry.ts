import { logger } from '../../../utils/logger';
import { BioSysManager } from '../../../biology/biosys/bio-sys-manager';
import { HumanBody, HumanBodyConfig } from '../../../biology/biosys/human-body';

/**
 * BioSys Registry
 * Central registry for BioSys instances - THE LAW of biological authority
 * 
 * HARD RULES:
 * - NO agent exists without BioSys
 * - NO fallback to parallel state
 * - NO mock biological values
 * - If BioSys fails, agent spawn FAILS
 */
export class BioSysRegistry {
  private static instance: BioSysRegistry;
  private bioSysManager: BioSysManager;
  private isInitialized = false;

  private constructor() {
    this.bioSysManager = BioSysManager.getInstance();
  }

  /**
   * Get singleton instance
   */
  static getInstance(): BioSysRegistry {
    if (!BioSysRegistry.instance) {
      BioSysRegistry.instance = new BioSysRegistry();
    }
    return BioSysRegistry.instance;
  }

  /**
   * Initialize BioSys for agent - FAIL CLOSED if initialization fails
   */
  async initializeBioSys(agentId: string, identityProfile: any, biologicalSex: string = 'neutral'): Promise<HumanBody> {
    try {
      logger.info(`[BioSysRegistry] Initializing BioSys for agent: ${agentId}`);

      // Create proper HumanBody with config
      const config: HumanBodyConfig = {
        biologicalSex: biologicalSex as 'male' | 'female' | 'neutral',
        age: 25, // Default age
        height: 170, // Default height in cm
        weight: 70, // Default weight in kg
        metabolism: 'normal'
      };

      const humanBody = new HumanBody(agentId, config);

      // Initialize with BioSysManager
      await this.bioSysManager.initializeAgent(
        agentId,
        identityProfile,
        biologicalSex
      );

      // Validate HumanBody is functional
      if (!humanBody || !humanBody.getIsAlive()) {
        throw new Error(`BioSys initialization failed for agent ${agentId} - non-functional HumanBody`);
      }

      logger.info(`[BioSysRegistry] BioSys initialized successfully for agent: ${agentId}`);
      return humanBody;

    } catch (error: any) {
      // FAIL CLOSED: No BioSys = no agent
      logger.error(`[BioSysRegistry] CRITICAL: BioSys initialization failed for agent ${agentId}:`, error);
      throw new Error(`BioSys initialization failed - agent spawn denied: ${error.message}`);
    }
  }

  /**
   * Get BioSys instance - THE ONLY way to access biological state
   */
  getBioSys(agentId: string): HumanBody | null {
    const humanBody = this.bioSysManager.getHumanBody(agentId);

    // FAIL CLOSED: If HumanBody missing, system is broken
    if (!humanBody) {
      logger.error(`[BioSysRegistry] CRITICAL: HumanBody missing for agent ${agentId} - system integrity violation`);
      return null;
    }

    return humanBody;
  }

  /**
   * Check if BioSys exists for agent
   */
  hasBioSys(agentId: string): boolean {
    return this.bioSysManager.getHumanBody(agentId) !== null;
  }

  /**
   * Remove BioSys for agent (cleanup)
   */
  async removeBioSys(agentId: string): Promise<void> {
    try {
      await this.bioSysManager.removeAgent(agentId);
      logger.info(`[BioSysRegistry] BioSys removed for agent: ${agentId}`);
    } catch (error) {
      logger.error(`[BioSysRegistry] Failed to remove BioSys for agent ${agentId}:`, error);
    }
  }

  /**
   * Get all agent IDs with BioSys
   */
  getAllAgentIds(): string[] {
    return this.bioSysManager.getAllAgentIds();
  }

  /**
   * Get registry status
   */
  getStatus(): {
    isInitialized: boolean;
    agentCount: number;
    agents: string[];
  } {
    const bioSysStatus = this.bioSysManager.getStatus();
    return {
      isInitialized: this.isInitialized,
      agentCount: bioSysStatus.agentCount,
      agents: bioSysStatus.agents
    };
  }

  /**
   * Initialize the registry
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) {
      return;
    }

    logger.info('[BioSysRegistry] Initializing BioSys Registry');
    await this.bioSysManager.start();
    this.isInitialized = true;
  }

  /**
   * Shutdown the registry
   */
  async shutdown(): Promise<void> {
    logger.info('[BioSysRegistry] Shutting down BioSys Registry');
    await this.bioSysManager.shutdown();
    this.isInitialized = false;
  }

  /**
   * Validate system integrity - all agents must have BioSys
   */
  validateSystemIntegrity(expectedAgentIds: string[]): boolean {
    const result = this.bioSysManager.validateSystemIntegrity();
    return result.valid || false;
  }
}
