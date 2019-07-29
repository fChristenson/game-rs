use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Position;

impl Component for Position {
  type Storage = NullStorage<Self>;
}
