use amethyst::ecs::{DenseVecStorage, Component};
use amethyst::renderer::sprite::SpriteRender;
use amethyst::core::timing::Time;

#[derive(PartialEq)]
pub enum Direction {
  Left,
  Right
}

pub struct AnimationData {
  pub current_frame: usize,
  pub total_frames: usize,
  pub time_per_frame: f32,
  pub elapsed_time: f32,
  pub direction: Direction,
  pub ongoing: bool,
}

impl AnimationData {
  pub fn new(time_per_frame: f32, total_frames: usize) -> AnimationData {
    AnimationData {
      current_frame: 0,
      total_frames,
      time_per_frame,
      elapsed_time: 0.0,
      direction: Direction::Right,
      ongoing: false,
    }
  }

  pub fn update_animation(&mut self, frame_offset: usize, sprite_render: &mut SpriteRender, time: &Time) {
    self.elapsed_time += time.delta_seconds();
    let frame_count = (self.elapsed_time / self.time_per_frame) as usize % self.total_frames + frame_offset;
    if frame_count != self.current_frame {
      self.current_frame = frame_count;
      sprite_render.sprite_number = frame_count;
    }
    if self.current_frame % self.total_frames == 0 {
      self.ongoing = false;
      self.elapsed_time = 0.0;
    } else {
      self.ongoing = true;
    }
  }

  pub fn set_direction(&mut self, x: f64) {
    if x > 0.0 {
      self.direction = Direction::Right;
    } else {
      self.direction = Direction::Left;
    }
  }
}

pub const HERO_ATTACK_RIGHT_START_FRAME: usize = 2;
pub const HERO_ATTACK_LEFT_START_FRAME: usize = 8;

pub struct HeroAttackAnimation {
  pub animation_data: AnimationData
}

impl HeroAttackAnimation {
  pub fn new(time_per_frame: f32, total_frames: usize) -> HeroAttackAnimation {
    HeroAttackAnimation{animation_data: AnimationData::new(time_per_frame, total_frames)}
  }
}

impl Component for HeroAttackAnimation {
  type Storage = DenseVecStorage<Self>;
}

pub const HERO_WALK_RIGHT_START_FRAME: usize = 14;
pub const HERO_WALK_LEFT_START_FRAME: usize = 20;

pub struct HeroWalkAnimation {
  pub animation_data: AnimationData
}

impl HeroWalkAnimation {
  pub fn new(time_per_frame: f32, total_frames: usize) -> HeroWalkAnimation {
    HeroWalkAnimation{animation_data: AnimationData::new(time_per_frame, total_frames)}
  }

  pub fn set_idle_animation(&mut self, sprite_render: &mut SpriteRender) {
    self.animation_data.elapsed_time = 0.0;
    if self.animation_data.direction == Direction::Right {
      self.animation_data.current_frame = 0;
    } else {
      self.animation_data.current_frame = 1;
    }
    sprite_render.sprite_number = self.animation_data.current_frame;
  }
}

impl Component for HeroWalkAnimation {
  type Storage = DenseVecStorage<Self>;
}
