use std::rc::Rc;

use rand::{thread_rng, Rng};
use thomas::{
    Dimensions2d, GameCommand, GameCommandsArg, IntCoords2d, Layer, Matrix, QueryResultList, Rgb,
    System, SystemsGenerator, TerminalCollider, TerminalRenderer, TerminalTransform, EVENT_INIT,
};

use crate::{
    add_building, GROUND_COLLISION_LAYER, PLAYER_X_OFFSET, PLAYER_Y_OFFSET, SCREEN_HEIGHT,
    SCREEN_WIDTH, components::SkylineBuilding,
};

const GROUND_COLOR: Rgb = Rgb(94, 153, 84);

pub struct WorldSystemsGenerator {}
impl SystemsGenerator for WorldSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], make_ground)),
            (EVENT_INIT, System::new(vec![], make_skyline)),
        ]
    }
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
    const AVAILABLE_HEIGHT: u64 = SCREEN_HEIGHT as u64 - PLAYER_Y_OFFSET as u64;

    const BUILDING_MIN_WIDTH: u64 = 3;
    const BUILDING_MAX_WIDTH: u64 = 6;

    const BUILDING_MIN_HEIGHT: u64 = AVAILABLE_HEIGHT - (AVAILABLE_HEIGHT as f64 * 0.8) as u64;
    const BUILDING_MAX_HEIGHT: u64 = AVAILABLE_HEIGHT - 1;

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
