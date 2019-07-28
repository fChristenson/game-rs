use amethyst::ecs::{Component, DenseVecStorage};

pub struct Position;

impl Component for Position {
  type Storage = DenseVecStorage<Self>;
}
