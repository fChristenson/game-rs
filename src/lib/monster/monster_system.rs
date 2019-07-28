use super::super::camera::ARENA_WIDTH;
use super::super::position::Position;
use super::monster_component::{Monster, MONSTER_WIDTH};
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

pub struct MonsterSystem;

impl<'s> System<'s> for MonsterSystem {
  type SystemData = (
    ReadStorage<'s, Monster>,
    ReadStorage<'s, Position>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (monster, position, mut transform): Self::SystemData) {
    for (_, _, transform) in (&monster, &position, &mut transform).join() {
      let x_pos = transform.translation().x.as_f32();
      if x_pos <= 0.0 - MONSTER_WIDTH {
        transform.set_translation_x(ARENA_WIDTH);
      } else {
        transform.prepend_translation_x(-2.0);
      }
    }
  }
}
