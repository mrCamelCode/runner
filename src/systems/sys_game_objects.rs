use std::{ops::RangeInclusive, rc::Rc};

use rand::{thread_rng, Rng};
use thomas::{
    GameCommand, GameCommandsArg, Identity, IntVector2, Query, QueryResult, QueryResultList,
    System, SystemsGenerator, TerminalCamera, TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    add_distance_marker,
    components::{
        CleanupOnScreenExit, FollowCamera, GameManager, GameObjectManager, Moveable, Player,
    },
    make_extra_life, make_obstacle, ObstacleType, BUILDING_PIECE_NAME, DISTANCE_MARKER_PIECE_NAME,
    EVENT_RESTART, OBSTACLE_NAME,
};

const GENERATE_OBSTACLE_WAIT_TIME_MILLIS_RANGE: RangeInclusive<u128> = 250..=3000;

const DISTANCE_MARKER_SPACING: u64 = 500;

const GENERATE_PLAYER_LIFE_WAIT_TIME_MILLIS: u128 = 5000;
const GENERATE_PLAYER_LIFE_CHANCE: u8 = 10;

pub struct GameObjectsSystemsGenerator {}
impl SystemsGenerator for GameObjectsSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_obstacle_manager)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameObjectManager>(),
                        Query::new().has::<GameManager>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                    ],
                    generate_obstacles,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<GameObjectManager>(|gom| {
                            gom.extra_life_generation_timer.elapsed_millis()
                                >= GENERATE_PLAYER_LIFE_WAIT_TIME_MILLIS
                        }),
                        Query::new().has::<GameManager>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                    ],
                    generate_player_lives,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == OBSTACLE_NAME)
                            .has_where::<Moveable>(|moveable| {
                                moveable.move_timer.elapsed_millis() >= moveable.move_interval
                            })
                            .has::<TerminalTransform>(),
                    ],
                    move_moveable_obstacles,
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
            (
                EVENT_RESTART,
                System::new(
                    vec![
                        Query::new().has_where::<Identity>(|id| &id.name == OBSTACLE_NAME),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == DISTANCE_MARKER_PIECE_NAME),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == BUILDING_PIECE_NAME)
                            .has::<FollowCamera>(),
                    ],
                    handle_restart_game,
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
            extra_life_generation_timer: Timer::start_new(),
            scroll_timer: Timer::start_new(),
            next_obstacle_wait_time: thread_rng()
                .gen_range(GENERATE_OBSTACLE_WAIT_TIME_MILLIS_RANGE),
        })]));
}

fn generate_obstacles(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [obstacle_manager_results, game_manager_results, main_cam_results, ..] = &results[..] {
        let mut obstacle_manager = obstacle_manager_results.get_only_mut::<GameObjectManager>();
        let game_manager = game_manager_results.get_only::<GameManager>();
        let main_cam_transform = main_cam_results.get_only::<TerminalTransform>();

        if obstacle_manager.obstacle_generation_timer.elapsed_millis()
            >= obstacle_manager.next_obstacle_wait_time
        {
            if game_manager.is_playing() {
                commands
                    .borrow_mut()
                    .issue(GameCommand::AddEntity(make_obstacle(
                        &main_cam_transform,
                        if thread_rng().gen_bool(0.5) {
                            ObstacleType::Ground
                        } else {
                            ObstacleType::Air
                        },
                    )));
            }

            obstacle_manager.obstacle_generation_timer.restart();
            obstacle_manager.next_obstacle_wait_time =
                thread_rng().gen_range(GENERATE_OBSTACLE_WAIT_TIME_MILLIS_RANGE);
        }
    }
}

fn generate_player_lives(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [generation_ready_game_object_manager_results, game_manager_results, main_cam_results, ..] =
        &results[..]
    {
        if !generation_ready_game_object_manager_results.is_empty() {
            let game_manager = game_manager_results.get_only::<GameManager>();
            let mut game_object_manager =
                generation_ready_game_object_manager_results.get_only_mut::<GameObjectManager>();
            let main_cam_transform = main_cam_results.get_only::<TerminalTransform>();

            let roll = thread_rng().gen_range(0..100 as u8);

            if roll < GENERATE_PLAYER_LIFE_CHANCE && game_manager.is_playing() {
                commands
                    .borrow_mut()
                    .issue(GameCommand::AddEntity(make_extra_life(&main_cam_transform)));
            }

            game_object_manager.extra_life_generation_timer.restart();
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

fn move_moveable_obstacles(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, moveable_obstacles_results, ..] = &results[..] {
        let game_manager = game_manager_results.get_only::<GameManager>();

        if game_manager.is_playing() {
            for moveable_obstacle_result in moveable_obstacles_results {
                let mut transform = moveable_obstacle_result
                    .components()
                    .get_mut::<TerminalTransform>();
                let mut moveable = moveable_obstacle_result.components().get_mut::<Moveable>();

                transform.coords += IntVector2::left();

                moveable.move_timer.restart();
            }
        }
    }
}

fn handle_restart_game(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [obstacle_results, distance_marker_results, building_piece_results, ..] = &results[..] {
        let destroy = |result: &QueryResult| {
            commands
                .borrow_mut()
                .issue(GameCommand::DestroyEntity(*result.entity()));
        };

        obstacle_results.iter().for_each(destroy);
        distance_marker_results.iter().for_each(destroy);

        for building_piece_result in building_piece_results {
            let mut follow_cam = building_piece_result.components().get_mut::<FollowCamera>();

            follow_cam.offset = IntVector2::zero();
        }
    }
}
