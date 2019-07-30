use super::hero_animation_component::{Direction, HeroAttackAnimation, HeroWalkAnimation, HERO_WALK_RIGHT_START_FRAME, HERO_WALK_LEFT_START_FRAME, HERO_ATTACK_LEFT_START_FRAME, HERO_ATTACK_RIGHT_START_FRAME};
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct HeroAnimationSystem;

impl<'s> System<'s> for HeroAnimationSystem {
  type SystemData = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, HeroAttackAnimation>,
    WriteStorage<'s, HeroWalkAnimation>,
    Read<'s, Time>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut sprite_render, mut attack_animation, mut walk_animation, time, input): Self::SystemData) {
    for (sprite_render, attack_animation, walk_animation) in (&mut sprite_render, &mut attack_animation, &mut walk_animation).join() {
      let x = input.axis_value("horizontal");
      let is_attacking = input.action_is_down("attack");

      if let Some(x) = x {
        if x != 0.0 {
          attack_animation.animation_data.set_direction(x);
          walk_animation.animation_data.set_direction(x);
        }

        if is_attacking.unwrap_or_default() || attack_animation.animation_data.ongoing {
          // attack
          if attack_animation.animation_data.direction == Direction::Right {
            return attack_animation.animation_data.update_animation(HERO_ATTACK_RIGHT_START_FRAME, sprite_render, &time);
          } else {
            return attack_animation.animation_data.update_animation(HERO_ATTACK_LEFT_START_FRAME, sprite_render, &time);
          }

        } else if x != 0.0 {
          // walk
          if walk_animation.animation_data.direction == Direction::Right {
            return walk_animation.animation_data.update_animation(HERO_WALK_RIGHT_START_FRAME, sprite_render, &time);
          } else {
            return walk_animation.animation_data.update_animation(HERO_WALK_LEFT_START_FRAME, sprite_render, &time);
          }
        } else {
          // idle
          return walk_animation.set_idle_animation(sprite_render);
        }
      }
    }
  }
}
