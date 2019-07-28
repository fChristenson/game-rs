use super::super::gravity::GRAVITY;
use super::hero_component::{HERO_JUMP_HEIGHT, HERO_JUMP_DURATION, Hero};
use super::hero_jump_component::Jump;
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

pub struct HeroJumpSystem;

const VELOCITY: f32 = 2.0 * HERO_JUMP_HEIGHT / (HERO_JUMP_DURATION / 2.0);

impl<'s> System<'s> for HeroJumpSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Jump>,
    ReadStorage<'s, Hero>,
    Read<'s, Time>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut transform, mut jump, hero, time, input): Self::SystemData) {
    for (_, jump, transform) in (&hero, &mut jump, &mut transform).join() {
      let y = input.axis_value("vertical");

      if let Some(y) = y {
        if jump.ongoing && jump.elapsed_time >= HERO_JUMP_DURATION {
          jump.ongoing = false;
          jump.elapsed_time = 0.0;
        }
        if y > 0.0 && !jump.ongoing {
          jump.ongoing = true;
        }
        if jump.ongoing {
          if jump.elapsed_time >= HERO_JUMP_DURATION / 2.0 {
            let applied_gravity= -2.0 * HERO_JUMP_HEIGHT / (HERO_JUMP_DURATION / 2.0).powf(2.0);
            let position = time.delta_seconds() * (VELOCITY + applied_gravity - GRAVITY);
            transform.prepend_translation_y(position);
          } else {
            let position = time.delta_seconds() * VELOCITY;
            transform.prepend_translation_y(position);
          }
          
          jump.elapsed_time += time.delta_seconds();
        }
      }
    }
  }
}
