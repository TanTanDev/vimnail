use cgmath;
use ggez;
use rand;

use cgmath::Point2;
use cgmath::Vector2;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{Align, Color, DrawParam, Font, Scale, Text, TextFragment};
use ggez::timer;
use ggez::input::keyboard::*;
use ggez::input::keyboard::KeyMods;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::f32;
use std::path;

use crate::input_action;
use crate::action_processer;
use crate::action_type;
use crate::mode;
use crate::insert_type;
use crate::mode_history;
use crate::constants;
use crate::graphics;
use crate::item;
use crate::item_collection;
use crate::edit_type;
use crate::direction;
use crate::board;

use action_processer::{ActionProcesser, ActionProcesserBuilder};
use mode::Mode;
use input_action::{InputAction};
use action_type::ActionType;
use insert_type::InsertType;
use mode_history::ModeHistory;
use graphics::mode_visualizer::ModeVisualizer;
use item::{Image, ItemType};
use item_collection::ItemCollection;
use edit_type::EditType;
use direction::Direction;
use board::Board;
pub struct App {
    //font: Font,
    //scale: f32,
    action_processer: ActionProcesser,
    mode: Mode,
    mode_history: ModeHistory,
    mode_visualizer: ModeVisualizer,
    board: Board,
    current_edit_index: Option<usize>,
}

impl App {
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        let action_processer = ActionProcesserBuilder::new()
            .with_mode(Mode::Any)
            .with_mode(Mode::Command)
            .with_mode(Mode::Edit)
            .with_mode(Mode::Insert)
            .with_mode(Mode::InsertType(ItemType::Image))
            .with_mode(Mode::InsertType(ItemType::Text))
            //.with_inputaction(Mode::Any, InputAction::new(KeyCode::Escape, ActionType::PreviousMode))
            .with_inputaction(Mode::Command, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::Command, InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::M, ActionType::ChangeMode(Mode::EditType(EditType::Move))))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::R, ActionType::ChangeMode(Mode::EditType(EditType::Rotate))))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Command)))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::S, ActionType::ChangeMode(Mode::EditType(EditType::Scale))))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Command)))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::InsertType(ItemType::Image))))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::T, ActionType::ChangeMode(Mode::InsertType(ItemType::Text))))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::P, ActionType::ChangeMode(Mode::InsertType(ItemType::ParticleSystem))))

            .with_inputaction(Mode::InsertType(ItemType::Image), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::InsertType(ItemType::Image), InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::Image)))
            .with_inputaction(Mode::InsertType(ItemType::Image), InputAction::new(KeyCode::L, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::InsertType(ItemType::Text), InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::Text)))
            .with_inputaction(Mode::InsertType(ItemType::Text), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::InsertType(ItemType::ParticleSystem), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::InsertType(ItemType::ParticleSystem), InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::ParticleSystem)))

            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::S, ActionType::ChangeMode(Mode::EditType(EditType::Scale))))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::R, ActionType::ChangeMode(Mode::EditType(EditType::Rotate))))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::H, ActionType::Move(Direction::Left)))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::J, ActionType::Move(Direction::Down)))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::K, ActionType::Move(Direction::Up)))
            .with_inputaction(Mode::EditType(EditType::Move), InputAction::new(KeyCode::L, ActionType::Move(Direction::Right)))

            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::M, ActionType::ChangeMode(Mode::EditType(EditType::Move))))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::S, ActionType::ChangeMode(Mode::EditType(EditType::Scale))))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::L, ActionType::Rotate(Direction::Right)))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::H, ActionType::Rotate(Direction::Left)))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::J, ActionType::Rotate(Direction::Down)))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::K, ActionType::Rotate(Direction::Up)))
            .with_inputaction(Mode::EditType(EditType::Rotate), InputAction::new(KeyCode::M, ActionType::ChangeMode(Mode::EditType(EditType::Move))))

            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::R, ActionType::ChangeMode(Mode::EditType(EditType::Rotate))))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::M, ActionType::ChangeMode(Mode::EditType(EditType::Move))))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::U, ActionType::ChangeMode(Mode::EditType(EditType::ScaleUniform))))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::H, ActionType::Scale(Direction::Left)))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::J, ActionType::Scale(Direction::Down)))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::K, ActionType::Scale(Direction::Up)))
            .with_inputaction(Mode::EditType(EditType::Scale), InputAction::new(KeyCode::L, ActionType::Scale(Direction::Right)))

            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::R, ActionType::ChangeMode(Mode::EditType(EditType::Rotate))))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::M, ActionType::ChangeMode(Mode::EditType(EditType::Move))))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::S, ActionType::ChangeMode(Mode::EditType(EditType::Scale))))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::H, ActionType::ScaleUniform(Direction::Left)))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::J, ActionType::ScaleUniform(Direction::Down)))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::K, ActionType::ScaleUniform(Direction::Up)))
            .with_inputaction(Mode::EditType(EditType::ScaleUniform), InputAction::new(KeyCode::L, ActionType::ScaleUniform(Direction::Right)))
            .build();

        let mut mode_history = ModeHistory::new();
        mode_history.register(Mode::Command);
        let mode = Mode::Command;

        let mode_visualizer = ModeVisualizer::new(ctx, mode);
        Ok(App {
            //font: Font::new(ctx, "/Montserrat-Black.ttf")?,
            //scale: 1.0,
            mode,
            action_processer,
            mode_history,
            board: Board::new(),
            mode_visualizer,
            current_edit_index: None,
        })
    }

    fn set_mode(&mut self, ctx: &mut Context, mode: Mode) {
        println!("going from mode {:?}, to {:?}",self.mode, mode);
        self.mode = mode;
        self.mode_history.register(mode);
        self.mode_visualizer.change(ctx, mode);
    }

    fn handle_key(&mut self, ctx: &mut Context, keycode: KeyCode) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let maybe_action_type = self.action_processer.process_input(self.mode, keycode);
        if let Some(action_type) = maybe_action_type {
            match action_type {
                ActionType::ChangeMode(m) => { 
                    self.mode = m;
                    self.mode_history.register(m);
                    self.mode_visualizer.change(ctx, m);
                    println!("changed mode to: {:?}", m);
                },
                ActionType::PreviousMode => {
                    let maybe_mode = self.mode_history.prev_consume();
                    if let Some(mode) = maybe_mode {
                        self.set_mode(ctx, mode);
                    } else {
                        // command should alwaus be top mode
                        self.mode_history.register(Mode::Command);
                        println!("No mode in history to jump to...");
                    }
                },
                ActionType::AddItem(item) => {
                    if item == ItemType::Image {
                        // todo: remove unwrap
                        let image = Image::new(ctx, "/ferris.png".to_string()).unwrap();
                        self.board.item_collection.add(image);
                        self.set_mode(ctx, Mode::Edit);
                        self.current_edit_index = Some(self.board.item_collection.items.len()-1);
                    }
                },
                ActionType::Move(direction) => {
                    self.board.edit_item(dt, direction, self.current_edit_index, item::Image::edit_move);
                },
                ActionType::Rotate(direction) => {
                    self.board.edit_item(dt, direction, self.current_edit_index, item::Image::rotate);
                },
                ActionType::Scale(direction) => {
                    self.board.edit_item(dt, direction, self.current_edit_index, item::Image::scale);
                },
                ActionType::ScaleUniform(direction) => {
                    self.board.edit_item(dt, direction, self.current_edit_index, item::Image::scale_uniform);
                }
                //_ => {},
            }
        }
    }
}

impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //const DESIRED_FPS: u32 = 60;
        //while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.board.draw(ctx)?;
        self.mode_visualizer.draw(ctx);
        ggez::graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {

        let maybe_action_type = self.action_processer.process_input(self.mode, keycode);
        self.handle_key(ctx, keycode);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}