//!
//!  main.rs
//!
//!  Created by Mitchell Nordine at 03:57PM on July 27, 2014.
//!
//!

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate nui;

use app::App;
use sdl2_game_window::GameWindowSDL2;
use piston::{
    Game,
    GameWindowSettings,
    GameIteratorSettings,
};

mod app;

fn main() {
    // Create a SDL2 window.
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Hello Piston".to_string(),
            size: [800, 800],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    // Some settings for how the game should be run.
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 180,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut app = App::new();
    app.run(&mut window, &game_iter_settings);
}

