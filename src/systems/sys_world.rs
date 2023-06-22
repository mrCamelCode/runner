use std::{collections::HashMap, rc::Rc, thread::current};

use rand::{thread_rng, Rng};
use thomas::{
    Dimensions2d, GameCommand, GameCommandsArg, Identity, IntCoords2d, Layer, Lerp, Matrix, Query,
    QueryResultList, Rgb, System, SystemsGenerator, TerminalCollider, TerminalRenderer,
    TerminalRendererState, TerminalTransform, Timer, EVENT_BEFORE_UPDATE, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    add_building,
    components::{SkylineBuilding, TimeOfDay, WorldTime, NOON_TIME, SUNRISE_TIME, SUNSET_TIME},
    get_color, GROUND_COLLISION_LAYER, PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT,
    SCREEN_WIDTH, SKY_COLORS, STAR_COLORS, STAR_DISPLAY, STAR_LAYER, STAR_NAME, SUN_ID,
    SUN_PIECE_NAME,
};

const GROUND_COLOR: Rgb = Rgb(94, 153, 84);
const ADVANCE_TIME_WAIT_TIME_MILLIS: u128 = 1000;

const COLOR_TRANSITION_TIME_MILLIS: u128 = 1500;
const SKY_COLOR_TRANSITION_TIMER_NAME: &str = "sky-color";
const STAR_COLOR_TRANSITION_TIMER_NAME: &str = "star-color";

const AVAILABLE_BACKGROUND_HEIGHT: u64 = SCREEN_HEIGHT as u64 - PLAYER_Y_OFFSET as u64;

const NUM_STARS: u8 = 26;

pub struct WorldSystemsGenerator {}
impl SystemsGenerator for WorldSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_world_time)),
            (EVENT_INIT, System::new(vec![], make_ground)),
            (EVENT_INIT, System::new(vec![], make_skyline)),
            (EVENT_INIT, System::new(vec![], make_stars)),
            (EVENT_INIT, System::new(vec![], make_sun)),
            // (
            //     EVENT_BEFORE_UPDATE,
            //     System::new(
            //         vec![
            //             Query::new().has::<TerminalRendererState>(),
            //             Query::new().has::<WorldTime>(),
            //         ],
            //         |results, _| {
            //             if let [state_results, world_time_results, ..] = &results[..] {
            //                 let mut state = state_results.get_only_mut::<TerminalRendererState>();

            //                 if state.options.default_background_color.is_none() {
            //                     let world_time = world_time_results.get_only::<WorldTime>();

            //                     state.options.default_background_color =
            //                         *get_color(&SKY_COLORS, &world_time.time_of_day());
            //                 }
            //             }
            //         },
            //     ),
            // ),
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
                    ],
                    update_world_colors_from_time,
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
            layer: Layer::base(),
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
            layer: Layer::base(),
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

    for _ in 0..10 {
        let size = Dimensions2d::new(
            thread_rng().gen_range(BUILDING_MIN_HEIGHT..=BUILDING_MAX_HEIGHT),
            thread_rng().gen_range(BUILDING_MIN_WIDTH..=BUILDING_MAX_WIDTH),
        );

        add_building(Rc::clone(&commands), x_coord, size.clone());

        x_coord += size.width() as i64;
    }
}

fn update_world_time(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, ..] = &results[..] {
        let mut world_time = world_time_results.get_only_mut::<WorldTime>();

        if world_time.advance_time_timer.elapsed_millis() >= ADVANCE_TIME_WAIT_TIME_MILLIS {
            if world_time.current_time == 23 {
                world_time.current_time = 0;
            } else {
                world_time.current_time += 1;
            }

            world_time.advance_time_timer.restart();
        }
    }
}

fn update_world_colors_from_time(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [world_time_results, terminal_renderer_state_results, stars_results, ..] = &results[..] {
        let mut world_time = world_time_results.get_only_mut::<WorldTime>();
        let mut terminal_renderer_state =
            terminal_renderer_state_results.get_only_mut::<TerminalRendererState>();

        let time_of_day = world_time.time_of_day();

        let current_sky_color_option = terminal_renderer_state.options.default_background_color;
        let target_sky_color_option = get_color(&SKY_COLORS, &time_of_day);

        // if current_sky_color_option.is_none() && target_sky_color_option.is_some() {
        //     terminal_renderer_state.options.default_background_color = *target_sky_color_option;
        // } else {
            terminal_renderer_state.options.default_background_color = blend_color_to_target(
                &current_sky_color_option,
                target_sky_color_option,
                world_time
                    .color_transition_timers
                    .get_mut(SKY_COLOR_TRANSITION_TIMER_NAME)
                    .unwrap(),
            );
        // }

        for star_result in stars_results {
            let mut renderer = star_result.components().get_mut::<TerminalRenderer>();

            let current_color_option = renderer.foreground_color;
            let target_color_option = if let Some(target_color) = get_color(&STAR_COLORS, &time_of_day) {
                Some(*target_color)
            } else {
                *target_sky_color_option
            };

            renderer.display = if target_color_option.is_none() { ' ' } else { STAR_DISPLAY };

            renderer.foreground_color = blend_color_to_target(
                &current_color_option,
                &target_color_option,
                world_time
                    .color_transition_timers
                    .get_mut(STAR_COLOR_TRANSITION_TIMER_NAME)
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

            sun_piece_transform.coords = sun_transform.coords + IntCoords2d::new(i as i64, 0);
        }
    }
}

fn get_sun_x(current_time: u8) -> i64 {
    if current_time < SUNRISE_TIME || current_time > SUNSET_TIME {
        -100
    } else {
        (SCREEN_WIDTH
            * ((current_time as u16 - SUNRISE_TIME as u16)
                / (SUNSET_TIME as u16 - SUNRISE_TIME as u16))) as i64
    }
}

fn get_sun_y(current_time: u8) -> i64 {
    if current_time < SUNRISE_TIME || current_time > SUNSET_TIME {
        -100
    } else {
        ((SCREEN_HEIGHT as i64 - 2) / (NOON_TIME as i64 - SUNRISE_TIME as i64).pow(2))
            * (current_time as i64 - NOON_TIME as i64).pow(2)
            + 1
    }
}
