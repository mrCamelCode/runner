use thomas::{
    GameCommand, GameCommandsArg, Input, IntCoords2d, Keycode, Layer, Priority, Query,
    QueryResultList, Rgb, System, SystemsGenerator, TerminalCamera, TerminalCollider,
    TerminalCollision, TerminalRenderer, TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    components::{FollowCamera, GameManager, Player},
    EVENT_RESTART, GROUND_COLLISION_LAYER, OBSTACLE_COLLISION_LAYER, PLAYER_COLLISION_LAYER,
    PLAYER_DISPLAY, PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT,
};

const MAX_AIR_JUMPS: u8 = 1;
const JUMP_FORCE: i8 = -50;
const JUMP_BUTTONS: [Keycode; 1] = [Keycode::Space];
const GRAVITY: i8 = 15;
const MAX_LIVES: u8 = 3;

pub struct PlayerSystemsGenerator {}
impl SystemsGenerator for PlayerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_player)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<GameManager>(|gm| gm.is_playing()),
                        Query::new().has::<Player>(),
                        Query::new().has::<Input>(),
                    ],
                    handle_input,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Player>(),
                        Query::new().has_where::<TerminalCollision>(|coll| {
                            coll.is_collision_between(
                                PLAYER_COLLISION_LAYER,
                                OBSTACLE_COLLISION_LAYER,
                            )
                        }),
                    ],
                    handle_obstacle_collision,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Player>(),
                        Query::new().has_where::<TerminalCollision>(|coll| {
                            coll.is_collision_between(
                                PLAYER_COLLISION_LAYER,
                                PLAYER_COLLISION_LAYER,
                            )
                        }),
                    ],
                    handle_extra_life_collision,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<GameManager>(|gm| gm.is_playing()),
                        Query::new().has::<Player>().has::<FollowCamera>(),
                    ],
                    apply_velocity,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new_with_priority(
                    Priority::higher_than(&Priority::default()),
                    vec![
                        Query::new().has::<Player>().has::<TerminalTransform>(),
                        Query::new().has_where::<TerminalCollision>(|coll| {
                            coll.is_collision_between(
                                PLAYER_COLLISION_LAYER,
                                GROUND_COLLISION_LAYER,
                            )
                        }),
                    ],
                    detect_ground,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new_with_priority(
                    Priority::higher_than(&Priority::default()),
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new().has::<Player>(),
                    ],
                    update_velocity,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Player>(),
                        Query::new()
                            .has_where::<TerminalCamera>(|cam| cam.is_main)
                            .has::<TerminalTransform>(),
                    ],
                    update_distance_traveled,
                ),
            ),
            (
                EVENT_RESTART,
                System::new(vec![Query::new().has::<Player>()], handle_restart_game),
            ),
        ]
    }
}

fn make_player(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    let coords = IntCoords2d::new(PLAYER_X_OFFSET, SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET);

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(Player {
            jump_timer: Timer::start_new(),
            gravity_timer: Timer::start_new(),
            velocity_timer: Timer::start_new(),
            num_times_jumped_since_landing: 0,
            vertical_velocity: 0,
            is_on_ground: false,
            distance_traveled: 0,
            lives: MAX_LIVES,
        }),
        Box::new(TerminalTransform { coords }),
        Box::new(FollowCamera {
            base_position: coords,
            offset: IntCoords2d::zero(),
        }),
        Box::new(TerminalRenderer {
            display: PLAYER_DISPLAY,
            layer: Layer::base(),
            background_color: None,
            foreground_color: Some(Rgb::white()),
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: PLAYER_COLLISION_LAYER,
        }),
    ]))
}

fn handle_input(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [running_game_manager, player_results, input_results, ..] = &results[..] {
        if !running_game_manager.is_empty() {
            let input = input_results.get_only::<Input>();
            let mut player = player_results.get_only_mut::<Player>();

            if JUMP_BUTTONS.iter().any(|button| input.is_key_down(&button))
                && (player.is_on_ground || player.num_times_jumped_since_landing < MAX_AIR_JUMPS)
            {
                if !player.is_on_ground {
                    player.num_times_jumped_since_landing += 1;
                }

                player.vertical_velocity = JUMP_FORCE as i64;
            }
        }
    }
}

fn apply_velocity(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [running_game_manager_results, player_results, ..] = &results[..] {
        if !running_game_manager_results.is_empty() {
            let mut player = player_results.get_only_mut::<Player>();
            let mut follow_cam = player_results.get_only_mut::<FollowCamera>();

            if player.vertical_velocity != 0
                && player.velocity_timer.elapsed_millis()
                    >= 1000 / i64::abs(player.vertical_velocity) as u128
            {
                follow_cam.offset += if player.vertical_velocity > 0 {
                    IntCoords2d::up()
                } else {
                    IntCoords2d::down()
                };

                player.velocity_timer.restart();
            }
        }
    }
}

fn update_velocity(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, player_results, ..] = &results[..] {
        let game_manager = game_manager_results.get_only::<GameManager>();
        let mut player = player_results.get_only_mut::<Player>();

        if game_manager.is_playing() {
            if player.is_on_ground {
                player.vertical_velocity = 0;
                player.num_times_jumped_since_landing = 0;
            } else if GRAVITY != 0
                && player.gravity_timer.elapsed_millis() >= 1000 / i8::abs(GRAVITY) as u128
            {
                player.vertical_velocity += GRAVITY as i64;

                player.gravity_timer.restart();
            }
        }
    }
}

fn detect_ground(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, player_ground_collision_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();

        player.is_on_ground = !player_ground_collision_results.is_empty();
    }
}

fn handle_obstacle_collision(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_results, collision_results, ..] = &results[..] {
        if !collision_results.is_empty() {
            let mut player = player_results.get_only_mut::<Player>();
            let obstacle_entity = collision_results[0]
                .components()
                .get::<TerminalCollision>()
                .get_entity_on_layer(OBSTACLE_COLLISION_LAYER)
                .unwrap();

            player.lives = player.lives.saturating_sub(1);

            commands
                .borrow_mut()
                .issue(GameCommand::DestroyEntity(obstacle_entity));
        }
    }
}

fn handle_extra_life_collision(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_results, collision_results, ..] = &results[..] {
        if !collision_results.is_empty() {
            let mut player = player_results.get_only_mut::<Player>();
            let extra_life_entity = collision_results[0]
                .components()
                .get::<TerminalCollision>()
                .bodies
                .iter()
                .find(|(entity, _)| entity != player_results[0].entity())
                .unwrap()
                .0;

            if player.lives < MAX_LIVES {
                player.lives += 1;
            }

            commands
                .borrow_mut()
                .issue(GameCommand::DestroyEntity(extra_life_entity));
        }
    }
}

fn update_distance_traveled(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, main_cam_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();
        let main_cam_transform = main_cam_results.get_only::<TerminalTransform>();

        player.distance_traveled = main_cam_transform.coords.x() as u64 + PLAYER_X_OFFSET as u64;
    }
}

fn handle_restart_game(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();

        player.lives = MAX_LIVES;
    }
}
