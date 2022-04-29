use cat_box::{get_keyboard_state, Game, Sprite, SpriteCollection};
use sdl2::keyboard::Scancode;

fn main() {
    let game = Game::new("Snake", 1000, 1000);
    let snake_boxes: Vec<(i32, i32)> = vec![(13, 13), (14, 13)];

    let mut snake = SpriteCollection::with_capacity(snake_boxes.len());
    for (x, y) in snake_boxes {
        let s = Sprite::new("snakecell.png", x * 37, y * 37).unwrap();
        snake.push(s);
    }

    game.run(|ctx| {
        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            if key == Scancode::Q {
                game.terminate();
            }
        }

        snake.draw(ctx).unwrap();
    })
    .unwrap();
}
