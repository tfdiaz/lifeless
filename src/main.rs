extern crate amethyst;
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::input::InputBundle;
use amethyst::ui::{DrawUi, UiBundle};

mod systems;
mod lifeless;
use crate::lifeless::Lifeless;


fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
    .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
    .level_for("gfx_glyph", amethyst::LogLevelFilter::Error)
    .start();
    use amethyst::utils::application_root_dir;
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
    .with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;
    
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::MovePlayerSystem, "player_system", &["input_system"])
        .with(systems::CopySystem, "copy_health", &[])
        .with(systems::DestroyFamilySystem {old_time: 0}, "destroy_system", &[])
        .with(systems::MoveMemberSystem, "member_system", &[])
        .with(systems::MoveLifeEventSystem, "life_event_system", &[])
        .with(systems::BounceMemberSystem, "collision_system", &["member_system"])
        .with(systems::GiveTakeHealthSystem, "give_take_health_system", &["input_system"])
        .with(systems::SpawnLifeEventSystem {counter: 0, timer: 100}, "spawn_life_event", &[])
        .with(systems::TakeDamageSystem, "take_damage_system", &[])
        .with(systems::ScoreSystem {old_time: 0}, "score_system", &[]);
    
    let mut game = Application::new("./", Lifeless, game_data)?;
    game.run();
    Ok(())
}