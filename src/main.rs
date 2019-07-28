mod lib;
use amethyst::assets::Processor;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{types::DefaultBackend, RenderingSystem, SpriteSheet};
use amethyst::utils::application_root_dir;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::window::WindowBundle;
use amethyst::{start_logger, Application, Error, GameDataBuilder, LoggerConfig};

use lib::{GameGraphCreator, GameState, HeroJumpSystem, HeroRunSystem, MonsterSystem, GroundSystem, GravitySystem};

fn main() -> Result<(), Error> {
    start_logger(LoggerConfig::default());
    let root_dir = application_root_dir()?;
    let display_config_path = root_dir.join("resources").join("display_config.ron");
    let assets_dir = root_dir.join("assets");
    let binding_path = root_dir.join("resources").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(HeroRunSystem, "hero_run_system", &["input_system"])
        .with(HeroJumpSystem, "hero_jump_system", &["input_system"])
        .with(MonsterSystem, "monster_system", &[])
        .with(GravitySystem, "gravity_system", &["hero_jump_system"])
        .with(GroundSystem, "ground_system", &["gravity_system", "hero_jump_system"])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            GameGraphCreator::default(),
        ));
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
