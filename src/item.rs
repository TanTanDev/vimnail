use crate::direction::Direction;
use ggez;
use ggez::Context;
use nalgebra;

#[derive(Clone, Copy)]
pub struct Item {
    //position: nalgebra::Vector2<f32>,
//scale: nalgebra::Vector2<f32>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ItemType {
    Image,
    Text,
    ParticleSystem,
}

impl Default for Item {
    fn default() -> Self {
        todo!()
    }
}

pub trait ItemData {}

pub struct Image {
    //path: String,
    image: ggez::graphics::Image,
    pub position: nalgebra::Vector2<f32>,
    pub rotation: f32,
    pub scale: nalgebra::Vector2<f32>,
}

impl ItemData for Image {}

impl Image {
    pub fn new(ctx: &mut Context, path: String) -> ggez::GameResult<Self> {
        let image = ggez::graphics::Image::new(ctx, &path)?;
        Ok(Image {
            //path,
            image,
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
        ggez::graphics::draw(ctx, &self.image, params)?;
        Ok(())
    }

    pub fn rotate(&mut self, dt: f32, move_dir: Direction) {
        let rotate_speed = 10.0;
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

    pub fn edit_move(&mut self, dt: f32, move_dir: Direction) {
        let move_speed = 1000.0;
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

    pub fn scale(&mut self, dt: f32, move_dir: Direction) {
        match move_dir {
            Direction::Left => {
                self.scale.x += dt;
            }
            Direction::Right => {
                self.scale.x -= dt;
            }
            Direction::Up => {
                self.scale.y += dt;
            }
            Direction::Down => {
                self.scale.y -= dt;
            }
        }
    }

    pub fn scale_uniform(&mut self, dt: f32, move_dir: Direction) {
        match move_dir {
            Direction::Left => {
                self.scale.x += dt;
                self.scale.y += dt;
            }
            Direction::Right => {
                self.scale.x -= dt;
                self.scale.y -= dt;
            }
            Direction::Up => {
                self.scale.x -= dt;
                self.scale.y -= dt;
            }
            Direction::Down => {
                self.scale.x += dt;
                self.scale.y += dt;
            }
        }
    }
}
