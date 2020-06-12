use ggez;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::env;
use std::path;

mod action_processer;
mod action_processer_utils;
mod action_type;
mod app;
mod board;
mod constants;
mod direction;
mod edit_type;
mod graphics;
mod input_action;
mod item;
mod item_collection;
mod key_state;
mod keyboard_input_tracker;
mod mode;
mod mode_history;

use app::App;

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
