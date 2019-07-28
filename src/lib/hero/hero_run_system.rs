use super::super::camera::ARENA_WIDTH;
use super::hero_animation::{Direction, HeroAnimation};
use super::hero_component::{Hero, HERO_RUN_SPEED, HERO_WIDTH};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct HeroRunSystem;

impl<'s> System<'s> for HeroRunSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Hero>,
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, HeroAnimation>,
    Read<'s, Time>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(
    &mut self,
    (mut transform, hero, mut sprite_render, mut animation, time, input): Self::SystemData,
  ) {
    for (_, sprite_render, animation, transform) in
      (&hero, &mut sprite_render, &mut animation, &mut transform).join()
    {
      let x = input.axis_value("horizontal");

      if let Some(x) = x {
        let x_pos = transform.translation().x.as_f32();

        if x_pos < HERO_WIDTH {
          transform.set_translation_x(x_pos + 1.0);
        } else if x_pos > ARENA_WIDTH - HERO_WIDTH {
          transform.set_translation_x(x_pos - 1.0);
        } else {
          let position = time.delta_seconds() * HERO_RUN_SPEED;
          transform.prepend_translation_x(x * position as f64);
        }

        if x > 0.0 {
          animation.direction = Direction::Right;
          update_animation(8, 6, sprite_render, animation, &time);
        } else if x < 0.0 {
          animation.direction = Direction::Left;
          update_animation(1, 6, sprite_render, animation, &time);
        }
        if animation.direction == Direction::Right && x == 0.0 {
          set_idle_animation(7, sprite_render, animation);
        } else if animation.direction == Direction::Left && x == 0.0 {
          set_idle_animation(0, sprite_render, animation);
        }
      }
    }
  }
}

fn set_idle_animation(
  sprite_number: usize,
  sprite_render: &mut SpriteRender,
  animation: &mut HeroAnimation,
) {
  sprite_render.sprite_number = sprite_number;
  animation.elapsed_time = 0.0;
}

fn update_animation(
  offset: usize,
  limit: usize,
  sprite_render: &mut SpriteRender,
  animation: &mut HeroAnimation,
  time: &Time
) {
  animation.elapsed_time += time.delta_seconds();
  let frame_count =
    (animation.elapsed_time / animation.time_per_frame) as usize % limit + offset;
  if frame_count != animation.current_frame {
    animation.current_frame = frame_count;
    sprite_render.sprite_number = frame_count;
  }
}
