use std::{collections::HashMap, rc::Rc};

use rand::{thread_rng, Rng};
use thomas::{
    Dimensions2d, GameCommand, GameCommandsArg, Identity, IntCoords2d, Layer, Matrix,
    QueryResultList, Rgb, System, SystemsGenerator, TerminalCollider, TerminalRenderer,
    TerminalTransform, Timer, EVENT_INIT,
};

use crate::{
    add_building,
    components::{FixedToCamera, SkylineBuilding, WorldTime},
    GROUND_COLLISION_LAYER, PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT, SCREEN_WIDTH,
    SKY_COLOR_TRANSITION_TIMER_NAME, STAR_COLOR_TRANSITION_TIMER_NAME, STAR_DISPLAY, STAR_LAYER,
    STAR_NAME, SUN_COLOR_TRANSITION_TIMER_NAME, SUN_ID, SUN_LAYER, SUN_PIECE_NAME,
    WINDOW_COLOR_TRANSITION_TIMER_NAME,
};

const AVAILABLE_BACKGROUND_HEIGHT: u64 = SCREEN_HEIGHT as u64 - PLAYER_Y_OFFSET as u64;

const GROUND_COLOR: Rgb = Rgb(94, 153, 84);

const NUM_STARS: u8 = 26;
const NUM_START_BUILDINGS: u8 = 15;

pub struct WorldSetupSystemsGenerator {}
impl SystemsGenerator for WorldSetupSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_world_time)),
            (EVENT_INIT, System::new(vec![], make_ground)),
            (EVENT_INIT, System::new(vec![], make_skyline)),
            (EVENT_INIT, System::new(vec![], make_stars)),
            (EVENT_INIT, System::new(vec![], make_sun)),
        ]
    }
}

fn make_sun(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    let coords = IntCoords2d::zero();

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalTransform { coords }),
        Box::new(FixedToCamera {
            base_position: coords,
            offset: IntCoords2d::zero(),
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
        Box::new(TerminalTransform { coords: coords }),
        Box::new(FixedToCamera {
            base_position: coords,
            offset: IntCoords2d::zero(),
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
        Box::new(FixedToCamera {
            base_position: coords,
            offset: IntCoords2d::zero(),
        }),
        Box::new(Identity {
            name: String::from(SUN_PIECE_NAME),
            id: String::from(""),
        }),
    ]));
}

fn make_stars(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    for _ in 0..NUM_STARS {
        let coords = IntCoords2d::new(
            thread_rng().gen_range(0..SCREEN_WIDTH as i64),
            thread_rng().gen_range(0..AVAILABLE_BACKGROUND_HEIGHT as i64),
        );

        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: STAR_DISPLAY,
                layer: STAR_LAYER,
                foreground_color: None,
                background_color: None,
            }),
            Box::new(TerminalTransform { coords }),
            Box::new(FixedToCamera {
                base_position: coords,
                offset: IntCoords2d::zero(),
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
    let coords = IntCoords2d::new(PLAYER_X_OFFSET, SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET);

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(TerminalTransform { coords: coords }),
        Box::new(FixedToCamera {
            base_position: coords,
            offset: IntCoords2d::zero(),
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
        let coords = ground_start_coords + *cell.location();

        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: ' ',
                layer: Layer::base(),
                background_color: Some(GROUND_COLOR),
                foreground_color: None,
            }),
            Box::new(TerminalTransform { coords }),
            Box::new(FixedToCamera {
                base_position: coords,
                offset: IntCoords2d::zero(),
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
