mod monster_component;
mod monster_system;

pub use monster_component::{Monster, init_monster, MonsterState, MONSTER_HEIGHT, MONSTER_WIDTH};
pub use monster_system::MonsterSystem;
