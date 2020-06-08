use crate::mode::Mode;

use ggez::graphics::{Mesh, Color, Text, TextFragment, Drawable, DrawParam, BlendMode, DrawMode, Rect};
use ggez;
use ggez::{GameResult};
use nalgebra;

pub struct ModeVisualizer
{
    blend_mode: Option<BlendMode>,
    text: Text,
    rect_mesh: Mesh,
    rect: Rect,
}

impl ModeVisualizer {
    pub fn new(ctx: &mut ggez::Context, mode: Mode) -> Self {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // todo: fix error handling
        let rect_mesh = ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect , ggez::graphics::BLACK).unwrap(); 

        let mut mode_visualizer = ModeVisualizer {
            blend_mode: Some(BlendMode::Multiply),
            //text_fragment,
            rect_mesh,
            text: Text::new(TextFragment::default()),
            rect,
        };
        mode_visualizer.change(ctx, mode);
        mode_visualizer
    }
    
    pub fn change(&mut self, ctx: &mut ggez::Context, mode: Mode) {
        self.text = Text::new(TextFragment{ text: mode.to_string(), color: Some(ggez::graphics::WHITE), .. Default::default()});
        let (text_w, text_h) = self.text.dimensions(ctx);
        self.rect = Rect::new(0.0, 0.0, text_w as f32, text_h as f32);
        self.rect_mesh = ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), self.rect, ggez::graphics::BLACK).unwrap(); 
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);
        let mut draw_param = DrawParam::default();
        draw_param.dest = nalgebra::Point2::new(0.0, screen_rect.h - self.rect.h).into(); 
        ggez::graphics::draw(ctx, &self.rect_mesh, draw_param)?;
        ggez::graphics::draw(ctx, &self.text, draw_param)?;
        Ok(())
    }
}