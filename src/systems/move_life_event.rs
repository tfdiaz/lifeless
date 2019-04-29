use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

extern crate rand;
use rand::prelude::*;

use crate::lifeless::{LifeEvent, ARENA_HEIGHT, ARENA_WIDTH};

pub struct MoveLifeEventSystem;

impl<'s> System<'s> for MoveLifeEventSystem {
    type SystemData = (
        ReadStorage<'s, LifeEvent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (life_events, mut locals, time): Self::SystemData) {
        let mut rng = rand::thread_rng();
        for (life_event, local) in (&life_events, &mut locals).join() {
            local.translate_x(life_event.velocity[0] * time.delta_seconds());
            local.translate_y(life_event.velocity[1] * time.delta_seconds());
            let x_coord = local.translation().x;
            let y_coord = local.translation().y;

            if x_coord < -20.0 || x_coord > 20.0 + ARENA_WIDTH
                || y_coord < -20.0 || y_coord > 20.0 + ARENA_HEIGHT {
                    local.translate_x(rng.gen_range(0.0, ARENA_WIDTH));
                    local.translate_y(rng.gen_range(0.0, ARENA_HEIGHT));
                }
        }
    }
}