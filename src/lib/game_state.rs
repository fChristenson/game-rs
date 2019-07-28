use amethyst::{StateData, GameData, SimpleState};
use super::ground::init_ground;
use super::camera::init_camera;
use super::hero::init_hero;
use super::monster::init_monster;

pub struct GameState;

impl SimpleState for GameState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    init_ground(world);
    init_hero(world);
    init_monster(world);
    init_camera(world)
  }
}
