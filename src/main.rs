use cat_box::{get_keyboard_state, Game, Sprite, SpriteCollection};
use sdl2::keyboard::Scancode;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let game = Game::new("Snake", 1000, 1000);
    let snake_boxes: Vec<(i32, i32)> = vec![(13, 13), (14, 13)];

    let mut snake = SpriteCollection::with_capacity(snake_boxes.len());
    for (x, y) in snake_boxes {
        let s = Sprite::new("snakecell.png", x * 37, y * 37).unwrap();
        snake.push(s);
    }

    let mut dir = Direction::Left;

    game.run(|ctx| {
        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            match key {
                Scancode::Q => game.terminate(),
                Scancode::W => dir = Direction::Up,
                Scancode::A => dir = Direction::Left,
                Scancode::S => dir = Direction::Down,
                Scancode::D => dir = Direction::Right,
                _ => (),
            };
        }

        match dir {
            Direction::Up => {
                snake[0].translate((0, 37));
            }
            Direction::Left => {
                snake[0].translate((-37, 0));
            }
            Direction::Down => {
                snake[0].translate((0, -37));
            }
            Direction::Right => {
                snake[0].translate((37, 0));
            }
        };

        {
            let mut last_part = snake[0].position();

            for s in &mut *snake {
                let (lastx, lasty) = last_part;
                let (x, y) = s.position();
                let (xdiff, ydiff) = (lastx - x, y - lasty);
                s.translate((xdiff, ydiff));
                last_part = s.position();
            }
        }

        std::thread::sleep(Duration::from_millis(125));
        snake.draw(ctx).unwrap();
    })
    .unwrap();
}
