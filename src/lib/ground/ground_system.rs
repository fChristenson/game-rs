use super::ground_component::GROUND_TOP;
use super::super::position::Position;
use amethyst::core::Transform;
use amethyst::ecs::{WriteStorage, ReadStorage, System, Join};

pub struct GroundSystem;

impl<'s> System<'s> for GroundSystem {
  type SystemData = (
    ReadStorage<'s, Position>,
    WriteStorage<'s, Transform>
  );

  fn run(&mut self, (position, mut transform): Self::SystemData) {
    for (_, transform) in (&position, &mut transform).join() {
        let y = transform.translation().y.as_f32() - 5.0;

        if y < GROUND_TOP {
          transform.set_translation_y(GROUND_TOP);  
        }
    }
  }
}
