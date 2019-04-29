use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage, Entities},
    renderer::{ SpriteRender, Rgba},
};

extern crate rand;

use rand::prelude::*;

use crate::lifeless::{LifeEvent, ARENA_HEIGHT, ARENA_WIDTH};

pub struct SpawnLifeEventSystem {
    pub counter: u32,
    pub timer: u32,
}

impl<'a> System<'a> for SpawnLifeEventSystem {
    type SystemData = (
        WriteStorage<'a, LifeEvent>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Rgba>,
        Entities<'a>,
    );

    fn run(&mut self, (mut life_events, mut transforms, mut sprite_render, mut color, entities): Self::SystemData) {
        self.counter += 1;
        if self.counter > self.timer {
            let mut sp: Option<SpriteRender> = None;
            let mut rng = rand::thread_rng();
            let sprite_choice = rng.gen_range(4, 8);
            for (_life_event, sprite) in (&life_events, &mut sprite_render).join() {
                sp = Some(SpriteRender {
                sprite_sheet: sprite.sprite_sheet.clone(),
                sprite_number: sprite_choice,
                });
            } 
            let mut transform = Transform::default();
            transform.set_xyz(rng.gen_range(0.0, ARENA_WIDTH), rng.gen_range(0.0, ARENA_HEIGHT), 0.0);
            let arr = [17.0, 20.0, 18.0, 17.0];
            if let Some(sprite) = sp {
                entities.build_entity()
                    .with(transform, &mut transforms)
                    .with(sprite, &mut sprite_render)
                    .with(LifeEvent::new(arr[sprite_choice % 4], 7.0, &mut rng), &mut life_events)
                    .with(Rgba(1.0, 0.0, 0.0, 0.5), &mut color)
                    .build();
            }
            self.counter = 0;
            self.timer -= 1;
            if self.timer < 3 {
                self.timer = 80;
            }
        }
    }
}