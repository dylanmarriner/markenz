
/**
 * SHED
 * 
 * Workshop and tool management system
 * Crafting, repair, and skill development
 */

import { ChaosSys } from '../chaos/chaos-sys';

export interface Tool {
  id: string;
  type: 'cutting' | 'building' | 'repair' | 'cooking' | 'agriculture' | 'crafting';
  name: string;
  condition: number; // 0-1, wear and tear
  efficiency: number; // 0-1, performance modifier
  durability: number; // 0-1, resistance to damage
  skill_required: number; // 0-1, skill level needed
}

export interface CraftingRecipe {
  id: string;
  name: string;
  category: 'tool' | 'building' | 'food' | 'resource';
  inputs: Map<string, number>; // resource requirements
  outputs: Map<string, number>; // produced items
  time_required: number; // seconds
  skill_required: number; // 0-1
  tool_required?: string; // tool ID needed
}

export interface Workbench {
  id: string;
  type: 'carpentry' | 'metalwork' | 'cooking' | 'crafting';
  condition: number; // 0-1
  efficiency: number; // 0-1
  current_project?: string; // recipe ID
  progress: number; // 0-1
}

export class Shed {
  private tools: Map<string, Tool>;
  private recipes: Map<string, CraftingRecipe>;
  private workbenches: Map<string, Workbench>;
  private skillLevels: Map<string, number>; // crafting skills
  private workshopEfficiency: number;

  constructor() {
    this.tools = new Map();
    this.recipes = new Map();
    this.workbenches = new Map();
    this.skillLevels = new Map();
    this.workshopEfficiency = 0.8;

    this._initializeShed();
  }

  update(dtSeconds: number): void {
    // Update tool conditions
    this._updateToolConditions(dtSeconds);

    // Update workbench conditions
    this._updateWorkbenchConditions(dtSeconds);

    // Process active crafting
    this._processCrafting(dtSeconds);

    // Skill development through practice
    this._updateSkillLevels(dtSeconds);
  }

  addTool(tool: Omit<Tool, 'id'>): string {
    const id = `tool_${Date.now()}_${ChaosSys.getInstance().next()}`;
    const fullTool: Tool = {
      id,
      ...tool
    };

    this.tools.set(id, fullTool);
    return id;
  }

  useTool(toolId: string, intensity: number): boolean {
    const tool = this.tools.get(toolId);
    if (!tool) return false;

    // Check skill requirement
    const skill = this.skillLevels.get(tool.type) || 0;
    if (skill < tool.skill_required) {
      return false; // Insufficient skill
    }

    // Apply wear and tear
    const wearRate = intensity * (1 - tool.durability * 0.5);
    tool.condition = Math.max(0, tool.condition - wearRate * 0.001);

    // Update efficiency based on condition
    tool.efficiency = tool.condition * tool.efficiency;

    return true;
  }

  repairTool(toolId: string, materials: { type: string; quantity: number }[]): boolean {
    const tool = this.tools.get(toolId);
    if (!tool) return false;

    // Check if materials are available (would need resource system integration)
    const repairAmount = 0.3; // 30% repair per action
    tool.condition = Math.min(1, tool.condition + repairAmount);
    tool.efficiency = tool.condition * tool.efficiency;

    return true;
  }

  startCrafting(recipeId: string, workbenchId: string): boolean {
    const recipe = this.recipes.get(recipeId);
    const workbench = this.workbenches.get(workbenchId);

    if (!recipe || !workbench) return false;

    // Check skill requirements
    const skill = this.skillLevels.get(recipe.category) || 0;
    if (skill < recipe.skill_required) {
      return false;
    }

    // Check tool requirements
    if (recipe.tool_required && !this.tools.has(recipe.tool_required)) {
      return false;
    }

    // Start crafting
    workbench.current_project = recipeId;
    workbench.progress = 0;

    return true;
  }

  getToolStatus(): {
    total_tools: number;
    average_condition: number;
    tool_types: string[];
    critical_repairs: string[];
  } {
    if (this.tools.size === 0) {
      return {
        total_tools: 0,
        average_condition: 0,
        tool_types: [],
        critical_repairs: []
      };
    }

    const conditions = Array.from(this.tools.values()).map(t => t.condition);
    const toolTypes = Array.from(new Set(Array.from(this.tools.values()).map(t => t.type)));
    const criticalRepairs = Array.from(this.tools.entries())
      .filter(([_, tool]) => tool.condition < 0.3)
      .map(([id, _]) => id);

    return {
      total_tools: this.tools.size,
      average_condition: conditions.reduce((sum, c) => sum + c, 0) / conditions.length,
      tool_types: toolTypes,
      critical_repairs: criticalRepairs
    };
  }

  getCraftingStatus(): {
    available_recipes: number;
    skill_level: { [category: string]: number };
    active_projects: number;
    workshop_efficiency: number;
  } {
    const skillLevel: { [category: string]: number } = {};
    for (const [category, level] of this.skillLevels) {
      skillLevel[category] = level;
    }

    const activeProjects = Array.from(this.workbenches.values())
      .filter(wb => wb.current_project).length;

    return {
      available_recipes: this.recipes.size,
      skill_level: skillLevel,
      active_projects: activeProjects,
      workshop_efficiency: this.workshopEfficiency
    };
  }

  learnSkill(skillType: string, amount: number): void {
    const currentLevel = this.skillLevels.get(skillType) || 0;
    const newLevel = Math.min(1, currentLevel + amount);
    this.skillLevels.set(skillType, newLevel);

    // Learning improves workshop efficiency
    this._updateWorkshopEfficiency();
  }

  private _initializeShed(): void {
    // Add workbenches
    this.workbenches.set('carpentry_bench', {
      id: 'carpentry_bench',
      type: 'carpentry',
      condition: 0.9,
      efficiency: 0.8,
      progress: 0
    });

    this.workbenches.set('metalwork_bench', {
      id: 'metalwork_bench',
      type: 'metalwork',
      condition: 0.85,
      efficiency: 0.75,
      progress: 0
    });

    // Add initial tools
    this.addTool({
      type: 'cutting',
      name: 'Hand Saw',
      condition: 0.8,
      efficiency: 0.7,
      durability: 0.6,
      skill_required: 0.2
    });

    this.addTool({
      type: 'building',
      name: 'Hammer',
      condition: 0.9,
      efficiency: 0.8,
      durability: 0.8,
      skill_required: 0.1
    });

    // Add initial recipes
    this.recipes.set('wooden_plank', {
      id: 'wooden_plank',
      name: 'Wooden Plank',
      category: 'resource',
      inputs: new Map([['wood', 1]]),
      outputs: new Map([['wooden_plank', 4]]),
      time_required: 30,
      skill_required: 0.1,
      tool_required: 'cutting'
    });

    this.recipes.set('simple_shelter', {
      id: 'simple_shelter',
      name: 'Simple Shelter',
      category: 'building',
      inputs: new Map([['wooden_plank', 20], ['stone', 10]]),
      outputs: new Map([['shelter', 1]]),
      time_required: 3600, // 1 hour
      skill_required: 0.3,
      tool_required: 'building'
    });

    // Initialize skills
    this.skillLevels.set('carpentry', 0.3);
    this.skillLevels.set('metalwork', 0.2);
    this.skillLevels.set('cooking', 0.4);
    this.skillLevels.set('crafting', 0.3);
  }

  private _updateToolConditions(dtSeconds: number): void {
    for (const tool of this.tools.values()) {
      // Natural decay
      tool.condition -= dtSeconds * 0.00001;
      tool.condition = Math.max(0, tool.condition);

      // Update efficiency
      tool.efficiency = tool.condition * tool.efficiency;
    }
  }

  private _updateWorkbenchConditions(dtSeconds: number): void {
    for (const workbench of this.workbenches.values()) {
      // Natural wear
      workbench.condition -= dtSeconds * 0.000005;
      workbench.condition = Math.max(0, workbench.condition);

      // Update efficiency
      workbench.efficiency = workbench.condition * workbench.efficiency;
    }
  }

  private _processCrafting(dtSeconds: number): void {
    for (const workbench of this.workbenches.values()) {
      if (!workbench.current_project) continue;

      const recipe = this.recipes.get(workbench.current_project);
      if (!recipe) continue;

      // Calculate crafting speed
      const skill = this.skillLevels.get(recipe.category) || 0;
      const toolBonus = recipe.tool_required ? 
        (this.tools.get(recipe.tool_required)?.efficiency || 1) : 1;
      
      const craftingSpeed = (1 + skill) * toolBonus * workbench.efficiency * this.workshopEfficiency;
      const progressIncrement = (craftingSpeed * dtSeconds) / recipe.time_required;

      workbench.progress = Math.min(1, workbench.progress + progressIncrement);

      // Complete crafting
      if (workbench.progress >= 1) {
        this._completeCrafting(recipe, workbench);
        workbench.current_project = undefined;
        workbench.progress = 0;

        // Skill gain from crafting
        this.learnSkill(recipe.category, 0.01);
      }
    }
  }

  private _updateSkillLevels(dtSeconds: number): void {
    // Very slow natural skill decay (if not used)
    for (const [skill, level] of this.skillLevels) {
      const decayRate = 0.00001;
      this.skillLevels.set(skill, Math.max(0, level - decayRate * dtSeconds));
    }
  }

  private _completeCrafting(recipe: CraftingRecipe, workbench: Workbench): void {
    // This would integrate with resource/inventory system
    // For now, just log the completion
    console.log(`Completed crafting: ${recipe.name}`);
    
    // Tool wear from crafting
    if (recipe.tool_required) {
      const tool = this.tools.get(recipe.tool_required);
      if (tool) {
        tool.condition = Math.max(0, tool.condition - 0.01);
      }
    }
  }

  private _updateWorkshopEfficiency(): void {
    // Efficiency based on average tool condition and workbench condition
    const avgToolCondition = this.tools.size > 0 ? 
      Array.from(this.tools.values()).reduce((sum, t) => sum + t.condition, 0) / this.tools.size : 0.5;
    
    const avgWorkbenchCondition = this.workbenches.size > 0 ?
      Array.from(this.workbenches.values()).reduce((sum, wb) => sum + wb.condition, 0) / this.workbenches.size : 0.5;

    this.workshopEfficiency = (avgToolCondition * 0.6) + (avgWorkbenchCondition * 0.4);
  }

  getShedReport(): {
    tools: any;
    crafting: any;
    skills: { [category: string]: number };
    efficiency: number;
  } {
    const skills: { [category: string]: number } = {};
    for (const [category, level] of this.skillLevels) {
      skills[category] = level;
    }

    return {
      tools: this.getToolStatus(),
      crafting: this.getCraftingStatus(),
      skills,
      efficiency: this.workshopEfficiency
    };
  }

  resetShed(): void {
    this.tools.clear();
    this.recipes.clear();
    this.workbenches.clear();
    this.skillLevels.clear();
    this.workshopEfficiency = 0.8;
    this._initializeShed();
  }
}
