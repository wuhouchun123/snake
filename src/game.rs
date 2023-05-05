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
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            self.snake.move_forward(Some(Direction::Right));
            self.waiting_time = 0.00;
        }
    }

    pub fn pressed_key(&self) {}
}
