use crate::item::Item;
use ggez::Context;

pub struct ItemCollection {
    pub items: Vec<Item>,
}

impl ItemCollection {
    pub fn new() -> ItemCollection {
        ItemCollection { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Item> {
        self.items.get_mut(index)
    }

    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        for item in &self.items {
            item.draw(ctx)?;
        }
        Ok(())
    }
}
