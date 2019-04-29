use amethyst::{
    core::{timing::Time},
    ecs::prelude::{Join, Write, ReadExpect, System, Read, WriteStorage, ReadStorage},
    ui::UiText,
};

use crate::lifeless::{Member, ScoreText, GameState};

pub struct ScoreSystem {
    pub old_time: u64,
}

impl<'s> System <'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Member>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        Read<'s, Time>,
        Write<'s, GameState>,
    );

    fn run(&mut self, (members, mut ui_text, score_text, time, mut game_state): Self::SystemData) {
        let mut count = 1;
        for _member in (&members).join() {
            count += 1;
        }
        let mut total = game_state.score;
        let val = time.absolute_time().as_millis() as u64;
        if val - self.old_time > 300 {
            total += count;
            self.old_time = val;
        }
        if let Some(text) = ui_text.get_mut(score_text.score) {
            let st_score = total.to_string();
            let mut s = "Score: ".to_string();
            s.push_str(&st_score);
            text.text = s;
            game_state.score = total;
        }
    }
}