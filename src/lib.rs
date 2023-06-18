mod constants;
pub use constants::*;

mod systems;
use systems::*;

mod components;

use thomas::{
    Dimensions2d, Game, GameCommand, GameOptions, Renderer, System, TerminalRendererOptions,
};

pub fn run() {
    Game::new(GameOptions {
        max_frame_rate: 60,
        press_escape_to_quit: true,
    })
    .add_systems_from_generator(PlayerSystemsGenerator {})
    .add_systems_from_generator(WorldSystemsGenerator {})
    .add_init_system(System::new(vec![], |_, commands| {
        commands.borrow_mut().issue(GameCommand::AddEntity(vec![]));
    }))
    .start(Renderer::Terminal(TerminalRendererOptions {
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT as u64, SCREEN_WIDTH as u64),
        include_default_camera: true,
        default_foreground_color: None,
        default_background_color: Some(SKY_COLOR),
    }));
}
