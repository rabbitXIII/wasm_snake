#[macro_use]
extern crate stdweb;

use std::cell::RefCell;
use std::rc::Rc;

use std::ops::DerefMut;
use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::IEventTarget;

mod canvas;
mod cell;
mod direction;
mod message;
mod snake;

use canvas::Canvas;
use direction::Direction;
use message::Message;
use snake::Snake;


fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 50, 50);
    let snake = Snake::new(&canvas);


    let input_queue = Rc::new(RefCell::new(Vec::new()));

    stdweb::web::document().add_event_listener({
        let input_queue = input_queue.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => input_queue.borrow_mut().push(Message::new(1, Direction::LEFT)),
                "ArrowRight" => input_queue.borrow_mut().push(Message::new(1, Direction::RIGHT)),
                "ArrowDown" => input_queue.borrow_mut().push(Message::new(1, Direction::DOWN)),
                "ArrowUp" => input_queue.borrow_mut().push(Message::new(1, Direction::UP)),
                _ => {}
            };
        }
    });

    fn game_loop(
        snake: Snake,
        canvas: Rc<Canvas>,
        input_queue: Rc<RefCell<Vec<Message>>>,
        frame_counter: u8
    ) {
        let next_snake =
            snake.next_frame(&canvas, input_queue.borrow_mut().deref_mut(), frame_counter);
        next_snake.draw(&canvas);
        let next_frame_counter = (frame_counter + 1) % 60;
        stdweb::web::window().request_animation_frame(move |_| {game_loop(next_snake, canvas.clone(), input_queue, next_frame_counter)});
    }

    stdweb::web::window().request_animation_frame(move |_| {game_loop(snake, Rc::new(canvas), input_queue, 0u8)});
    stdweb::event_loop();
}
