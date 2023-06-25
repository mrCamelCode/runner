use std::rc::Rc;

use thomas::{
    GameCommand, GameCommandsArg, Identity, IntVector2, Priority, Query, QueryResultList, System,
    SystemsGenerator, TerminalCamera, TerminalTransform, Timer, WorldText, EVENT_INIT,
    EVENT_UPDATE,
};

use crate::{
    add_distance_marker,
    components::{CleanupOnScreenExit, GameObjectManager, Player},
    make_obstacle, DISTANCE_MARKER_PIECE_NAME, OBSTACLE_NAME, SCREEN_WIDTH,
};

const GENERATE_OBSTACLE_WAIT_TIME_MILLIS: u128 = 3000;

const DISTANCE_MARKER_SPACING: u64 = 500;

pub struct GameObjectsSystemsGenerator {}
impl SystemsGenerator for GameObjectsSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_obstacle_manager)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<GameObjectManager>()],
                    generate_obstacles,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has::<CleanupOnScreenExit>()
                            .has::<TerminalTransform>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                    ],
                    cleanup_entities,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == DISTANCE_MARKER_PIECE_NAME),
                        Query::new().has::<Player>(),
                    ],
                    generate_distance_markers,
                ),
            ),
        ]
    }
}

fn make_obstacle_manager(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    commands
        .borrow_mut()
        .issue(GameCommand::AddEntity(vec![Box::new(GameObjectManager {
            obstacle_generation_timer: Timer::start_new(),
            scroll_timer: Timer::start_new(),
        })]));
}

fn generate_obstacles(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [obstacle_manager_results, ..] = &results[..] {
        let mut obstacle_manager = obstacle_manager_results.get_only_mut::<GameObjectManager>();

        if obstacle_manager.obstacle_generation_timer.elapsed_millis()
            >= GENERATE_OBSTACLE_WAIT_TIME_MILLIS
        {
            commands
                .borrow_mut()
                .issue(GameCommand::AddEntity(make_obstacle()));

            obstacle_manager.obstacle_generation_timer.restart();
        }
    }
}

fn cleanup_entities(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [transform_results, main_cam_results, ..] = &results[..] {
        let main_cam_transform = main_cam_results.get_only::<TerminalTransform>();

        for transform_result in transform_results {
            let transform = transform_result.components().get::<TerminalTransform>();

            if transform.coords.x() < main_cam_transform.coords.x() - 10 {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*transform_result.entity()));
            }
        }
    }
}

fn generate_distance_markers(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [existing_distance_marker_pieces_results, player_results, ..] = &results[..] {
        if existing_distance_marker_pieces_results.len() == 0 {
            let player = player_results.get_only::<Player>();

            let next_distance_marker_distance = DISTANCE_MARKER_SPACING
                * ((f64::floor(player.distance_traveled as f64 / DISTANCE_MARKER_SPACING as f64)
                    + 1.0) as u64);

            add_distance_marker(Rc::clone(&commands), next_distance_marker_distance as u64);
        }
    }
}
