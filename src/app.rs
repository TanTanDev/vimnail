use ggez;

use ggez::event;
use ggez::input::keyboard::KeyMods;
use ggez::input::keyboard::*;
use ggez::timer;
use ggez::{Context, GameResult};
use std::f32;

use crate::action_processer;
use crate::action_processer_utils;
use crate::action_type;
use crate::board;
use crate::graphics;
use crate::item;
use crate::key_state::KeyState;
use crate::keyboard_input_tracker::KeyboardInputTracker;
use crate::mode;
use crate::mode_history;

use action_processer::{ActionProcesser, ActionProcesserBuilder};
use action_type::ActionType;
use board::Board;
use graphics::mode_visualizer::ModeVisualizer;
use item::ItemType;
use mode::Mode;
use mode_history::ModeHistory;

pub struct App {
    //font: Font,
    //scale: f32,
    action_processer: ActionProcesser,
    mode: Mode,
    mode_history: ModeHistory,
    mode_visualizer: ModeVisualizer,
    board: Board,
    current_edit_index: Option<usize>,
    keyboard_input_tracker: KeyboardInputTracker,
}

impl App {
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        let mut action_processer_builder = ActionProcesserBuilder::new();
        action_processer_utils::configure_default(&mut action_processer_builder);
        let action_processer = action_processer_builder.build();

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
            keyboard_input_tracker: KeyboardInputTracker::new(),
            mode_visualizer,
            current_edit_index: None,
        })
    }

    fn set_mode(&mut self, ctx: &mut Context, mode: Mode) {
        println!("going from mode {:?}, to {:?}", self.mode, mode);
        self.mode = mode;
        self.mode_history.register(mode);
        self.mode_visualizer.change(ctx, mode);
    }
}

impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //const DESIRED_FPS: u32 = 60;
        //while timer::check_update_time(ctx, DESIRED_FPS) {}

        // todo: improve code, need to clone because iterating something containing to self
        // whilst trying to mutate self variables, is not allowed
        let is_fast = ggez::input::keyboard::is_mod_active(ctx, KeyMods::SHIFT);
        let keys = self.keyboard_input_tracker.keys.clone();
        for (key_code, key_state) in keys
            .iter()
            .filter(|(_key_code, key_state)| **key_state != KeyState::Up)
        {
            //println!("{:?}", key);
            let dt = ggez::timer::delta(ctx).as_secs_f32();
            let maybe_action_type = self
                .action_processer
                .process_input(self.mode, *key_code, *key_state);
            if let Some(action_type) = maybe_action_type {
                match action_type {
                    ActionType::ChangeMode(m) => {
                        self.mode = m;
                        self.mode_history.register(m);
                        self.mode_visualizer.change(ctx, m);
                        println!("changed mode to: {:?}", m);
                    }
                    ActionType::_PreviousMode => {
                        let maybe_mode = self.mode_history.prev_consume();
                        if let Some(mode) = maybe_mode {
                            self.set_mode(ctx, mode);
                        } else {
                            // command should alwaus be top mode
                            self.mode_history.register(Mode::Command);
                            println!("No mode in history to jump to...");
                        }
                    }
                    ActionType::AddItem(item) => {
                        if item == ItemType::Image {
                            // todo: remove unwrap
                            let image = item::build_image_item(ctx, "/ferris.png".to_string()).unwrap();
                            self.board.item_collection.add(image);
                            self.set_mode(ctx, Mode::Edit);
                            self.current_edit_index =
                                Some(self.board.item_collection.items.len() - 1);
                        }
                    }
                    ActionType::Move(direction) => {
                        self.board.edit_item(
                            dt,
                            direction,
                            is_fast,
                            self.current_edit_index,
                            item::Item::edit_move,
                        );
                    }
                    ActionType::Rotate(direction) => {
                        self.board.edit_item(
                            dt,
                            direction,
                            is_fast,
                            self.current_edit_index,
                            item::Item::rotate,
                        );
                    }
                    ActionType::Scale(direction) => {
                        self.board.edit_item(
                            dt,
                            direction,
                            is_fast,
                            self.current_edit_index,
                            item::Item::scale,
                        );
                    }
                    ActionType::ScaleUniform(direction) => {
                        self.board.edit_item(
                            dt,
                            direction,
                            is_fast,
                            self.current_edit_index,
                            item::Item::scale_uniform,
                        );
                    } //_ => {},
                }
            }
        }
        self.keyboard_input_tracker.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.board.draw(ctx)?;
        self.mode_visualizer.draw(ctx)?;
        ggez::graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            self.keyboard_input_tracker
                .update_key(keycode, KeyState::Pressed);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.keyboard_input_tracker
            .update_key(keycode, KeyState::Up);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(
            ctx,
            ggez::graphics::Rect::new(0.0, 0.0, width, height),
        )
        .unwrap();
    }
}
