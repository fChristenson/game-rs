use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Component for Position {
  type Storage = DenseVecStorage<Self>;
}
