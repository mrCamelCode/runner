mod constants;
pub use constants::*;

mod systems;
use systems::*;

mod components;

mod prefabs;
pub use prefabs::*;

use thomas::{
    Dimensions2d, Game, GameCommand, GameOptions, Identity, IntCoords2d, Query, Renderer, Rgb,
    System, TerminalRendererOptions, TerminalRendererState, Text,
};

pub fn run() {
    Game::new(GameOptions {
        max_frame_rate: 60,
        press_escape_to_quit: true,
    })
    .add_systems_from_generator(PlayerSystemsGenerator {})
    .add_systems_from_generator(WorldSetupSystemsGenerator {})
    .add_systems_from_generator(WorldUpdateSystemsGenerator {})
    .add_systems_from_generator(GameManagerSystemsGenerator {})
    .add_systems_from_generator(FixedToCameraSystemsGenerator {})
    // .add_systems_from_generator(GameObjectsSystemsGenerator {})
    .add_systems_from_generator(HudSystemsGenerator {})
    .start(Renderer::Terminal(TerminalRendererOptions {
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT as u64, SCREEN_WIDTH as u64),
        include_default_camera: true,
        default_foreground_color: None,
        default_background_color: None,
    }));
}
