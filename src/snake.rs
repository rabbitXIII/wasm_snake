use crate::stdweb::unstable::TryInto;

use crate::canvas::Canvas;

use crate::cell::Cell;
use crate::direction::Direction;

#[derive(Clone)]
struct Player {
  head: Cell,
  tail: Vec<Cell>,
  direction: Direction,
  should_grow: bool,
}

impl Player {
  pub fn new() -> Player {
    Player {
        head: Cell { x: 5, y: 5 },
        tail: Vec::new(),
        direction: Direction::RIGHT,
        should_grow: false,
      }
  }

  pub fn next(&self, canvas: &Canvas, input_queue: &mut Vec<Direction>) -> Player {
    let mut new_tail = self.tail.clone();
    new_tail.push(self.head);
    if !self.should_grow {
      new_tail.pop();
    }

    let mut next_direction = self.direction;
    while let Some(direction) = input_queue.pop() {
      next_direction = direction;
    }

    let next_head: Cell = match self.direction {
      Direction::DOWN => Cell {
        x: self.head.x,
        y: (self.head.y + 1) % canvas.get_height(),
      },
      Direction::UP => Cell {
        x: self.head.x,
        y: self.head.y.checked_sub(1).unwrap_or(canvas.get_height() - 1),
      },
      Direction::LEFT => Cell {
        x: self.head.x.checked_sub(1).unwrap_or(canvas.get_width() - 1),
        y: self.head.y,
      },
      Direction::RIGHT => Cell {
        x: (self.head.x + 1) % canvas.get_width(),
        y: self.head.y,
      },
    };

    Player {
      head: next_head,
      tail: new_tail,
      direction: next_direction,
      should_grow: false,
    }
  }
}

pub struct Snake {
  food: Cell,
  player: Player,
}

impl Snake {
  pub fn new(canvas: &Canvas) -> Snake {
    Snake {
      food: Snake::create_food(canvas),
      player: Player::new(),
    }
  }

  fn create_food(canvas: &Canvas) -> Cell {
    let random_x: u32 = js! {return Math.floor(Math.random() * @{canvas.get_width()})}
      .try_into()
      .unwrap();
    let random_y: u32 = js! {return Math.floor(Math.random() * @{canvas.get_height()})}
      .try_into()
      .unwrap();
    Cell { x: random_x, y: random_y }
  }

  pub fn next_frame(&self, canvas: &Canvas, input_queue: &mut Vec<Direction>) -> Snake {
    let next_player = self.player.next(&canvas, input_queue);
    let next_food =
      if next_player.head.x == self.food.x && next_player.head.y == self.food.y {
        Snake::create_food(canvas)
      } else {
        self.food
      };
    Snake {
      food: next_food,
      player: next_player,
    }
  }

  pub fn draw(&self, canvas: &Canvas) {
    canvas.clear();
    canvas.draw(self.food, "red");
    canvas.draw(self.player.head, "green");
    for index in 0..self.player.tail.len() {
      canvas.draw(self.player.tail[index], "lightgreen");
    }
  }
}