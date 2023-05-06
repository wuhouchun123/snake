use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Clone, Copy)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    // 初始化蛇
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // 绘制蛇
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // 蛇头部的位置
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // 蛇移动
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => {}
        };
        let (head_x, head_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Bottom => Block {
                x: head_x,
                y: head_y + 1,
            },
            Direction::Top => Block {
                x: head_x,
                y: head_y - 1,
            },
            Direction::Left => Block {
                x: head_x - 1,
                y: head_y,
            },
            Direction::Right => Block {
                x: head_x + 1,
                y: head_y,
            },
        };
        // 头部添加new_block
        self.body.push_front(new_block);

        // 尾部删除block
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block); //???
        
    }

    // 下一个头部的位置
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match self.direction {
            Direction::Bottom => (head_x, head_y + 1),
            Direction::Top => (head_x, head_y - 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
}
