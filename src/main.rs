use cat_box::{draw_text, get_keyboard_state, Game, Sprite, SpriteCollection};
use rand::thread_rng;
use rand::Rng;
use sdl2::keyboard::Scancode;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

macro_rules! set_if_not_opp {
    ($i:ident, $e:expr, $opp:expr) => {
        if $i != $opp {
            $i = $e
        }
    };
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

    let mut score = 0u64;

    game.run(|ctx| {
        draw_text(
            ctx,
            format!("Score: {}", score),
            "ibm_bios-2y.ttf",
            36,
            (100, 100),
            cat_box::TextMode::Transparent {
                colour: (255, 255, 255),
            },
        )
        .unwrap();

        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            use Direction::*;
            match key {
                Scancode::Q | Scancode::Escape => {
                    println!("Game over!");
                    println!("Your score was: {}", score);
                    game.terminate();
                }
                Scancode::W | Scancode::Up => set_if_not_opp!(dir, Up, Down),
                Scancode::A | Scancode::Left => set_if_not_opp!(dir, Left, Right),
                Scancode::S | Scancode::Down => set_if_not_opp!(dir, Down, Up),
                Scancode::D | Scancode::Right => set_if_not_opp!(dir, Right, Left),
                _ => (),
            };
        }

        {
            let mut last_part = snake[0].position();

            for s in snake.iter().skip(1) {
                let (lastx, lasty) = last_part;
                let (x, y) = s.position();
                let (xdiff, ydiff) = (lastx - x, y - lasty);
                last_part = s.position();
                s.translate((xdiff, ydiff));
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

        {
            let hitted = cat_box::physics::check_for_collision_with_collection(&snake[0], &snake);
            if hitted.len() > 1 {
                println!("Game over!");
                println!("Your score was: {}", score);
                game.terminate();
            }
        }

        if !cat_box::physics::check_for_collision_with_collection(&apple, &snake).is_empty() {
            let x = thread_rng().gen_range(0..=27) * 37;
            let y = thread_rng().gen_range(0..=27) * 37;

            let (currx, curry) = apple.position();
            let (xdiff, ydiff) = (x - currx, curry - y);
            apple.translate((xdiff, ydiff));
            let second_to_last = snake[snake.len() - 2].position();
            let last = snake[snake.len() - 1].position();

            let direc = check_direction(last, second_to_last);

            let (newx, newy) = match direc {
                Direction::Left => (last.0 - 37, last.1),
                Direction::Right => (last.0 + 37, last.1),
                Direction::Up => (last.0, last.1 - 37),
                Direction::Down => (last.0, last.1 + 37),
            };

            let s = Sprite::new("snakecell.png", newx, newy).unwrap();
            snake.push(s);

            score += 1;
        }

        {
            let (mut x, mut y) = snake[0].position();
            x /= 37;
            y /= 37;

            if dir == Direction::Left || dir == Direction::Right {
                if x <= 0 {
                    let diff = (27 * 37) - (x * 37);
                    snake[0].translate((diff, 0));
                } else if x >= 27 {
                    let diff = 0 - (x * 37);
                    snake[0].translate((diff, 0));
                }
            }

            if dir == Direction::Up || dir == Direction::Down {
                if y <= 0 {
                    let diff = (y * 37) - (27 * 37);
                    snake[0].translate((0, diff));
                } else if y >= 27 {
                    snake[0].translate((0, (y * 37)));
                }
            }
        }

        // So that the snake doesn't move at super speed
        std::thread::sleep(Duration::from_millis(125));
        apple.draw(ctx).unwrap();
        snake.draw(ctx).unwrap();
    })
    .unwrap();
}

fn check_direction(square1: (i32, i32), square2: (i32, i32)) -> Direction {
    if square1.0 < square2.0 {
        return Direction::Left;
    } else if square1.0 > square2.0 {
        return Direction::Right;
    };
    if square1.1 > square2.1 {
        return Direction::Up;
    } else if square1.1 < square2.1 {
        return Direction::Down;
    };

    unreachable!(
        "why are we still here? just to suffer? (this should never ever ever happen, by the way)"
    );
}
