use rand::Rng;

pub const GRID_SIZE: i32 = 20;
pub const BLOCK_SIZE: i32 = 20;
pub const GRID_SIZE_IN_PIXELS: f32 = (GRID_SIZE * BLOCK_SIZE) as f32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub trait Positioned {
    fn get_position(&self) -> &Position;
    fn get_size(&self) -> f32;
}

pub fn get_random_direction() -> Direction {
    let mut rng = rand::rng();
    return DIRECTIONS[rng.random_range(0..DIRECTIONS.len())];
}

pub fn get_random_position() -> Position {
    let range = GRID_SIZE * BLOCK_SIZE;
    let x = rand::rng().random_range(0..range) as f32;
    let y = rand::rng().random_range(0..range) as f32;
    return Position { x, y };
}

pub fn intersects<T: Positioned, U: Positioned>(a: &T, b: &U) -> bool {
    let half_snake = a.get_size() / 2.0;
    let half_food = b.get_size() / 2.0;

    let left_snake = a.get_position().x - half_snake;
    let right_snake = a.get_position().x + half_snake;
    let top_snake = a.get_position().y - half_snake;
    let bottom_snake = a.get_position().y + half_snake;

    let left_food = b.get_position().x - half_food;
    let right_food = b.get_position().x + half_food;
    let top_food = b.get_position().y - half_food;
    let bottom_food = b.get_position().y + half_food;

    return !(left_snake > right_food
        || right_snake < left_food
        || top_snake > bottom_food
        || bottom_snake < top_food);
}
