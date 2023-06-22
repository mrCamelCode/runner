mod constants;
pub use constants::*;

mod systems;
use systems::*;

mod components;

mod prefabs;
pub use prefabs::*;

use thomas::{
    Dimensions2d, Game, GameCommand, GameOptions, Renderer, System, TerminalRendererOptions, Text, IntCoords2d, Rgb, Identity, Query, TerminalRendererState,
};

pub fn run() {
    Game::new(GameOptions {
        max_frame_rate: 60,
        press_escape_to_quit: true,
    })
    .add_systems_from_generator(PlayerSystemsGenerator {})
    .add_systems_from_generator(WorldSystemsGenerator {})
    .add_init_system(System::new(vec![], |_, commands| {
        // commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        //     Box::new(Text {
        //         anchor: thomas::UiAnchor::TopLeft,
        //         background_color: None,
        //         foreground_color: Some(Rgb::magenta()),
        //         justification: thomas::Alignment::Left,
        //         offset: IntCoords2d::zero(),
        //         value: String::from(""),
        //     }),
        //     Box::new(Identity {
        //         id: String::from("background-color-tag"),
        //         name: String::from(""),
        //     }),
        // ]));
    }))
    .add_update_system(System::new(vec![Query::new().has::<Text>().has_where::<Identity>(|id| id.id == "background-color-tag"), Query::new().has::<TerminalRendererState>()], |results, _| {
        // if let [tag_results, renderer_state_results, ..] = &results[..] {
        //     let mut text = tag_results.get_only_mut::<Text>();
        //     let state = renderer_state_results.get_only::<TerminalRendererState>();

        //     text.value = format!("{:?}", state.options.default_background_color);
        // }
    }))
    .start(Renderer::Terminal(TerminalRendererOptions {
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT as u64, SCREEN_WIDTH as u64),
        include_default_camera: true,
        default_foreground_color: None,
        default_background_color: None,
    }));
}
