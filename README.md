# wasm_snake
Silly game of snake made using rust and stdweb to compile into webassembly.  I no nothing about front-end development or rust, so read these files with a grain of salt..

![Snake Game made with Rust/WASM](https://raw.githubusercontent.com/rabbitXIII/wasm_snake/master/static/wasm_snake.png)


## About

The game listens for key press events and adds these to an input queue.  Every animation frame, the input queue is drained and the last-pressed direction is used to set the new direction for snake.  

Presses in the opposite direction from the current are ignored, since that's an auto-death.

## Install
```sh
$ cargo install -f cargo-web
```

## Run
```sh
$ cargo web start --target=wasm32-unknown-unknown
```

