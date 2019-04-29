use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::lifeless::Member;

pub struct MoveMemberSystem;

impl<'s> System<'s> for MoveMemberSystem {
    type SystemData = (
        ReadStorage<'s, Member>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (members, mut locals, time): Self::SystemData) {
        for (member, local) in (&members, &mut locals).join() {
            local.translate_x(member.velocity[0] * time.delta_seconds());
            local.translate_y(member.velocity[1] * time.delta_seconds());
        }
    }
}