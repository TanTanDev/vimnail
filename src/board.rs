use crate::direction::Direction;
use crate::item::Image;
use crate::item_collection::ItemCollection;
use ggez::conf;
use ggez::{graphics, Context, GameResult};
use image::{DynamicImage, ImageBuffer};
use std::path::Path;

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
        edit_func: fn(&mut Image, f32, Direction, bool),
    ) {
        if let Some(index) = maybe_index {
            if let Some(image) = self.item_collection.get_mut(index) {
                edit_func(image, dt, direction, is_fast);
            }
        }
    }

    pub fn save_image(&mut self, ctx: &mut Context, path: &String) {
        //TODO: Add better error handling and bubling the error into UI

        let (graphic_width, graphic_height) = graphics::drawable_size(ctx);

        let buffer_canvas = graphics::Canvas::new(
            ctx,
            graphic_width as u16,
            graphic_height as u16,
            conf::NumSamples::One,
            graphics::get_window_color_format(ctx),
        )
        .ok();

        //divert drawing into canvas
        graphics::set_canvas(ctx, buffer_canvas.as_ref());
        self.draw(ctx).unwrap();
        ggez::graphics::present(ctx).unwrap();

        // capture the image from canvas into raw vector
        let canvas = buffer_canvas.unwrap();
        let image_rgba = canvas.to_rgba8(ctx).unwrap();

        // convert the image to DynamicImage from raw vector
        let image_buffer = DynamicImage::ImageRgba8(
            ImageBuffer::from_raw(graphic_width as u32, graphic_height as u32, image_rgba).unwrap(),
        );

        // flip the image because the raw image vector is flipped.. dunno why ¯\_(ツ)_/¯
        let image_buffer = image_buffer.flipv();

        //save the image
        let image_path = Path::new(path).as_os_str();

        match image_buffer.save(image_path) {
            Ok(_) => println!("Image saved successfully!"),
            Err(err) => {
                println!("Failed to save image : {:?}", err);
            }
        }

        //return to drawing into screen
        graphics::set_canvas(ctx, Option::None);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        self.item_collection.draw(ctx)
    }
}
