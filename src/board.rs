use crate::direction::Direction;
use crate::item::Item;
use crate::item_collection::ItemCollection;
use ggez::{Context, GameResult};
pub struct Board {
    pub item_collection: ItemCollection,
}

impl Board {
    pub fn new() -> Self {
        Board {
            item_collection: ItemCollection::new(),
        }
    }

    // invoke provided function on item if it exists
    pub fn edit_item(
        &mut self,
        dt: f32,
        direction: Direction,
        is_fast: bool,
        maybe_index: Option<usize>,
        edit_func: fn(&mut Item, f32, Direction, bool),
    ) {
        if let Some(index) = maybe_index {
            if let Some(image) = self.item_collection.get_mut(index) {
                edit_func(image, dt, direction, is_fast);
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        self.item_collection.draw(ctx)
    }
}
