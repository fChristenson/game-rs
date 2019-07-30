use super::super::ground::GROUND_TOP;
use super::super::camera::ARENA_HEIGHT;
use super::hero_animation_component::{HeroAttackAnimation, HeroWalkAnimation};
use super::hero_jump_component::Jump;
use super::super::position_component::Position;
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::{
  Component, World, NullStorage
};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};

#[derive(Default)]
pub struct Hero;

pub const HERO_RUN_SPEED: f32 = 200.0;
pub const HERO_JUMP_DURATION: f32 = 1.0;
pub const HERO_JUMP_HEIGHT: f32 = (ARENA_HEIGHT - GROUND_TOP) / 4.0;
pub const HERO_WIDTH: f32 = 32.0;
pub const HERO_HEIGHT: f32 = 32.0;

impl Component for Hero {
  type Storage = NullStorage<Self>;
}

pub fn init_hero(world: &mut World) {
  let sprite_handle = load_hero_sprite_sheet(world);

  world.register::<Hero>();

  let mut transform = Transform::default();
  transform.set_translation_xyz(HERO_WIDTH, GROUND_TOP * 1.2, 0.0);

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_handle,
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(Hero)
    .with(Position::default())
    .with(Jump::new())
    .with(HeroWalkAnimation::new(0.1, 6))
    .with(HeroAttackAnimation::new(0.1, 6))
    .with(transform)
    .with(sprite_render)
    .build();
}

fn load_hero_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let loader = world.read_resource::<Loader>();
  let texture_storage = world.read_resource::<AssetStorage<Texture>>();
  let texture_handle = loader.load(
    "textures/hero_spritesheet.png",
    ImageFormat::default(),
    (),
    &texture_storage,
  );

  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "textures/hero_spritesheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
