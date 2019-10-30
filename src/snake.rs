use crate::stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::HtmlElement;
use crate::stdweb::web::{INode, INonElementParentNode};

use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::direction::Direction;
use crate::message::Message;


#[derive(Clone)]
enum Color {
  GREEN,
  BLUE,
  RED,
}

#[derive(Clone)]
struct Player {
  id: u8,
  head: Cell,
  tail: Vec<Cell>,
  direction: Direction,
  should_grow: bool,
  score: usize,
  color: Color,
}

impl Player {
  pub fn new(id: u8, color: Color) -> Player {
    Player {
        id,
        head: Cell { x: 5, y: 5 },
        tail: Vec::new(),
        direction: Direction::RIGHT,
        should_grow: true,
        score: 0,
        color: color,
      }
  }

  pub fn next(&self, canvas: &Canvas, input_queue: &mut Vec<Message>) -> Player {
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

    if self.tail.clone().into_iter().any(|cell| cell.eq(&self.head)) {
      return Player::new(self.id, Color::GREEN)
    }

    let mut new_tail = self.tail.clone();
    new_tail.insert(0, self.head);
    if !self.should_grow {
      new_tail.pop();
    }

    let mut next_direction = self.direction;
    for index in 0..input_queue.len() {
      let message = input_queue.get(index);
      if message.is_some() && message.unwrap().get_player_id() == self.id {
        let player_message = input_queue.remove(index);
        if player_message.get_direction().opposite() != self.direction {
          next_direction = player_message.get_direction();
        }
      }
    }

    let score = new_tail.len() - 1;

    Player {
      id: self.id,
      head: next_head,
      tail: new_tail,
      direction: next_direction,
      should_grow: false,
      score: score,
      color: self.color.clone(),
    }
  }

  pub fn grow(&self) -> Player {
    Player {
      id: self.id,
      head: self.head,
      tail: self.tail.clone(),
      direction: self.direction,
      should_grow: true,
      score: self.score,
      color: self.color.clone(),
    }
  }
}

pub struct Snake {
  food: Cell,
  players: Vec<Player>,
}

impl Snake {
  pub fn new(canvas: &Canvas) -> Snake {
    Snake {
      food: Snake::create_food(canvas),
      players: vec![Player::new(1, Color::GREEN)],
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

  pub fn next_frame(&self, canvas: &Canvas, input_queue: &mut Vec<Message>, frame_counter: u8) -> Snake {
    let length = input_queue.len() as i32;
    js! {console.log(@{length})};

    let calculated_player = if frame_counter % 5 == 0 {
      self.players[0].next(&canvas, input_queue)
    } else {
      self.players[0].clone()
    };
    let next_player;
    let next_food =
      if calculated_player.head.x == self.food.x && calculated_player.head.y == self.food.y {
        next_player = calculated_player.grow();
        Snake::create_food(canvas)
      } else {
        next_player = calculated_player;
        self.food
      };
    Snake {
      food: next_food,
      players: vec![next_player],
    }
  }

  pub fn draw(&self, canvas: &Canvas) {
    canvas.clear();
    canvas.draw(self.food, "red");
    for player_id in 0..self.players.len() {
      let current_player = &self.players[player_id];
      for index in 0..current_player.tail.len() {
        canvas.draw(current_player.tail[index], "lightgreen");
      }
      canvas.draw(current_player.head, "green");
      let output_div: HtmlElement = document().get_element_by_id("score").unwrap().try_into().unwrap();
      output_div.set_text_content(format!("Player {} Score: {}", current_player.id, current_player.score).as_str());
    }
  }
}