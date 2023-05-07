use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0; // 重新开始游戏的时间

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

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }

        if !self.food_exist {
            self.add_food();
        }
    }

    pub fn pressed_key(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Top),
            Key::Down => Some(Direction::Bottom),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    // 是否蛇还活着
    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);

        // 根据下一个蛇头的位置，确定头部是否碰到身体
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            // 蛇移动
            self.snake.move_forward(dir);
            // 检查吃
            self.check_eating();
        } else {
            self.game_over = true;
        };

        self.waiting_time = 0.0;
    }

    // 检查是否迟到了food
    fn check_eating(&mut self) {
        let (next_x, next_y): (i32, i32) = self.snake.head_position();
        if next_x == self.food_x && next_y == self.food_y {
            self.food_exist = false;
            self.snake.restore_tail(self.food_x, self.food_y);
        }
    }

    // 重新开始
    fn restart(&mut self) {
        self.food_exist = true;
        self.food_x = 10;
        self.food_y = 12;
        self.waiting_time = 0.0;
        self.snake = Snake::new(3, 3);
        self.game_over = false;
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_food_x = rng.gen_range(1..self.width - 1);
        let mut new_food_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_food_x, new_food_y) {
            new_food_x = rng.gen_range(1..self.width - 1);
            new_food_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_food_x;
        self.food_y = new_food_y;
        self.food_exist = true;
    }
}
