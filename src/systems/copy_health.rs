use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, Write},
};

use crate::lifeless::{Player, Health, GameState};

pub struct CopySystem;

impl<'s> System<'s> for CopySystem {
    type SystemData = (
        Write<'s, GameState>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Health>,
    );

    fn run(&mut self, (mut game_state, players, health): Self::SystemData) {
        for(_player, health) in (&players, &health).join() {
            game_state.health = health.health;
        }
    }
}