use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use rand::{thread_rng, Rng};
use thomas::{
    Dimensions2d, GameCommand, GameCommandsArg, Identity, IntCoords2d, IntVector2, Layer, Lerp,
    Matrix, Priority, Query, QueryResultList, Rgb, System, SystemsGenerator, TerminalCollider,
    TerminalRenderer, TerminalRendererState, TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    add_building, add_distance_marker,
    components::{
        Player, SkylineBuilding, TimeOfDay, WorldTime, NOON_TIME, SUNRISE_TIME, SUNSET_TIME,
    },
    get_color, BUILDING_PIECE_NAME, EVENT_GAME_OBJECT_SCROLL, EVENT_TIME_OF_DAY_CHANGE,
    GROUND_COLLISION_LAYER, PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT, SCREEN_WIDTH,
    SKY_COLORS, STAR_COLORS, STAR_DISPLAY, STAR_LAYER, STAR_NAME, SUN_COLORS, SUN_ID, SUN_LAYER,
    SUN_PIECE_NAME, WINDOW_DISPLAY,
};

const GROUND_COLOR: Rgb = Rgb(94, 153, 84);
const ADVANCE_TIME_WAIT_TIME_MILLIS: u128 = 1000;

const COLOR_TRANSITION_TIME_MILLIS: u128 = 800;
const SKY_COLOR_TRANSITION_TIMER_NAME: &str = "sky-color";
const STAR_COLOR_TRANSITION_TIMER_NAME: &str = "star-color";
const SUN_COLOR_TRANSITION_TIMER_NAME: &str = "sun-color";
const WINDOW_COLOR_TRANSITION_TIMER_NAME: &str = "window-color";

const WINDOW_TURN_OFF_TIME_MILLIS: u128 = 300;

const AVAILABLE_BACKGROUND_HEIGHT: u64 = SCREEN_HEIGHT as u64 - PLAYER_Y_OFFSET as u64;

const NUM_STARS: u8 = 26;
const NUM_START_BUILDINGS: u8 = 15;


pub struct WorldSystemsGenerator {}
impl SystemsGenerator for WorldSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_world_time)),
            (EVENT_INIT, System::new(vec![], make_ground)),
            (EVENT_INIT, System::new(vec![], make_skyline)),
            (EVENT_INIT, System::new(vec![], make_stars)),
            (EVENT_INIT, System::new(vec![], make_sun)),
            (
                EVENT_UPDATE,
                System::new(vec![Query::new().has::<WorldTime>()], update_world_time),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<WorldTime>(),
                        Query::new().has::<TerminalRendererState>(),
                        Query::new()
                            .has_where::<Identity>(|identity| identity.name == STAR_NAME)
                            .has::<TerminalRenderer>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.name == SUN_PIECE_NAME)
                            .has::<TerminalRenderer>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.name == BUILDING_PIECE_NAME)
                            .has::<TerminalRenderer>(),
                    ],
                    update_world_colors_from_time,
                ),
            ),
            (
                EVENT_TIME_OF_DAY_CHANGE,
                System::new(
                    vec![
                        Query::new().has::<WorldTime>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.name == BUILDING_PIECE_NAME)
                            .has_where::<TerminalRenderer>(|renderer| renderer.display == ' '),
                    ],
                    turn_on_windows,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<WorldTime>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.name == BUILDING_PIECE_NAME)
                            .has_where::<TerminalRenderer>(|renderer| {
                                renderer.display == WINDOW_DISPLAY
                            }),
                    ],
                    turn_off_windows,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<WorldTime>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.id == SUN_ID)
                            .has::<TerminalTransform>(),
                        Query::new()
                            .has_where::<Identity>(|identity| &identity.name == SUN_PIECE_NAME)
                            .has::<TerminalTransform>(),
                    ],
                    update_sun_position,
                ),
            ),
        ]
    }
}

fn make_sun(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalTransform {
            coords: IntCoords2d::zero(),
        }),
        Box::new(Identity {
            name: String::from(""),
            id: String::from(SUN_ID),
        }),
    ]));
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalRenderer {
            display: ' ',
            background_color: Some(Rgb::yellow()),
            foreground_color: None,
            layer: SUN_LAYER,
        }),
        Box::new(TerminalTransform {
            coords: IntCoords2d::zero(),
        }),
        Box::new(Identity {
            name: String::from(SUN_PIECE_NAME),
            id: String::from(""),
        }),
    ]));
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalRenderer {
            display: ' ',
            background_color: Some(Rgb::yellow()),
            foreground_color: None,
            layer: SUN_LAYER,
        }),
        Box::new(TerminalTransform {
            coords: IntCoords2d::zero(),
        }),
        Box::new(Identity {
            name: String::from(SUN_PIECE_NAME),
            id: String::from(""),
        }),
    ]));
}

fn make_stars(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    for _ in 0..NUM_STARS {
        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: STAR_DISPLAY,
                layer: STAR_LAYER,
                foreground_color: None,
                background_color: None,
            }),
            Box::new(TerminalTransform {
                coords: IntCoords2d::new(
                    thread_rng().gen_range(0..SCREEN_WIDTH as i64),
                    thread_rng().gen_range(0..AVAILABLE_BACKGROUND_HEIGHT as i64),
                ),
            }),
            Box::new(Identity {
                id: String::from(""),
                name: String::from(STAR_NAME),
            }),
        ]))
    }
}

fn make_world_time(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    commands
        .borrow_mut()
        .issue(GameCommand::AddEntity(vec![Box::new(WorldTime {
            current_time: 9,
            advance_time_timer: Timer::start_new(),
            color_transition_timers: HashMap::from([
                (SKY_COLOR_TRANSITION_TIMER_NAME, Timer::new()),
                (STAR_COLOR_TRANSITION_TIMER_NAME, Timer::new()),
                (SUN_COLOR_TRANSITION_TIMER_NAME, Timer::new()),
                (WINDOW_COLOR_TRANSITION_TIMER_NAME, Timer::new()),
            ]),
        })]))
}

fn make_ground(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    make_real_ground(Rc::clone(&commands));
    make_decorative_ground(Rc::clone(&commands));
}

fn make_real_ground(commands: GameCommandsArg) {
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalTransform {
            coords: IntCoords2d::new(PLAYER_X_OFFSET, SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET),
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: GROUND_COLLISION_LAYER,
        }),
    ]));
}

fn make_decorative_ground(commands: GameCommandsArg) {
    let ground_fill_matrix = Matrix::new(
        Dimensions2d::new(PLAYER_Y_OFFSET as u64, SCREEN_WIDTH as u64),
        || (),
    );
    let ground_start_coords = IntCoords2d::new(0, SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET + 1);

    for cell in &ground_fill_matrix {
        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: ' ',
                layer: Layer::base(),
                background_color: Some(GROUND_COLOR),
                foreground_color: None,
            }),
            Box::new(TerminalTransform {
                coords: ground_start_coords + *cell.location(),
            }),
            Box::new(SkylineBuilding {}),
        ]));
    }
}

fn make_skyline(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    const BUILDING_MIN_WIDTH: u64 = 3;
    const BUILDING_MAX_WIDTH: u64 = 6;

    const BUILDING_MIN_HEIGHT: u64 =
        AVAILABLE_BACKGROUND_HEIGHT - (AVAILABLE_BACKGROUND_HEIGHT as f64 * 0.8) as u64;
    const BUILDING_MAX_HEIGHT: u64 = AVAILABLE_BACKGROUND_HEIGHT - 1;

    let mut x_coord = thread_rng().gen_range(1..5);

    for _ in 0..NUM_START_BUILDINGS {
        let size = Dimensions2d::new(
            thread_rng().gen_range(BUILDING_MIN_HEIGHT..=BUILDING_MAX_HEIGHT),
            thread_rng().gen_range(BUILDING_MIN_WIDTH..=BUILDING_MAX_WIDTH),
        );

        add_building(Rc::clone(&commands), x_coord, size.clone());

        x_coord += size.width() as i64;
    }
}

fn update_world_time(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [world_time_results, ..] = &results[..] {
        let mut world_time = world_time_results.get_only_mut::<WorldTime>();

        if world_time.advance_time_timer.elapsed_millis() >= ADVANCE_TIME_WAIT_TIME_MILLIS {
            let prev_time_of_day = world_time.time_of_day();

            if world_time.current_time == 23 {
                world_time.current_time = 0;
            } else {
                world_time.current_time += 1;
            }

            world_time.advance_time_timer.restart();

            if world_time.time_of_day() != prev_time_of_day {
                commands
                    .borrow_mut()
                    .issue(GameCommand::TriggerEvent(EVENT_TIME_OF_DAY_CHANGE));
            }
        }
    }
}

fn turn_on_windows(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, windows_results, ..] = &results[..] {
        let world_time = world_time_results.get_only::<WorldTime>();

        let num_windows_to_turn_on = match world_time.time_of_day() {
            TimeOfDay::Dusk => windows_results.len() / 5,
            TimeOfDay::Night => windows_results.len() / 2,
            _ => 0,
        };

        let mut picked_window_indices: HashSet<usize> = HashSet::new();

        for _ in 0..num_windows_to_turn_on {
            if let Some(picked_window_index) =
                pick_window_index(windows_results.len(), &picked_window_indices)
            {
                picked_window_indices.insert(picked_window_index);

                windows_results[picked_window_index]
                    .components()
                    .get_mut::<TerminalRenderer>()
                    .display = WINDOW_DISPLAY;
            }
        }
    }
}

fn turn_off_windows(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, windows_results, ..] = &results[..] {
        let mut world_time = world_time_results.get_only_mut::<WorldTime>();
        let transition_timer = world_time
            .color_transition_timers
            .get_mut(WINDOW_COLOR_TRANSITION_TIMER_NAME)
            .unwrap();

        if !transition_timer.is_running() {
            transition_timer.restart();
        }

        if transition_timer.elapsed_millis() >= WINDOW_TURN_OFF_TIME_MILLIS {
            transition_timer.restart();

            let mut num_windows_to_turn_off = windows_results.len() / 3;
            if num_windows_to_turn_off == 0 && windows_results.len() > 0 {
                num_windows_to_turn_off = windows_results.len();
            }

            let mut picked_window_indices: HashSet<usize> = HashSet::new();

            if world_time.is_light() {
                for _ in 0..num_windows_to_turn_off {
                    if let Some(picked_window_index) =
                        pick_window_index(windows_results.len(), &picked_window_indices)
                    {
                        picked_window_indices.insert(picked_window_index);

                        windows_results[picked_window_index]
                            .components()
                            .get_mut::<TerminalRenderer>()
                            .display = ' ';
                    }
                }
            }
        }
    }
}

fn pick_window_index(
    window_collection_len: usize,
    unavailable_indices: &HashSet<usize>,
) -> Option<usize> {
    if window_collection_len == unavailable_indices.len() {
        return None;
    }

    let mut index = thread_rng().gen_range(0..window_collection_len);

    while unavailable_indices.contains(&index) {
        if index + 1 >= window_collection_len {
            index = 0;
        } else {
            index += 1;
        }
    }

    return Some(index);
}

fn update_world_colors_from_time(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, terminal_renderer_state_results, stars_results, sun_pieces_results, building_pieces_results, ..] =
        &results[..]
    {
        let mut world_time = world_time_results.get_only_mut::<WorldTime>();
        let mut terminal_renderer_state =
            terminal_renderer_state_results.get_only_mut::<TerminalRendererState>();

        let time_of_day = world_time.time_of_day();

        let current_sky_color_option = terminal_renderer_state.options.default_background_color;
        let target_sky_color_option = get_color(&SKY_COLORS, &time_of_day);

        terminal_renderer_state.options.default_background_color = blend_color_to_target(
            &current_sky_color_option,
            target_sky_color_option,
            world_time
                .color_transition_timers
                .get_mut(SKY_COLOR_TRANSITION_TIMER_NAME)
                .unwrap(),
        );

        for star_result in stars_results {
            let mut renderer = star_result.components().get_mut::<TerminalRenderer>();

            let current_color_option = renderer.foreground_color;
            let target_star_color_option = get_color(&STAR_COLORS, &time_of_day);
            let target_color_option = if let Some(target_color) = target_star_color_option {
                Some(*target_color)
            } else {
                *target_sky_color_option
            };

            renderer.display = if target_star_color_option.is_none() {
                ' '
            } else {
                STAR_DISPLAY
            };

            renderer.foreground_color = blend_color_to_target(
                &current_color_option,
                &target_color_option,
                world_time
                    .color_transition_timers
                    .get_mut(STAR_COLOR_TRANSITION_TIMER_NAME)
                    .unwrap(),
            );
        }

        for sun_piece_result in sun_pieces_results {
            let mut renderer = sun_piece_result.components().get_mut::<TerminalRenderer>();

            let current_color_option = renderer.background_color;
            let target_color_option = get_color(&SUN_COLORS, &world_time.time_of_day());

            renderer.background_color = blend_color_to_target(
                &current_color_option,
                target_color_option,
                world_time
                    .color_transition_timers
                    .get_mut(SUN_COLOR_TRANSITION_TIMER_NAME)
                    .unwrap(),
            );
        }
    }
}

fn blend_color_to_target(
    current_color_option: &Option<Rgb>,
    target_color_option: &Option<Rgb>,
    transition_timer: &mut Timer,
) -> Option<Rgb> {
    if current_color_option.is_none() && target_color_option.is_some() {
        return *target_color_option;
    } else if let Some(current_color) = current_color_option {
        if let Some(target_color) = target_color_option {
            let mut interpolated_color: Option<Rgb> = *current_color_option;

            if target_color != current_color {
                if !transition_timer.is_running() {
                    transition_timer.restart();
                }

                interpolated_color = Some(Rgb::lerp(
                    current_color,
                    target_color,
                    transition_timer.elapsed_millis() as f32 / COLOR_TRANSITION_TIME_MILLIS as f32,
                ));
            }

            if transition_timer.elapsed_millis() >= COLOR_TRANSITION_TIME_MILLIS {
                transition_timer.stop();

                interpolated_color = Some(*target_color);
            }

            return interpolated_color;
        }
    }

    *current_color_option
}

fn update_sun_position(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, sun_results, sun_pieces_results, ..] = &results[..] {
        let world_time = world_time_results.get_only::<WorldTime>();
        let mut sun_transform = sun_results.get_only_mut::<TerminalTransform>();

        sun_transform.coords = IntCoords2d::new(
            get_sun_x(world_time.current_time),
            get_sun_y(world_time.current_time),
        );

        for i in 0..sun_pieces_results.len() {
            let sun_piece_result = &sun_pieces_results[i];

            let mut sun_piece_transform =
                sun_piece_result.components().get_mut::<TerminalTransform>();

            sun_piece_transform.coords = sun_transform.coords + IntVector2::new(i as i64, 0);
        }
    }
}

fn get_sun_x(current_time: u8) -> i64 {
    if current_time <= SUNRISE_TIME || current_time > SUNSET_TIME {
        -100
    } else {
        f64::round(
            SCREEN_WIDTH as f64
                * ((current_time as f64 - SUNRISE_TIME as f64)
                    / (SUNSET_TIME as f64 - SUNRISE_TIME as f64)),
        ) as i64
    }
}

fn get_sun_y(current_time: u8) -> i64 {
    if current_time <= SUNRISE_TIME || current_time > SUNSET_TIME {
        -100
    } else {
        f64::round(
            ((SCREEN_HEIGHT as f64 - 2.0) / (NOON_TIME as f64 - SUNRISE_TIME as f64).powf(2.0))
                * (current_time as f64 - NOON_TIME as f64).powf(2.0)
                + 1.0,
        ) as i64
    }
}

