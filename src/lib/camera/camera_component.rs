use amethyst::core::Transform;
use amethyst::ecs::World;
use amethyst::prelude::*;
use amethyst::renderer::Camera;

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

pub fn init_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
    .with(transform)
    .build();
}
