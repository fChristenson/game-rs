use super::hero_component::{Hero, HERO_HEIGHT, HERO_WIDTH};
use super::hero_animation_component::HeroAttackAnimation;
use super::super::monster::{Monster, MonsterState, MONSTER_HEIGHT, MONSTER_WIDTH};
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

pub struct HeroAttackSystem;

impl<'s> System<'s> for HeroAttackSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    WriteStorage<'s, Monster>,
    ReadStorage<'s, Hero>,
    ReadStorage<'s, HeroAttackAnimation>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (transform, mut monster, hero, animation, input): Self::SystemData) {
    let is_attacking = input.action_is_down("attack");

    for (_, animation, hero_transform) in (&hero, &animation, &transform).join() {
      let hero_x = hero_transform.translation().x.as_f32() - (HERO_WIDTH * 0.5);
      let hero_y = hero_transform.translation().y.as_f32() - (HERO_HEIGHT * 0.5);

      for (monster, monster_transform) in (&mut monster, &transform).join() {
        let monster_x = monster_transform.translation().x.as_f32() - (MONSTER_WIDTH * 0.5);
        let monster_y = monster_transform.translation().y.as_f32() - (MONSTER_HEIGHT * 0.5);

        let monster_top = monster_y + MONSTER_HEIGHT;
        let monster_right = monster_x + MONSTER_WIDTH;
        let monster_bottom = monster_y;
        let monster_left = monster_x;
        let is_hit = in_box(hero_x, hero_y, monster_left, monster_bottom, monster_right, monster_top);

        if is_hit && animation.animation_data.ongoing {
          monster.state = MonsterState::Dead;
        }
      }
    }
  }
}

fn in_box(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
  x >= left && x <= right && y >= bottom && y <= top
}
