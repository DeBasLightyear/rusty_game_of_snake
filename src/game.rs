use macroquad::{
    color::{BLACK, Color, GREEN, PINK, RED},
    input::KeyCode,
    window::{Conf, clear_background, next_frame},
};

use crate::{
    common::{BLOCK_SIZE, GRID_SIZE, Position, Positioned},
    food::{Food, random_food},
    snake::{Snake, random_snake},
};

/// Basic window configuration.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_string(),
        window_width: GRID_SIZE * BLOCK_SIZE,
        window_height: GRID_SIZE * BLOCK_SIZE,
        ..Default::default()
    }
}

/// Start the game.
pub async fn game_loop() {
    let mut snake = random_snake();
    let mut foods = get_a_bunch_food();

    loop {
        clear_background(BLACK);

        if snake.is_alive {
            update_snake(&mut snake);
            update_foods(&mut foods, &mut snake);
        } else {
            display_game_over_screen(&snake);
            if macroquad::input::is_key_pressed(KeyCode::Space) {
                snake = random_snake();
                foods = get_a_bunch_food();
            }
        }

        next_frame().await
    }
}

fn get_a_bunch_food() -> Vec<Food> {
    return (0..5).map(|_| random_food()).collect();
}

fn update_snake(snake: &mut Snake) {
    snake.capture_input();
    snake.update();
    draw_entity(snake, PINK);

    for part in &snake.tail {
        draw(part.get_position(), snake.size, GREEN);
    }
}

fn update_foods(foods: &mut Vec<Food>, snake: &mut Snake) {
    let previous_nr_of_foods = foods.len();
    foods.retain(|food| !food.is_eaten);

    if previous_nr_of_foods > foods.len() {
        foods.push(random_food());
    }

    for food in foods {
        snake.maybe_eat_food(food);
        draw_entity(food, RED);
    }
}

/// Draws any strcut that implements the `Positioned` interface.
fn draw_entity<T: Positioned>(item: &T, color: Color) {
    let position = item.get_position();
    let size = item.get_size();
    draw(position, size, color);
}

/// Draws something at the given position.
fn draw(position: &Position, size: f32, color: Color) {
    macroquad::shapes::draw_rectangle(position.x as f32, position.y as f32, size, size, color);
}

/// Display a game over screen with the final score.
fn display_game_over_screen(snake: &Snake) {
    let screen_w = macroquad::window::screen_width();
    let screen_h = macroquad::window::screen_height();

    // let text_params = ;
    macroquad::text::draw_text_ex(
        "GAME OVER",
        screen_w / 2.0 - 100.0,
        screen_h / 2.0 - 30.0,
        macroquad::text::TextParams {
            font_size: 40,
            ..Default::default()
        },
    );

    macroquad::text::draw_text_ex(
        &format!("Score: {}", snake.tail.len()),
        screen_w / 2.0 - 50.0,
        screen_h / 2.0,
        macroquad::text::TextParams {
            font_size: 30,
            ..Default::default()
        },
    );

    macroquad::text::draw_text_ex(
        "Press SPACE to restart",
        screen_w / 2.0 - 160.0,
        screen_h / 2.0 + 20.0,
        macroquad::text::TextParams {
            font_size: 30,
            ..Default::default()
        },
    );
}
