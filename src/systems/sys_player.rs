use thomas::{
    GameCommand, GameCommandsArg, Input, IntCoords2d, Keycode, Layer, Priority, Query,
    QueryResultList, Rgb, System, SystemsGenerator, TerminalCamera, TerminalCollider,
    TerminalCollision, TerminalRenderer, TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    components::{FixedToCamera, Player},
    GROUND_COLLISION_LAYER, OBSTACLE_COLLISION_LAYER, PLAYER_COLLISION_LAYER, PLAYER_DISPLAY,
    PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT,
};

const JUMP_WAIT_TIME_MILLIS: u128 = 100;
const JUMP_FORCE: i8 = -50;
const JUMP_BUTTONS: [Keycode; 1] = [Keycode::Space];
const GRAVITY: i8 = 15;

pub struct PlayerSystemsGenerator {}
impl SystemsGenerator for PlayerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_player)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<Player>(), Query::new().has::<Input>()],
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
                    vec![Query::new().has::<Player>().has::<FixedToCamera>()],
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
                    vec![Query::new().has::<Player>()],
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
            vertical_velocity: 0,
            is_on_ground: false,
            distance_traveled: 0,
        }),
        Box::new(TerminalTransform { coords }),
        Box::new(FixedToCamera {
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
    if let [player_results, input_results, ..] = &results[..] {
        let input = input_results.get_only::<Input>();
        let mut player = player_results.get_only_mut::<Player>();

        if JUMP_BUTTONS.iter().any(|button| input.is_key_down(&button)) && player.is_on_ground
        // && player.jump_timer.elapsed_millis() >= JUMP_WAIT_TIME_MILLIS
        {
            player.vertical_velocity = JUMP_FORCE as i64;

            // player.jump_timer.restart();
        }
    }
}

fn apply_velocity(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();
        let mut fixed_to_camera = player_results.get_only_mut::<FixedToCamera>();

        if player.vertical_velocity != 0
            && player.velocity_timer.elapsed_millis()
                >= 1000 / i64::abs(player.vertical_velocity) as u128
        {
            fixed_to_camera.offset += if player.vertical_velocity > 0 {
                IntCoords2d::up()
            } else {
                IntCoords2d::down()
            };

            player.velocity_timer.restart();
        }
    }
}

fn update_velocity(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();

        if player.is_on_ground {
            player.vertical_velocity = 0;
        } else if GRAVITY != 0
            && player.gravity_timer.elapsed_millis() >= 1000 / i8::abs(GRAVITY) as u128
        {
            player.vertical_velocity += GRAVITY as i64;

            player.gravity_timer.restart();
        }
    }
}

fn detect_ground(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, player_ground_collision_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();

        player.is_on_ground = player_ground_collision_results.len() > 0;
    }
}

fn handle_obstacle_collision(results: Vec<QueryResultList>, commands: GameCommandsArg) {}

fn update_distance_traveled(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_results, main_cam_results, ..] = &results[..] {
        let mut player = player_results.get_only_mut::<Player>();
        let main_cam_transform = main_cam_results.get_only::<TerminalTransform>();

        player.distance_traveled = main_cam_transform.coords.x() + PLAYER_X_OFFSET;
    }
}
