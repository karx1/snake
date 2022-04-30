use cat_box::{get_keyboard_state, Game, Sprite, SpriteCollection};
use rand::thread_rng;
use rand::Rng;
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

    let mut apple = {
        let x = thread_rng().gen_range(0..=27);
        let y = thread_rng().gen_range(0..=27);

        Sprite::new("apple.png", x * 37, y * 37).unwrap()
    };

    let mut dir = Direction::Left;

    game.run(|ctx| {
        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            match key {
                Scancode::Q => game.terminate(),
                Scancode::W | Scancode::Up => dir = Direction::Up,
                Scancode::A | Scancode::Left => dir = Direction::Left,
                Scancode::S | Scancode::Down => dir = Direction::Down,
                Scancode::D | Scancode::Right => dir = Direction::Right,
                _ => (),
            };
        }

        {
            let mut last_part = snake[0].position();

            for s in snake.iter().skip(1) {
                let (lastx, lasty) = last_part;
                let (x, y) = s.position();
                let (xdiff, ydiff) = (lastx - x, y - lasty);
                s.translate((xdiff, ydiff));
                last_part = s.position();
            }
        }

        // The snake head needs to be moved after the body or else the body will just collapse into the head
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

        if cat_box::physics::check_for_collision(&snake[0], &apple) {
            let x = thread_rng().gen_range(0..=27) * 37;
            let y = thread_rng().gen_range(0..=27) * 37;

            let (currx, curry) = apple.position();
            let (xdiff, ydiff) = (x - currx, curry - y);
            apple.translate((xdiff, ydiff));
        }

        // So that the snake doesn't move at super speed
        std::thread::sleep(Duration::from_millis(125));
        apple.draw(ctx).unwrap();
        snake.draw(ctx).unwrap();
    })
    .unwrap();
}
