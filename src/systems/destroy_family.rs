use amethyst::{
    core::{timing::Time},
    ecs::prelude::{Join, ReadExpect, System, Read, WriteStorage, ReadStorage, Entities},
    ui::UiText,
};

use crate::lifeless::{Member, Health, ScoreText};

pub struct DestroyFamilySystem {
    pub old_time: u64,
}

impl<'a> System<'a> for DestroyFamilySystem {
    type SystemData = (
        ReadStorage<'a, Member>,
        WriteStorage<'a, UiText>,
        ReadExpect<'a, ScoreText>,
        ReadStorage<'a, Health>,
        Read<'a, Time>,
        Entities<'a>,
    );
    fn run(&mut self, (members, mut ui_text, score_text, healths, time, entities): Self::SystemData) {
        let val = time.absolute_time().as_secs();
        for (_member, health, entity) in (&members, &healths, &*entities).join() {
            if health.health <= 0.0 {
                if let Some(text) = ui_text.get_mut(score_text.message) {
                    text.text = "A family member succumbs!".to_string();
                    self.old_time = time.absolute_time().as_secs();
                }
                entities.delete(entity);
            }
        }
        if val - self.old_time > 3 {
            if let Some(text) = ui_text.get_mut(score_text.message) {
                text.text = "".to_string();
            }
        }
    }
}