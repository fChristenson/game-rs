use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::{Component, NullStorage, World};
use amethyst::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

#[derive(Default)]
pub struct Ground;

const GROUND_HEIGHT: f32 = 64.0;
const GROUND_WIDTH: f32 = 128.0;
pub const GROUND_TOP: f32 = GROUND_HEIGHT * 1.5;

impl Component for Ground {
  type Storage = NullStorage<Self>;
}

pub fn init_ground(world: &mut World) {
  let sprite_handle = load_ground_sprite_sheet(world);

  world.register::<Ground>();

  let tiles = {
    let width = world.read_resource::<ScreenDimensions>().width();
    (width / GROUND_WIDTH) as u32
  };

  for i in 0..tiles {
    let mut transform = Transform::default();
    transform.set_translation_xyz(i as f32 * GROUND_WIDTH, GROUND_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
      sprite_sheet: sprite_handle.clone(),
      sprite_number: 0,
    };

    world
      .create_entity()
      .with(transform)
      .with(sprite_render.clone())
      .build();
  }
}

fn load_ground_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let loader = world.read_resource::<Loader>();
  let texture_storage = world.read_resource::<AssetStorage<Texture>>();
  let texture_handle = loader.load(
    "textures/ground.png",
    ImageFormat::default(),
    (),
    &texture_storage,
  );

  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "textures/ground_spritesheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
