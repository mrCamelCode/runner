use thomas::{
    GameCommand, GameCommandsArg, Input, IntCoords2d, IntVector2, Keycode, Query, QueryResultList,
    System, SystemsGenerator, TerminalCamera, TerminalTransform, Timer, EVENT_AFTER_INIT,
    EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    components::{GameManager, GameState, Moveable, Player},
    EVENT_DEFEAT, EVENT_GAME_PAUSE_STATE_CHANGE, EVENT_RESTART, EVENT_VICTORY, GAME_VICTORY_SCORE,
    PLAYER_X_OFFSET,
};

const CAMERA_SCROLL_WAIT_TIME_MILLIS: u128 = 100;

pub struct GameManagerSystemsGenerator {}
impl SystemsGenerator for GameManagerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_game_manager)),
            (
                EVENT_AFTER_INIT,
                System::new(
                    vec![Query::new().has_where::<TerminalCamera>(|cam| cam.is_main)],
                    augment_main_cam,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has_where::<Moveable>(|moveable| {
                                moveable.move_timer.elapsed_millis() >= moveable.move_interval
                            })
                            .has::<TerminalTransform>(),
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
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Input>(),
                        Query::new().has_where::<GameManager>(|gm| gm.is_waiting_to_start()),
                    ],
                    handle_press_key_to_start,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Input>(),
                        Query::new().has_where::<GameManager>(|gm| gm.is_game_over()),
                    ],
                    handle_press_key_to_restart,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Input>(),
                        Query::new()
                            .has_where::<GameManager>(|gm| gm.is_playing() || gm.is_paused()),
                    ],
                    handle_toggle_pause,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has_where::<GameManager>(|gm| {
                        gm.score >= GAME_VICTORY_SCORE && gm.is_playing()
                    })],
                    trigger_victory,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new().has_where::<Player>(|player| player.lives == 0),
                    ],
                    trigger_defeat,
                ),
            ),
            (
                EVENT_RESTART,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                    ],
                    handle_restart_game,
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
            game_state: GameState::WaitingToStart,
        })]));
}

fn augment_main_cam(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [main_cam_results, ..] = &results[..] {
        let main_cam = &main_cam_results[0];

        commands
            .borrow_mut()
            .issue(GameCommand::AddComponentsToEntity(
                *main_cam.entity(),
                vec![Box::new(Moveable {
                    move_timer: Timer::start_new(),
                    move_interval: CAMERA_SCROLL_WAIT_TIME_MILLIS,
                })],
            ));
    }
}

fn scroll_camera(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, main_cam_results, ..] = &results[..] {
        let game_manager = game_manager_results.get_only::<GameManager>();

        if !main_cam_results.is_empty() && game_manager.is_playing() {
            let mut main_cam_moveable = main_cam_results.get_only_mut::<Moveable>();
            let mut main_cam_transform = main_cam_results.get_only_mut::<TerminalTransform>();

            main_cam_transform.coords += IntVector2::right();

            main_cam_moveable.move_timer.restart();
        }
    }
}

fn update_score(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, player_results, ..] = &results[..] {
        let player = player_results.get_only::<Player>();
        let mut game_manager = game_manager_results.get_only_mut::<GameManager>();

        game_manager.score = player.distance_traveled - PLAYER_X_OFFSET as u64;
    }
}

fn handle_press_key_to_start(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [input_results, not_playing_game_manager_results, ..] = &results[..] {
        let input = input_results.get_only::<Input>();

        if !not_playing_game_manager_results.is_empty() && input.is_any_key_down() {
            let mut game_manager = not_playing_game_manager_results.get_only_mut::<GameManager>();

            game_manager.game_state = GameState::Playing;
        }
    }
}

fn handle_press_key_to_restart(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [input_results, victory_or_defeat_game_manager_results, ..] = &results[..] {
        let input = input_results.get_only::<Input>();

        if !victory_or_defeat_game_manager_results.is_empty() && input.is_any_key_down() {
            let mut game_manager =
                victory_or_defeat_game_manager_results.get_only_mut::<GameManager>();

            game_manager.game_state = GameState::Playing;

            commands
                .borrow_mut()
                .issue(GameCommand::TriggerEvent(EVENT_RESTART));
        }
    }
}

fn handle_toggle_pause(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [input_results, playing_game_manager_results, ..] = &results[..] {
        let input = input_results.get_only::<Input>();

        if !playing_game_manager_results.is_empty() {
            let mut game_manager = playing_game_manager_results.get_only_mut::<GameManager>();

            if input.is_key_down(&Keycode::Escape) {
                game_manager.game_state = if game_manager.is_paused() {
                    GameState::Playing
                } else {
                    GameState::Paused
                };

                commands
                    .borrow_mut()
                    .issue(GameCommand::TriggerEvent(EVENT_GAME_PAUSE_STATE_CHANGE));
            }
        }
    }
}

fn trigger_victory(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [winning_score_game_manager_results, ..] = &results[..] {
        if !winning_score_game_manager_results.is_empty() {
            let mut game_manager = winning_score_game_manager_results.get_only_mut::<GameManager>();

            game_manager.game_state = GameState::Victory;

            commands
                .borrow_mut()
                .issue(GameCommand::TriggerEvent(EVENT_VICTORY));
        }
    }
}

fn trigger_defeat(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [game_manager_results, dead_player_results, ..] = &results[..] {
        if !dead_player_results.is_empty() {
            let mut game_manager = game_manager_results.get_only_mut::<GameManager>();

            game_manager.game_state = GameState::Defeat;

            commands
                .borrow_mut()
                .issue(GameCommand::TriggerEvent(EVENT_DEFEAT));
        }
    }
}

fn handle_restart_game(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, main_cam_results, ..] = &results[..] {
        let mut game_manager = game_manager_results.get_only_mut::<GameManager>();
        let mut main_cam_transform = main_cam_results.get_only_mut::<TerminalTransform>();

        game_manager.game_state = GameState::Playing;
        game_manager.score = 0;

        main_cam_transform.coords = IntCoords2d::new(0, main_cam_transform.coords.y());
    }
}
