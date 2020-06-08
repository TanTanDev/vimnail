//! This example demonstrates how to use `Text` to draw TrueType font texts efficiently.

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

mod input_action;
mod action_processer;
mod action_type;
mod mode;
mod insert_type;
mod mode_history;
mod constants;
mod graphics;

use action_processer::{ActionProcesser, ActionProcesserBuilder};
use mode::Mode;
use input_action::{InputAction};
use action_type::ActionType;
use insert_type::InsertType;
use mode_history::ModeHistory;
use graphics::mode_visualizer::ModeVisualizer;

struct App {
    //font: Font,
    //scale: f32,
    action_processer: ActionProcesser,
    mode: Mode,
    mode_history: ModeHistory,
    mode_visualizer: ModeVisualizer,
}

impl App {
    fn new(ctx: &mut Context) -> GameResult<App> {
        let action_processer = ActionProcesserBuilder::new()
            .with_mode(Mode::Any)
            .with_mode(Mode::Command)
            .with_mode(Mode::Edit)
            .with_mode(Mode::Insert)
            .with_mode(Mode::InsertType(InsertType::Image))
            .with_mode(Mode::InsertType(InsertType::Text))
            .with_inputaction(Mode::Any, InputAction::new(KeyCode::Escape, ActionType::PreviousMode))
            .with_inputaction(Mode::Command, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::Command, InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::Edit, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::InsertType(InsertType::Image))))
            .with_inputaction(Mode::Insert, InputAction::new(KeyCode::T, ActionType::ChangeMode(Mode::InsertType(InsertType::Text))))
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
            mode_visualizer,
        })
    }
}

impl event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //const DESIRED_FPS: u32 = 60;
        //while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.mode_visualizer.draw(ctx);
        ggez::graphics::present(ctx)?;
        //timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
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
                        println!("going from mode {:?}, to {:?}",self.mode, mode);
                        self.mode = mode;
                        self.mode_history.register(mode);
                        self.mode_visualizer.change(ctx, mode);
                    } else {
                        // command should alwaus be top mode
                        self.mode_history.register(Mode::Command);
                        println!("No mode in history to jump to...");
                    }
                }
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

pub fn main() -> GameResult {
    if cfg!(debug_assertions) && env::var("yes_i_really_want_debug_mode").is_err() {
        eprintln!(
            "Note: Release mode will improve performance greatly.\n    \
             e.g. use `cargo run --example text --release`"
        );
    }
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ContextBuilder::new("text_cached", "ggez")
        .window_setup(WindowSetup::default().title("Vimnail"))
        .window_mode(
            WindowMode::default()
                .dimensions(640.0, 480.0)
                .resizable(true),
        )
        .add_resource_path(resource_dir)
        .build()?;
    let state = &mut App::new(ctx)?;
    event::run(ctx, events_loop, state)
}
