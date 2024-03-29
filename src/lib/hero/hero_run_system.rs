use super::super::camera::ARENA_WIDTH;
use super::hero_component::{Hero, HERO_RUN_SPEED, HERO_WIDTH};
use super::super::position_component::Position;
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

pub struct HeroRunSystem;

impl<'s> System<'s> for HeroRunSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Hero>,
    WriteStorage<'s, Position>,
    Read<'s, Time>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut transform, hero, mut position, time, input): Self::SystemData) {
    for (_, position, transform) in (&hero, &mut position, &mut transform).join() {
      let x = input.axis_value("horizontal");

      if let Some(x) = x {
        let x_pos = transform.translation().x.as_f32();

        if x_pos < HERO_WIDTH {
          transform.set_translation_x(x_pos + 1.0);
        } else if x_pos > ARENA_WIDTH - HERO_WIDTH {
          transform.set_translation_x(x_pos - 1.0);
        } else {
          let pos = time.delta_seconds() * HERO_RUN_SPEED;
          transform.prepend_translation_x(x * pos as f64);
        }

        position.x = transform.translation().x.as_f32();
      }
    }
  }
}
