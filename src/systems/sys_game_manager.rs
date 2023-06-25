use thomas::{
    GameCommand, GameCommandsArg, IntVector2, Query, QueryResultList, System, SystemsGenerator,
    TerminalCamera, TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{components::{GameManager, Player}, PLAYER_X_OFFSET};

const CAMERA_SCROLL_WAIT_TIME_MILLIS: u128 = 100;

pub struct GameManagerSystemsGenerator {}
impl SystemsGenerator for GameManagerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_game_manager)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                        Query::new().has::<GameManager>(),
                    ],
                    scroll_camera,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new().has::<Player>(),
                    ],
                    update_score,
                ),
            ),
        ]
    }
}

fn make_game_manager(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    commands
        .borrow_mut()
        .issue(GameCommand::AddEntity(vec![Box::new(GameManager {
            camera_scroll_timer: Timer::start_new(),
            score: 0,
        })]));
}

fn scroll_camera(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [main_cam_results, game_manager_results, ..] = &results[..] {
        let mut game_manager = game_manager_results.get_only_mut::<GameManager>();

        if game_manager.camera_scroll_timer.elapsed_millis() >= CAMERA_SCROLL_WAIT_TIME_MILLIS {
            let mut main_cam_transform = main_cam_results.get_only_mut::<TerminalTransform>();

            main_cam_transform.coords += IntVector2::right();

            game_manager.camera_scroll_timer.restart();
        }
    }
}

fn update_score(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, player_results, ..] = &results[..] {
        let player = player_results.get_only::<Player>();
        let mut game_manager = game_manager_results.get_only_mut::<GameManager>();

        game_manager.score = player.distance_traveled - PLAYER_X_OFFSET;
    }
}
