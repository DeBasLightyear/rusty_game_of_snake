mod common;
mod food;
mod game;
mod snake;

use crate::game::{game_loop, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    game_loop().await;
}
