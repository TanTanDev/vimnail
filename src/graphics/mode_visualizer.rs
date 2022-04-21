use crate::mode::Mode;

use ggez;
use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::GameResult;
use nalgebra;

pub struct ModeVisualizer {
    text: Text,
    rect_mesh: Mesh,
    rect: Rect,
}

impl ModeVisualizer {
    pub fn new(ctx: &mut ggez::Context, mode: Mode) -> Self {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // todo: fix error handling
        let rect_mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            rect,
            ggez::graphics::Color::BLACK,
        )
        .unwrap();
        let mut mode_visualizer = ModeVisualizer {
            rect_mesh,
            text: Text::new(TextFragment::default()),
            rect,
        };
        mode_visualizer.change(ctx, mode);
        mode_visualizer
    }

    pub fn change(&mut self, ctx: &mut ggez::Context, mode: Mode) {
        self.text = Text::new(TextFragment {
            text: mode.to_string(),
            color: Some(ggez::graphics::Color::WHITE),
            ..Default::default()
        });
        let Rect {
            w: text_w,
            h: text_h,
            ..
        } = self.text.dimensions(ctx);
        self.rect = Rect::new(0.0, 0.0, text_w as f32, text_h as f32);
        self.rect_mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            ggez::graphics::Color::BLACK,
        )
        .unwrap();
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);
        let draw_param = DrawParam::default();
        draw_param.dest(nalgebra::Point2::new(0.0, screen_rect.h - self.rect.h));
        ggez::graphics::draw(ctx, &self.rect_mesh, draw_param)?;
        ggez::graphics::draw(ctx, &self.text, draw_param)?;
        Ok(())
    }
}
