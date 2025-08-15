use macroquad::input::KeyCode;
use macroquad::input::is_key_pressed;
use macroquad::time::get_frame_time;

use crate::common::Direction;
use crate::common::GRID_SIZE_IN_PIXELS;
use crate::common::Position;
use crate::common::Positioned;
use crate::common::get_random_direction;
use crate::common::get_random_position;
use crate::common::intersects;
use crate::food::Food;

#[derive(Debug)]
pub struct Snake {
    pub head: Position,
    pub size: f32,
    pub tail: Vec<TailPart>,
    pub is_alive: bool,
    direction: Direction,
    speed: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct TailPart {
    pub position: Position,
    pub size: f32,
}

impl Positioned for TailPart {
    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_size(&self) -> f32 {
        self.size
    }
}

impl Snake {
    pub fn new(position: Position, direction: Direction) -> Self {
        return Self {
            head: position,
            tail: vec![TailPart {
                position,
                size: 20.0,
            }],
            is_alive: true,
            direction,
            speed: 200.0, // pixels per second
            size: 20.0,
        };
    }

    pub fn maybe_eat_food(&mut self, food: &mut Food) {
        let is_being_eaten = intersects(self, food);

        if is_being_eaten && !food.is_eaten {
            food.is_eaten = is_being_eaten;
            let last_position = *self.tail.last().unwrap();
            self.tail.push(last_position);
        }
    }

    pub fn capture_input(&mut self) {
        const DIRECTIONS: &[(KeyCode, Direction)] = &[
            (KeyCode::Up, Direction::Up),
            (KeyCode::Down, Direction::Down),
            (KeyCode::Left, Direction::Left),
            (KeyCode::Right, Direction::Right),
        ];

        for (key, direction) in DIRECTIONS.iter() {
            if is_key_pressed(*key) && !is_opposite_directions(&self.direction, direction) {
                self.direction = *direction;
                break;
            }
        }
    }

    pub fn update(&mut self) {
        // remember where we were before updating
        let prev_position = self.head.clone();

        // perform updates
        self.set_direction();
        self.maybe_teleport();
        self.move_tail();
        self.maybe_kill_snake();

        // move the previous position of the head to the body
        self.tail[0] = TailPart {
            position: prev_position,
            size: self.size,
        }
    }

    /// Set the direction based on the keyboard input.
    fn set_direction(&mut self) {
        let delta = get_frame_time();
        match self.direction {
            Direction::Up => self.head.y -= self.speed * delta,
            Direction::Down => self.head.y += self.speed * delta,
            Direction::Left => self.head.x -= self.speed * delta,
            Direction::Right => self.head.x += self.speed * delta,
        }
    }

    /// Teleports the snake to the other side of the canvas when it travels over the edge.
    fn maybe_teleport(&mut self) {
        match (
            // x-axis
            self.head.x < 0.0,
            self.head.x >= GRID_SIZE_IN_PIXELS,
            // y-axis
            self.head.y < 0.0,
            self.head.y >= GRID_SIZE_IN_PIXELS,
        ) {
            // x-axis
            (true, _, _, _) => self.head.x = GRID_SIZE_IN_PIXELS - self.size,
            (_, true, _, _) => self.head.x = 0.0,
            // y-axis
            (_, _, true, _) => self.head.y = GRID_SIZE_IN_PIXELS - self.size,
            (_, _, _, true) => self.head.y = 0.0,
            _ => {}
        }
    }

    /// Move the entire tail up one position (except for the first block).
    fn move_tail(&mut self) {
        for i in (1..self.tail.len()).rev() {
            self.tail[i] = self.tail[i - 1];
        }
    }

    /// Kills the snake if it collides with itself
    fn maybe_kill_snake(&mut self) {
        const SKIP: usize = 15; // small buffer so the snake doesn't eat itself immediately

        for i in SKIP..self.tail.len() {
            if intersects(self, &self.tail[i]) {
                self.is_alive = false;
            }
        }
    }
}

impl Positioned for Snake {
    fn get_position(&self) -> &Position {
        &self.head
    }

    fn get_size(&self) -> f32 {
        self.size
    }
}

fn is_opposite_directions(a: &Direction, b: &Direction) -> bool {
    return a == get_opposite_direction(b);
}

fn get_opposite_direction(direction: &Direction) -> &Direction {
    match direction {
        Direction::Up => &Direction::Down,
        Direction::Down => &Direction::Up,
        Direction::Left => &Direction::Right,
        Direction::Right => &Direction::Left,
    }
}

pub fn random_snake() -> Snake {
    return Snake::new(get_random_position(), get_random_direction());
}
