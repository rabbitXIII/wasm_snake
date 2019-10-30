#[derive(Clone,Copy, PartialEq)]
pub enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

impl Direction {
  pub fn opposite(self) -> Direction {
    return match self {
      Direction::DOWN => {
        Direction::UP
      },
      Direction::UP => {
        Direction::DOWN
      },
      Direction::RIGHT => {
        Direction::LEFT
      },
      Direction::LEFT => {
        Direction::RIGHT
      }
    }
  }
}