use super::position::Position;
use super::hero::HERO_JUMP_HEIGHT;
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{WriteStorage, Read, ReadStorage, System, Join};

pub const GRAVITY: f32 = HERO_JUMP_HEIGHT * -0.5;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
  type SystemData = (
    ReadStorage<'s, Position>,
    Read<'s, Time>,
    WriteStorage<'s, Transform>
  );

  fn run(&mut self, (position, time, mut transform): Self::SystemData) {
    for (_, transform) in (&position, &mut transform).join() {
      let position = time.delta_seconds() * GRAVITY;
      transform.prepend_translation_y(position);
    }
  }
}
