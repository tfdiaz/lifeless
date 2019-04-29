use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::lifeless::{Player, ARENA_HEIGHT, ARENA_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (transform, _player) in (&mut transforms, &players).join() {
            let updown = input.axis_value("updown");
            let leftright = input.axis_value("leftright");
            if let Some(y_amount) = updown {
                let scaled_amount = 1.2 * y_amount as f32;
                let player_y = transform.translation().y;
                transform.set_y(
                    (player_y + scaled_amount)
                        .min(ARENA_HEIGHT - PLAYER_HEIGHT * 0.5)
                        .max(PLAYER_HEIGHT * 0.5),
                );
            }
            if let Some(x_amount) = leftright {
                let scaled_amount = 1.2 * x_amount as f32;
                let player_x = transform.translation().x;
                transform.set_x(
                    (player_x + scaled_amount)
                        .min(ARENA_WIDTH - PLAYER_WIDTH * 0.5)
                        .max(PLAYER_WIDTH * 0.5),
                );
            }
        }
    }
}