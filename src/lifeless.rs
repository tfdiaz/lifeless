use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::winit::VirtualKeyCode;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::ecs::World;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, Rgba,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

extern crate rand;

use rand::prelude::*;

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub const PLAYER_HEIGHT: f32 = 32.0;
pub const PLAYER_WIDTH: f32 = 26.0;

pub struct Lifeless;

#[derive(Clone, Default)]
pub struct GameState {
    pub health: f32,
    pub score: u32,
}

pub struct Player {
    pub width: f32,
    pub height: f32
}

pub struct Member {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

pub struct Health {
    pub health: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct LifeEvent {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

pub struct ScoreText {
    pub score: Entity,
    pub message: Entity,
}

impl LifeEvent {
    pub fn new(w: f32, h: f32, rng: &mut ThreadRng) -> LifeEvent {
        LifeEvent {
            width: w,
            height: h,
            velocity: [rng.gen_range(-100.0, 100.0), rng.gen_range(-100.0, 100.0)],
        }
    }
}

impl Member {
    fn new(rng: &mut ThreadRng) -> Member {
        Member {
            width: 26.0,
            height: 32.0,
            velocity: [rng.gen_range(-100.0, 100.0), rng.gen_range(-100.0, 100.0)],
        }
    }
}

impl Player {
    fn new() -> Player {
        Player {
            width: 26.0,
            height: 32.0,
        }
    }
}

impl Health {
    fn new(r: f32, g: f32, b: f32) -> Health {
        Health {
            health: 100.0,
            red: r,
            green: g,
            blue: b,
        }
    }
}

impl Component for LifeEvent {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Member {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

fn initalize_life_event(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();
    let mut rng = rand::thread_rng();
    transform.set_xyz(rng.gen_range(0.0, ARENA_WIDTH), rng.gen_range(0.0, ARENA_HEIGHT), 0.0);
    let arr = [17.0, 20.0, 18.0, 17.0];
    let sprite_choice = rng.gen_range(4, 8);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_choice,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(LifeEvent::new(arr[sprite_choice % 4], 7.0, &mut rng))
        .with(Rgba(1.0, 0.0, 0.0, 0.5))
        .with(transform)
        .build();
}

use amethyst::{
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

fn initalize_scoretext(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let score_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        0., -50., 1., ARENA_WIDTH, 50., 0,
    );
    
    let message_transform = UiTransform::new(
        "Msg".to_string(), Anchor::BottomMiddle,
        0., 60., 1., ARENA_WIDTH * 3., 30., 0,
    );
    let score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font.clone(),
            "Score: 0".to_string(),
            [1.,1.,1.,1.],
            50.,
        )).build();
    let message = world
        .create_entity()
        .with(message_transform)
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1., 1., 1., 1.],
            25.,
        )).build();
    world.add_resource(ScoreText { score, message });
}

fn initalize_member(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();
    let mut rng = rand::thread_rng();
    transform.set_xyz(rng.gen_range(ARENA_WIDTH / 4.0, ARENA_WIDTH * 3. / 4.0 ),
        rng.gen_range(ARENA_HEIGHT / 4.0, ARENA_HEIGHT * 3. / 4.0 ), 0.0
    );

    let sprite_render1 = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: rng.gen_range(0, 4),
    };

    let sprite_render2 = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: rng.gen_range(0, 4),
    };

    let sprite_render3 = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: rng.gen_range(0,4),
    };

    world
        .create_entity()
        .with(sprite_render1)
        .with(Member::new(&mut rng))
        .with(Rgba(0.0, 1.0, 0.0, 0.5))
        .with(Health::new(0.0, 1.0, 0.0))
        .with(transform.clone())
        .build();

    transform.set_xyz(rng.gen_range(ARENA_WIDTH / 4.0, ARENA_WIDTH * 3. / 4.0 ),
        rng.gen_range(ARENA_HEIGHT / 4.0, ARENA_HEIGHT * 3. / 4.0 ), 0.0
    );

    world
        .create_entity()
        .with(sprite_render2)
        .with(Member::new(&mut rng))
        .with(Rgba(0.0, 1.0, 0.0, 0.5))
        .with(Health::new(0.0, 1.0, 0.0))
        .with(transform.clone())
        .build();

    transform.set_xyz(rng.gen_range(ARENA_WIDTH / 4.0, ARENA_WIDTH * 3. / 4.0 ),
        rng.gen_range(ARENA_HEIGHT / 4.0, ARENA_HEIGHT * 3. / 4.0 ), 0.0
    );

    world
        .create_entity()
        .with(sprite_render3)
        .with(Member::new(&mut rng))
        .with(Rgba(0.0, 1.0, 0.0, 0.5))
        .with(Health::new(0.0, 1.0, 0.0))
        .with(transform)
        .build();
}

fn initalize_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();
    let mut rng = rand::thread_rng();
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: rng.gen_range(0, 4),
    };
    transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Rgba(1.0, 1.0, 1.0, 0.5))
        .with(Player::new())
        .with(Health::new(1.0, 1.0, 1.0))
        .with(transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/lifeless_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/lifeless_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

impl SimpleState for Lifeless {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initalize_player(world, sprite_sheet_handle.clone());
        initalize_member(world, sprite_sheet_handle.clone());
        initalize_life_event(world, sprite_sheet_handle.clone());
        initialize_camera(world);
        initalize_scoretext(world);
        world.add_resource(GameState {health: 100., score: 0});
    }

    fn handle_event(&mut self, state_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        let world = state_data.world;
        let game_state = world.read_resource::<GameState>();
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                println!("Final Score: {}", game_state.score);
                println!("Thank you for playing Lifeless by Tomas");
                return Trans::Quit
            }
        }
        Trans::None
    }
    fn fixed_update(&mut self, state_data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        let world = state_data.world;
        let game_state = world.read_resource::<GameState>();
        if game_state.health <= 0.0 {
            println!("Final Score: {}", game_state.score);
            println!("Thank you for playing Lifeless by Tomas");
            Trans::Quit
        } else { Trans::None }
    }
}
