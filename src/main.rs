/**
* Copyright (c) AWildDevAppears
*/
mod gamestate;
mod models;

use raylib::{drawing::RaylibDraw, ffi::Color};

use crate::gamestate::GameState;

fn main() {
    let mut state = GameState::new();

    let (mut game, thread) = raylib::init()
        .size(state.screen_width, state.screen_height)
        .title(state.game_name.as_str())
        .build();

    state.preload(&mut game, &thread);

    while !game.window_should_close() {
        state.update(&mut game);

        let mut draw = game.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);

        state.draw(&mut draw);
    }
}
