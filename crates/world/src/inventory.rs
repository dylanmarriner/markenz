use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy, PartialOrd, Ord, Eq)]
pub enum ItemType {
    Tool,
    Resource,
    Food,
    Vehicle,
    BuildingBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: u64,
    pub item_type: ItemType,
    pub quantity: u32,
    pub durability: u16,
}

impl Item {
    pub fn new(id: u64, item_type: ItemType, quantity: u32, durability: u16) -> Self {
        Self {
            id,
            item_type,
            quantity,
            durability,
        }
    }

    pub fn tool(id: u64, durability: u16) -> Self {
        Self::new(id, ItemType::Tool, 1, durability)
    }

    pub fn resource(id: u64, item_type: ItemType, quantity: u32) -> Self {
        Self::new(id, item_type, quantity, u16::MAX) // Resources don't have durability
    }

    pub fn degrade(&mut self, amount: u16) -> bool {
        if self.durability > amount {
            self.durability -= amount;
            true
        } else {
            self.durability = 0;
            false // Tool broken
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub max_slots: u16,
    pub items: BTreeMap<u64, Item>,
}

impl Inventory {
    pub fn new(max_slots: u16) -> Self {
        Self {
            max_slots,
            items: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_slots as usize {
            return Err("Inventory full".to_string());
        }

        // Check if we can stack with existing item
        if let Some(existing_item) = self.items.get_mut(&item.id) {
            if existing_item.item_type == item.item_type && matches!(item.item_type, ItemType::Resource) {
                existing_item.quantity += item.quantity;
                return Ok(());
            }
        }

        let _ = self.items.insert(item.id, item);
        Ok(())
    }

    pub fn remove_item(&mut self, item_id: u64) -> Option<Item> {
        self.items.remove(&item_id)
    }

    pub fn find_items(&self, item_type: ItemType) -> Vec<&Item> {
        self.items
            .values()
            .filter(|item| item.item_type == item_type)
            .collect()
    }

    pub fn has_item_type(&self, item_type: ItemType) -> bool {
        self.items.values().any(|item| item.item_type == item_type)
    }

    pub fn count_item_type(&self, item_type: ItemType) -> u32 {
        self.items
            .values()
            .filter(|item| item.item_type == item_type)
            .map(|item| item.quantity)
            .sum()
    }
}
