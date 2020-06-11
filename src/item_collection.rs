use crate::item::Image;
use ggez::Context;

pub struct ItemCollection {
    pub items: Vec<Image>,
}

impl ItemCollection {
    pub fn new() -> ItemCollection {
        ItemCollection { items: Vec::new() }
    }

    pub fn add(&mut self, image: Image) {
        self.items.push(image);
    }

    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<&Image> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Image> {
        self.items.get_mut(index)
    }

    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        for item in &self.items {
            item.draw(ctx)?;
        }
        Ok(())
    }
}
