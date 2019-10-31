
use crate::direction::Direction;

#[derive(Clone, Copy)]
pub struct Message {
  player_id: u8,
  direction: Direction,
}

impl Message {
  pub fn new(player_id: u8, direction: Direction) -> Message {
    Message {
      player_id,
      direction
    }
  }
  pub fn get_player_id(self) -> u8 {
    self.player_id
  }

  pub fn get_direction(self) -> Direction {
    self.direction
  }
}