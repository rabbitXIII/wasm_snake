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
mod snake;

use canvas::Canvas;
use direction::Direction;
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
                "ArrowLeft" => input_queue.borrow_mut().push(Direction::LEFT),
                "ArrowRight" => input_queue.borrow_mut().push(Direction::RIGHT),
                "ArrowDown" => input_queue.borrow_mut().push(Direction::DOWN),
                "ArrowUp" => input_queue.borrow_mut().push(Direction::UP),
                _ => {}
            };
        }
    });

    fn game_loop(
        snake: Snake,
        canvas: Rc<Canvas>,
        input_queue: Rc<RefCell<Vec<Direction>>>,
        timeout: u32,
    ) {
        stdweb::web::set_timeout(
            move || {
                let new_snake =
                    snake.next_frame(&canvas, input_queue.borrow_mut().deref_mut());
                new_snake.draw(&canvas);
                game_loop(new_snake, canvas.clone(), input_queue, timeout);
            },
            timeout,
        )
    }
    game_loop(snake, Rc::new(canvas), input_queue, 17);
    stdweb::event_loop();
}
