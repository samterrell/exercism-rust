// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

pub struct Robot {
    d: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { d, x, y }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            North => Self { d: East, ..self },
            East => Self { d: South, ..self },
            South => Self { d: West, ..self },
            West => Self { d: North, ..self },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            North => Self { d: West, ..self },
            East => Self { d: North, ..self },
            South => Self { d: East, ..self },
            West => Self { d: South, ..self },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let Self { d, x, y } = self;
        match d {
            North => Self { d, x, y: y + 1 },
            East => Self { d, x: x + 1, y },
            South => Self { d, x, y: y - 1 },
            West => Self { d, x: x - 1, y },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |state, c| match c {
            'R' => state.turn_right(),
            'L' => state.turn_left(),
            'A' => state.advance(),
            _ => state,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
