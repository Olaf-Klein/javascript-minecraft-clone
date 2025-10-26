use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub id: String,
    pub count: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub size: usize,
    pub slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            slots: vec![None; size],
        }
    }

    pub fn first_empty(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_none())
    }

    pub fn add_item(&mut self, item: ItemStack) -> Result<(), ItemStack> {
        if let Some(i) = self.slots.iter().position(|s| match s {
            Some(s) => s.id == item.id && s.count < u16::MAX,
            None => false,
        }) {
            if let Some(slot) = &mut self.slots[i] {
                slot.count = slot.count.saturating_add(item.count);
                return Ok(());
            }
        }

        if let Some(i) = self.first_empty() {
            self.slots[i] = Some(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn remove_at(&mut self, idx: usize, count: u16) -> Option<ItemStack> {
        if idx >= self.size {
            return None;
        }
        if let Some(mut stack) = self.slots[idx].take() {
            if count >= stack.count {
                return Some(stack);
            } else {
                let remaining = stack.count - count;
                let taken = ItemStack { id: stack.id.clone(), count };
                stack.count = remaining;
                self.slots[idx] = Some(stack);
                return Some(taken);
            }
        }
        None
    }
}
