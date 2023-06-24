use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
  Up,
  Right,
  Down,
  Left
}

struct SnakeCell(usize);
struct Snake {
  body: Vec<SnakeCell>,
  direction: Direction
}

impl Snake {
  fn new(spawn_index: usize) -> Snake {
    Snake {
      body: vec!(SnakeCell(spawn_index)),
      direction: Direction::Right
    }
  }
}

#[wasm_bindgen]
pub struct World {
  width: usize,
  size: usize,
  snake: Snake
}

#[wasm_bindgen]
impl World {
  pub fn new(width: usize, snake_idx: usize) -> World {    
    World {
      width,
      size: width * width,
      snake: Snake::new(snake_idx)
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn snake_head_idx(&self) -> usize {
    self.snake.body[0].0
  }

  pub fn update(&mut self) {
    let snake_idx = self.snake_head_idx(); 

    let (row, col) = (snake_idx / self.width, snake_idx % self.width);
    let (row, col) = match self.snake.direction {
      Direction::Right => {
        (row, (col + 1) % self.width)
      },
      Direction::Left => {
        (row, (col - 1) % self.width)
      },
      Direction::Up => {
        ((row - 1) & self.width, col)
      },
      Direction::Down => {
        ((row + 1) & self.width, col)
      }
    };

    self.snake.body[0].0 = (row * self.width) + col;
  }
}

// wasm-pack build --target web ---> targetting JS