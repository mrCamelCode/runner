use std::rc::Rc;

use thomas::{
    GameCommand, GameCommandsArg, Identity, IntVector2, Priority, Query, QueryResultList, System,
    SystemsGenerator, TerminalTransform, Timer, WorldText, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    add_distance_marker,
    components::{GameObjectManager, Player},
    make_obstacle, DISTANCE_MARKER_PIECE_NAME, EVENT_GAME_OBJECT_SCROLL, OBSTACLE_NAME,
    SCREEN_WIDTH,
};

const GENERATE_OBSTACLE_WAIT_TIME_MILLIS: u128 = 3000;
const OBSTACLE_SCROLL_WAIT_TIME_MILLIS: u128 = 50;

const DISTANCE_MARKER_SPACING: u32 = 100;

pub struct GameObjectsSystemsGenerator {}
impl SystemsGenerator for GameObjectsSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], setup_obstacle_manager)),
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
                            .has_where::<Identity>(|id| &id.name == OBSTACLE_NAME)
                            .has::<TerminalTransform>(),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == DISTANCE_MARKER_PIECE_NAME)
                            .has::<TerminalTransform>(),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == DISTANCE_MARKER_PIECE_NAME)
                            .has::<WorldText>(),
                        Query::new().has::<GameObjectManager>(),
                    ],
                    scroll_game_objects,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new()
                        .has_where::<Identity>(|id| &id.name == OBSTACLE_NAME)
                        .has_where::<TerminalTransform>(|transform| transform.coords.x() < 0)],
                    cleanup_obstacles,
                ),
            ),
            (
                EVENT_GAME_OBJECT_SCROLL,
                System::new_with_priority(
                    Priority::lower_than(&Priority::default()),
                    vec![Query::new().has::<Player>()],
                    generate_distance_markers,
                ),
            ),
        ]
    }
}

fn setup_obstacle_manager(_: Vec<QueryResultList>, commands: GameCommandsArg) {
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

fn cleanup_obstacles(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [obstacles_to_cleanup_results, ..] = &results[..] {
        for obstacle_result in obstacles_to_cleanup_results {
            commands
                .borrow_mut()
                .issue(GameCommand::DestroyEntity(*obstacle_result.entity()));
        }
    }
}

fn scroll_game_objects(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [obstacles_results, distance_marker_piece_results, distance_marker_text_results, obstacle_manager_results, ..] =
        &results[..]
    {
        let mut obstacle_manager = obstacle_manager_results.get_only_mut::<GameObjectManager>();

        if obstacle_manager.scroll_timer.elapsed_millis() >= OBSTACLE_SCROLL_WAIT_TIME_MILLIS {
            for obstacle_result in obstacles_results {
                let mut transform = obstacle_result.components().get_mut::<TerminalTransform>();

                transform.coords += IntVector2::left();
            }

            for distance_marker_result in distance_marker_piece_results {
                let mut transform = distance_marker_result
                    .components()
                    .get_mut::<TerminalTransform>();

                transform.coords += IntVector2::left();
            }

            for distance_marker_text_result in distance_marker_text_results {
                let mut world_text = distance_marker_text_result.components().get_mut::<WorldText>();

                world_text.coords += IntVector2::left();
            }

            obstacle_manager.scroll_timer.restart();

            commands
                .borrow_mut()
                .issue(GameCommand::TriggerEvent(EVENT_GAME_OBJECT_SCROLL));
        }
    }
}

fn generate_distance_markers(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_results, ..] = &results[..] {
        let player = player_results.get_only::<Player>();

        if (player.distance_traveled + SCREEN_WIDTH as u64) % DISTANCE_MARKER_SPACING as u64 == 0 {
            add_distance_marker(
                Rc::clone(&commands),
                player.distance_traveled + SCREEN_WIDTH as u64,
            );
        }
    }
}
