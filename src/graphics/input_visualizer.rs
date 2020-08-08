use crate::text_input::TextInput;

use ggez;
use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::GameResult;
use nalgebra;

pub struct InputVisualizer {
    text: Text,
    rect_mesh: Mesh,
    rect: Rect,
    prefix: String,
}

impl InputVisualizer {
    pub fn new(ctx: &mut ggez::Context, text: &TextInput) -> Self {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // todo: fix error handling
        let rect_mesh =
            ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, ggez::graphics::BLACK)
                .unwrap();
        let mut input_visualizer = InputVisualizer {
            rect_mesh,
            text: Text::new(TextFragment::default()),
            rect,
            prefix: String::from(""),
        };
        input_visualizer.change(ctx, text);
        input_visualizer
    }

    pub fn change(&mut self, ctx: &mut ggez::Context, text: &TextInput) {

        let mut text_content = String::from("");

        if self.prefix.len() > 0 {
            text_content  = format!("{} : {}", self.prefix, text.content.to_string())
        }

        self.text = Text::new(TextFragment {
            text: text_content,
            color: Some(ggez::graphics::WHITE),
            ..Default::default()
        });
        let (text_w, text_h) = self.text.dimensions(ctx);
        self.rect = Rect::new(0.0, 0.0, text_w as f32, text_h as f32);
        self.rect_mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            ggez::graphics::BLACK,
        )
        .unwrap();
    }

    pub fn set_prefix(&mut self, prefix: String){
        self.prefix = prefix;
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        let screen_rect = ggez::graphics::screen_coordinates(ctx);
        let mut draw_param = DrawParam::default();
        draw_param.dest = nalgebra::Point2::new(0.0, screen_rect.h - self.rect.h - 20.0).into();
        ggez::graphics::draw(ctx, &self.rect_mesh, draw_param)?;
        ggez::graphics::draw(ctx, &self.text, draw_param)?;
        Ok(())
    }
}
