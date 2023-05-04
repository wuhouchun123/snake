use piston_window::WindowSettings;

extern crate piston_window;
extern crate rand;

mod draw;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::snake::Snake;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake", [30 * 25 as u32, 30 * 25 as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        // 监听按键事件
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //
        }

        // 绘制界面
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g); // ???
                                  // game.draw(&c, g);
            let snake = Snake::new(3, 3);
            snake.draw(&c, g);
        });

        // 更新界面
        event.update(|arg| {
            // game.update();
        });
    }
}
