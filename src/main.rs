use cat_box::{get_keyboard_state, Game, Sprite, SpriteCollection};
use sdl2::keyboard::Scancode;

fn main() {
    let game = Game::new("Snake", 1000, 1000);

    game.run(|ctx| {
        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            if key == Scancode::Q {
                game.terminate();
            }
        }
    })
    .unwrap();
}
