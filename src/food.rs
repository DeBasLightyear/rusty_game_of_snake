use crate::common::{Position, Positioned, get_random_position};

#[derive(Debug)]
pub struct Food {
    pub position: Position,
    pub size: f32,
    pub is_eaten: bool,
}

impl Food {
    pub fn new(position: Position) -> Self {
        return Self {
            position,
            size: 10.0,
            is_eaten: false,
        };
    }
}

impl Positioned for Food {
    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_size(&self) -> f32 {
        self.size
    }
}

pub fn random_food() -> Food {
    return Food::new(get_random_position());
}
