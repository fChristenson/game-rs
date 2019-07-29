use super::super::ground::GROUND_TOP;
use super::super::camera::ARENA_WIDTH;
use super::super::position_component::Position;
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::{
  Component, DenseVecStorage, World
};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

pub const MONSTER_WIDTH: f32 = 64.0;
pub const MONSTER_HEIGHT: f32 = 64.0;

#[derive(PartialEq)]
pub enum MonsterState {
  Alive,
  Dead
}

pub struct Monster {
  pub state: MonsterState,
}

impl Monster {
  pub fn new() -> Monster {
    Monster{state: MonsterState::Alive}
  }
}

impl Component for Monster {
  type Storage = DenseVecStorage<Self>;
}

pub fn init_monster(world: &mut World) {
  let sprite_handle = load_monster_sprite_sheet(world);

  world.register::<Monster>();

  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH, GROUND_TOP * 1.2, 0.0);

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_handle,
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(Monster::new())
    .with(Position)
    .with(transform)
    .with(sprite_render)
    .build();
}

fn load_monster_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let loader = world.read_resource::<Loader>();
  let texture_storage = world.read_resource::<AssetStorage<Texture>>();
  let texture_handle = loader.load(
    "textures/monster_spritesheet.png",
    ImageFormat::default(),
    (),
    &texture_storage,
  );

  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "textures/monster_spritesheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
