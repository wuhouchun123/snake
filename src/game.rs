use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;

pub struct Game {
    snake: Snake,
    food_exist: bool,
    width: i32,
    height: i32,
    food_x: i32,
    food_y: i32,
    waiting_time: f64,
    game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::new(3, 3);
        Game {
            snake,
            food_exist: true,
            width,
            height,
            food_x: 10,
            food_y: 12,
            waiting_time: 0.00,
            game_over: false,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // 绘制蛇
        self.snake.draw(con, g);

        // 绘制墙
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 1, 1, self.height - 2, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 1, 1, self.height - 2, con, g);

        // 绘制食物
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn pressed_key(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Top),
            Key::Down => Some(Direction::Bottom),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        self.update_snake(dir);
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.waiting_time = 0.0;
        } else {
            self.game_over = true;
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);
        next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1
    }
}
