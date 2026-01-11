use crate::types::Agent;
use crate::inventory::{Inventory, Item, ItemType};
use crate::action::CraftRecipe;

pub fn craft(agent: &Agent, recipe: &CraftRecipe, inventory: &mut Inventory) -> Result<Item, String> {
    // Validate that agent has all required inputs
    for (input_type, required_qty) in &recipe.inputs {
        let available_qty = inventory.count_item_type(input_type.clone());
        if available_qty < *required_qty {
            return Err(format!("Insufficient {:?}: need {}, have {}", input_type, required_qty, available_qty));
        }
    }
    
    // Consume input items
    for (input_type, required_qty) in &recipe.inputs {
        let mut remaining_qty = *required_qty;
        let item_ids: Vec<u64> = inventory
            .items
            .values()
            .filter(|item| item.item_type == *input_type)
            .take_while(|item| {
                if remaining_qty == 0 {
                    return false;
                }
                if item.quantity <= remaining_qty {
                    remaining_qty -= item.quantity;
                    true
                } else {
                    false
                }
            })
            .map(|item| item.id)
            .collect();
        
        for item_id in item_ids {
            let _ = inventory.remove_item(item_id);
        }
    }
    
    // Create output item
    let output_item_id = generate_craft_item_id(agent.id, &recipe.output);
    let output_item = match recipe.output {
        ItemType::Tool => Item::tool(output_item_id, 100), // Tools start with 100 durability
        ItemType::Resource => Item::resource(output_item_id, recipe.output, 1),
        ItemType::Food => Item::resource(output_item_id, recipe.output, 1),
        ItemType::Vehicle => Item::resource(output_item_id, recipe.output, 1),
        ItemType::BuildingBlock => Item::resource(output_item_id, recipe.output, 1),
    };
    
    // Add output to inventory
    inventory.add_item(output_item.clone())?;
    
    Ok(output_item)
}

pub fn get_available_recipes(agent: &Agent) -> Vec<CraftRecipe> {
    let mut recipes = Vec::new();
    
    // Basic tool recipes
    recipes.push(CraftRecipe::new(
        vec![(ItemType::Resource, 2)].into_iter().collect(),
        ItemType::Tool,
        5, // 5 ticks to craft
    ));
    
    // Building block recipes
    recipes.push(CraftRecipe::new(
        vec![(ItemType::Resource, 1)].into_iter().collect(),
        ItemType::BuildingBlock,
        2, // 2 ticks to craft
    ));
    
    // Food recipes (if agent has food resources)
    if agent.inventory.values().any(|asset| {
        asset.name == "Resource"
    }) {
        recipes.push(CraftRecipe::new(
            vec![(ItemType::Resource, 1)].into_iter().collect(),
            ItemType::Food,
            3, // 3 ticks to craft
        ));
    }
    
    recipes
}

fn generate_craft_item_id(agent_id: u64, output_type: &ItemType) -> u64 {
    // Generate deterministic item ID
    let type_seed = match output_type {
        ItemType::Tool => 9000,
        ItemType::Resource => 9100,
        ItemType::Food => 9200,
        ItemType::Vehicle => 9300,
        ItemType::BuildingBlock => 9400,
    };
    
    agent_id * 10000 + type_seed
}
