use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage, Entities},
    renderer::Rgba,
};

use crate::lifeless::{LifeEvent, Member, Player, Health, PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct TakeDamageSystem;

impl<'a> System<'a> for TakeDamageSystem {
    type SystemData = (
        ReadStorage<'a, LifeEvent>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Member>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Rgba>,
        WriteStorage<'a, Health>,
        Entities<'a>,
    );

    fn run(&mut self, (life_events, transforms, members, players, mut color, mut healths, entities): Self::SystemData) {

        for (_member, transform, color, health) in (&members, &transforms, &mut color, &mut healths).join() {
            let member_x = transform.translation().x;
            let member_y = transform.translation().y;
            for (life_event, transform, entity) in (&life_events, &transforms, &*entities).join() {
                let life_x = transform.translation().x;
                let life_y = transform.translation().y;
                if life_x >= member_x - life_event.width &&
                    life_x <= member_x + PLAYER_WIDTH &&
                    life_y <= member_y + PLAYER_HEIGHT * 0.5 &&
                    life_y >= member_y - PLAYER_HEIGHT * 0.5 {
                        health.health -= 10.0;
                        health.red -= 0.1;
                        health.green -= 0.1;
                        health.blue -= 0.1;
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
                        entities.delete(entity);
                }
            }
        }
        for (_player, transform, color, health) in (&players, &transforms, &mut color, &mut healths).join() {
            let player_x = transform.translation().x;
            let player_y = transform.translation().y;
            for (life_event, transform, entity) in (&life_events, &transforms, &*entities).join() {
                let life_x = transform.translation().x;
                let life_y = transform.translation().y;
                if life_x >= player_x - life_event.width &&
                    life_x <= player_x + PLAYER_WIDTH &&
                    life_y <= player_y + PLAYER_HEIGHT * 0.5 &&
                    life_y >= player_y - PLAYER_HEIGHT * 0.5 {
                        health.health -= 10.0;
                        health.red -= 0.1;
                        health.green -= 0.1;
                        health.blue -= 0.1;
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
                        entities.delete(entity);
                }
            }
        }
    }
}