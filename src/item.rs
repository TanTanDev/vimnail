use crate::constants::*;
use crate::direction::Direction;

use ggez;
use ggez::Context;
use ggez::graphics::{Color, Drawable, Font, Image, Scale, Text, TextFragment};
use nalgebra;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ItemType {
    Image,
    Text,
    ParticleSystem,
}

// impl Default for Item {
//     fn default() -> Self {
//         todo!()
//     }
// }

pub struct Item {
    content: Box<dyn Drawable>,
    pub position: nalgebra::Vector2<f32>,
    pub rotation: f32,
    pub scale: nalgebra::Vector2<f32>,
}

impl Item {
    pub fn new(c: Box<dyn Drawable>) -> ggez::GameResult<Self> {
        Ok(Item {
            content: c,
            position: nalgebra::Vector2::<f32>::new(0.0, 0.0),
            scale: nalgebra::Vector2::<f32>::new(1.0, 1.0),
            rotation: 0.0,
        })
    }

    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        let mut params = ggez::graphics::DrawParam::default();
        params.dest = mint::Point2 {
            x: self.position.x,
            y: self.position.y,
        };
        params.rotation = self.rotation;
        params.offset = mint::Point2 { x: 0.5, y: 0.5 };
        params.scale = self.scale.into();
        self.content.draw(ctx, params)?;
        Ok(())
    }

    pub fn rotate(&mut self, dt: f32, move_dir: Direction, is_fast: bool) {
        let rotate_speed = match is_fast {
            true => SPEED_ROTATE_FAST,
            false => SPEED_ROTATE_DEFAULT,
        };
        let delta_speed = rotate_speed * dt;
        match move_dir {
            Direction::Left => {
                self.rotation -= delta_speed;
            }
            Direction::Right => {
                self.rotation += delta_speed;
            }
            Direction::Up => {
                self.rotation += delta_speed;
            }
            Direction::Down => {
                self.rotation -= delta_speed;
            }
        }
    }

    pub fn edit_move(&mut self, dt: f32, move_dir: Direction, is_fast: bool) {
        let move_speed = match is_fast {
            true => SPEED_MOVE_FAST,
            false => SPEED_MOVE_DEFAULT,
        };
        let move_delta = move_speed * dt;
        match move_dir {
            Direction::Up => {
                self.position.y -= move_delta;
            }
            Direction::Down => {
                self.position.y += move_delta;
            }
            Direction::Left => {
                self.position.x -= move_delta;
            }
            Direction::Right => {
                self.position.x += move_delta;
            }
        }
    }

    pub fn scale(&mut self, dt: f32, move_dir: Direction, is_fast: bool) {
        let scale_speed = match is_fast {
            true => SPEED_SCALE_FAST,
            false => SPEED_SCALE_DEFAULT,
        };
        let scale_delta = scale_speed * dt;
        match move_dir {
            Direction::Left => {
                self.scale.x += scale_delta;
            }
            Direction::Right => {
                self.scale.x -= scale_delta;
            }
            Direction::Up => {
                self.scale.y += scale_delta;
            }
            Direction::Down => {
                self.scale.y -= scale_delta;
            }
        }
    }

    pub fn scale_uniform(&mut self, dt: f32, move_dir: Direction, is_fast: bool) {
        let scale_speed = match is_fast {
            true => SPEED_SCALE_FAST,
            false => SPEED_SCALE_DEFAULT,
        };
        let scale_delta = scale_speed * dt;
        match move_dir {
            Direction::Left => {
                self.scale.x += scale_delta;
                self.scale.y += scale_delta;
            }
            Direction::Right => {
                self.scale.x -= scale_delta;
                self.scale.y -= scale_delta;
            }
            Direction::Up => {
                self.scale.x -= scale_delta;
                self.scale.y -= scale_delta;
            }
            Direction::Down => {
                self.scale.x += scale_delta;
                self.scale.y += scale_delta;
            }
        }
    }
}

//TODO: Add more parameters support for more customized Images
pub fn build_image_item(ctx: &mut Context, path: String) -> ggez::GameResult<Item> {
    Ok(Item::new(Box::new(Image::new(ctx, path)?))?)
}

//TODO: Add more parameters support for more customized Text
pub fn build_text_item(text: String) -> ggez::GameResult<Item> {
    Ok(Item::new(Box::new(Text::new(TextFragment {
        text,
        color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
        font: Some(Font::default()),
        scale: Some(Scale::uniform(50.0)),
        ..Default::default()
    })))?)
}