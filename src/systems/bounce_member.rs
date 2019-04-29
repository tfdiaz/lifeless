use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::lifeless::{Member, ARENA_HEIGHT, ARENA_WIDTH};

pub struct BounceMemberSystem;

impl<'s> System<'s> for BounceMemberSystem {
    type SystemData = (
        WriteStorage<'s, Member>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut members, transform): Self::SystemData) {
        for (member, transform) in (&mut members, &transform).join() {
            let member_x = transform.translation().x;
            let member_y = transform.translation().y;

            if member_y >= ARENA_HEIGHT - member.height * 0.5 && member.velocity[1] >0.0 {
                member.velocity[1] = -member.velocity[1];
            }
            else if member_y <= member.height * 0.5 && member.velocity[1] < 0.0 {
                member.velocity[1] = -member.velocity[1];
            }
            if member_x >= ARENA_WIDTH - member.width * 0.5 && member.velocity[0] >0.0 {
                member.velocity[0] = -member.velocity[0];
            }
            else if member_x <= member.width * 0.5 && member.velocity[0] < 0.0 {
                member.velocity[0] = -member.velocity[0];
            }
        }
    }
}