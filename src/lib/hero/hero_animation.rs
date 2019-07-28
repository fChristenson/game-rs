use amethyst::ecs::{DenseVecStorage, Component};

#[derive(PartialEq)]
pub enum Direction {
  Left,
  Right
}

pub struct HeroAnimation {
  pub current_frame: usize,
  pub time_per_frame: f32,
  pub elapsed_time: f32,
  pub direction: Direction,
}

impl HeroAnimation {
  pub fn new(time_per_frame: f32) -> HeroAnimation {
    HeroAnimation {
      current_frame: 0,
      time_per_frame,
      elapsed_time: 0.0,
      direction: Direction::Right
    }
  }
}

impl Component for HeroAnimation {
  type Storage = DenseVecStorage<Self>;
}
