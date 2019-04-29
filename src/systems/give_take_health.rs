use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::InputHandler,
    renderer::Rgba,
};

use crate::lifeless::{Player, Member, Health, GameState, PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct GiveTakeHealthSystem;

impl<'s> System<'s> for GiveTakeHealthSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Member>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, Rgba>,
        Read<'s, InputHandler<String, String>>,
        Write<'s, GameState>,
    );

    fn run(&mut self, (player, members, transforms, mut healths, mut rgbas, input, mut game_state): Self::SystemData) {
        let mut x_coord: f32 = 0.0;
        let mut y_coord: f32 = 0.0;
        let mut tank: f32 = 0.0;

        let take = input.action_is_down("take");
        let give = input.action_is_down("give");
        for (_player, transform) in (&player, &transforms).join() {
            x_coord = transform.translation().x;
            y_coord = transform.translation().y;
        }
        if let Some(take_it) = take {
            if take_it {
            for (_member, transform, health, color) in (&members, &transforms, &mut healths, &mut rgbas).join() {
                let member_x = transform.translation().x;
                let member_y = transform.translation().y;
                if collide(x_coord, y_coord, member_x, member_y) {
                    health.health -= 1.0;
                    tank += 1.0;
                    if health.health < 0.0 {
                        tank += health.health;
                        health.health = 0.0;
                    }
                    health.red -= 0.01;
                    health.green -= 0.01;
                    health.blue -= 0.01;
                    if health.red < 0.0 {
                        health.red = 0.0;
                    }
                    if health.green < 0.0 {
                        health.green = 0.0;
                    }
                    if health.blue < 0.0 {
                        health.blue = 0.0;
                    }
                    color.0 = health.red;
                    color.1 = health.green;
                    color.2 = health.blue;
                }
            }}
        }
        if let Some(give_it) = give {
            if give_it {
            for (_member, transform, health, color) in (&members, &transforms, &mut healths, &mut rgbas).join() {            
                let member_x = transform.translation().x;
                let member_y = transform.translation().y;

                if collide(x_coord, y_coord, member_x, member_y) {
                    health.health += 1.0;
                    tank -= 1.0;
                    if health.health < 0.0 {
                        tank += health.health;
                        health.health = 0.0;
                    }
                    health.red += 0.01;
                    health.green += 0.01;
                    health.blue += 0.01;
                    if health.red > 1.0 {
                        health.red = 1.0;
                    }
                    if health.green > 1.0 {
                        health.green = 1.0;
                    }
                    if health.blue > 1.0 {
                        health.blue = 1.0;
                    }
                    color.0 = health.red;
                    color.1 = health.green;
                    color.2 = health.blue;
                }
            }
        }}
        if tank != 0.0 {
            for (_player, health, color) in (&player, &mut healths, &mut rgbas).join() {
                health.health += tank;
                if tank > 0.0 {
                    health.red += 0.01;
                    health.green += 0.01;
                    health.blue += 0.01;
                }
                else if tank < 0.0 {
                    health.red -= 0.01;
                    health.green -= 0.01;
                    health.blue -= 0.01;
                }
                game_state.health = health.health;
                color.0 = health.red;
                color.1 = health.green;
                color.2 = health.blue;
            }
        }
    }
}

fn collide (pos_x: f32, pos_y: f32, check_x: f32, check_y: f32) -> bool {
    if pos_x >= check_x  && pos_x <= check_x + PLAYER_WIDTH && pos_y >= check_y && pos_y <= check_y + PLAYER_HEIGHT {
        true
    }
    else {
        false
    }
}