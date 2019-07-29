use super::hero_animation_component::{Direction, HeroAnimation};
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct HeroAnimationSystem;

impl<'s> System<'s> for HeroAnimationSystem {
  type SystemData = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, HeroAnimation>,
    Read<'s, Time>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut sprite_render, mut animation, time, input): Self::SystemData) {
    for (sprite_render, animation) in (&mut sprite_render, &mut animation).join() {
      let x = input.axis_value("horizontal");
      let is_attacking = input.action_is_down("attack");

      if let Some(x) = x {
        if is_attacking.unwrap_or_default() && x > 0.0 {
          animation.direction = Direction::Right;
          return update_animation(2, 6, sprite_render, animation, &time);
        } else if is_attacking.unwrap_or_default() && x < 0.0 {
          animation.direction = Direction::Left;
          return update_animation(8, 6, sprite_render, animation, &time);
        } else if x > 0.0 {
          animation.direction = Direction::Right;
          return update_animation(14, 6, sprite_render, animation, &time);
        } else if x < 0.0 {
          animation.direction = Direction::Left;
          return update_animation(20, 6, sprite_render, animation, &time);
        }

        if is_attacking.unwrap_or_default() && animation.direction == Direction::Right && x == 0.0 {
          animation.direction = Direction::Right;
          return update_animation(2, 6, sprite_render, animation, &time);
        } else if is_attacking.unwrap_or_default() && animation.direction == Direction::Left && x == 0.0 {
          animation.direction = Direction::Left;
          return update_animation(8, 6, sprite_render, animation, &time);
        } else if animation.direction == Direction::Right && x == 0.0 {
          return set_idle_animation(0, sprite_render, animation);
        } else if animation.direction == Direction::Left && x == 0.0 {
          return set_idle_animation(1, sprite_render, animation);
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
  time: &Time,
) {
  animation.elapsed_time += time.delta_seconds();
  let frame_count = (animation.elapsed_time / animation.time_per_frame) as usize % limit + offset;
  if frame_count != animation.current_frame {
    animation.current_frame = frame_count;
    sprite_render.sprite_number = frame_count;
  }
}
