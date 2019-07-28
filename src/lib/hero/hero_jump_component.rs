use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Jump {
  pub elapsed_time: f32,
  pub ongoing: bool,
}

impl Jump {
  pub fn new() -> Jump {
    Jump{elapsed_time: 0.0, ongoing: false}
  }
}

impl Component for Jump {
  type Storage = DenseVecStorage<Self>;
}
