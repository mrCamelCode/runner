use rand::{thread_rng, Rng};
use thomas::{
    Alignment, Component, Dimensions2d, GameCommand, GameCommandsArg, Identity, IntCoords2d,
    IntVector2, Layer, Matrix, Rgb, TerminalCollider, TerminalRenderer, TerminalTransform, Vector2,
    WorldText,
};

use crate::{
    components::FixedToCamera, ALTERNATE_BUILDING_COLOR, BUILDING_COLOR, BUILDING_PIECE_NAME,
    DISTANCE_MARKER_PIECE_NAME, OBSTACLE_BACKGROUND_COLOR, OBSTACLE_COLLISION_LAYER, OBSTACLE_NAME,
    PLAYER_Y_OFFSET, SCREEN_HEIGHT, SCREEN_WIDTH, SKYLINE_LAYER, WINDOW_COLOR,
};

pub fn add_building(commands: GameCommandsArg, x_coord: i64, size: Dimensions2d) {
    let building_shape_matrix = Matrix::new(size, || ());

    let start_coords = IntCoords2d::new(
        x_coord,
        SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET - size.height() as i64 + 1,
    );

    let background_color = if thread_rng().gen_bool(0.5) {
        BUILDING_COLOR
    } else {
        ALTERNATE_BUILDING_COLOR
    };

    for cell in &building_shape_matrix {
        let coords = start_coords + *cell.location();

        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: ' ',
                layer: SKYLINE_LAYER,
                foreground_color: Some(WINDOW_COLOR),
                background_color: Some(background_color),
            }),
            Box::new(TerminalTransform { coords: coords }),
            Box::new(FixedToCamera {
                base_position: coords,
                offset: IntCoords2d::zero(),
            }),
            Box::new(Identity {
                id: String::from(""),
                name: String::from(BUILDING_PIECE_NAME),
            }),
        ]));
    }
}

pub fn make_obstacle() -> Vec<Box<dyn Component>> {
    vec![
        Box::new(TerminalRenderer {
            background_color: Some(OBSTACLE_BACKGROUND_COLOR),
            foreground_color: None,
            display: ' ',
            layer: Layer::base(),
        }),
        Box::new(TerminalTransform {
            coords: IntCoords2d::new(
                SCREEN_WIDTH as i64 + 1,
                SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET,
            ),
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: OBSTACLE_COLLISION_LAYER,
        }),
        Box::new(Identity {
            id: String::from(""),
            name: String::from(OBSTACLE_NAME),
        }),
    ]
}

pub fn add_distance_marker(commands: GameCommandsArg, distance: u64) {
    let board_matrix = Matrix::new(Dimensions2d::new(1, 5), || ());

    let base_x_pos = SCREEN_WIDTH;

    let board_y_pos: i64 = SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET as i64 - 1;

    for cell in &board_matrix {
        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: ' ',
                layer: Layer::below(&Layer::base()),
                background_color: Some(Rgb(47, 168, 80)),
                foreground_color: None,
            }),
            Box::new(TerminalTransform {
                coords: IntCoords2d::new(base_x_pos as i64 + cell.location().x(), board_y_pos),
            }),
            Box::new(Identity {
                id: String::from(""),
                name: String::from(DISTANCE_MARKER_PIECE_NAME),
            }),
        ]));
    }

    for i in 0..2 {
        let support_y_pos: i64 = board_y_pos + 1;

        let support_x_pos = if i == 0 {
            base_x_pos as i64
        } else {
            (base_x_pos as u64 + board_matrix.dimensions().width() - 1) as i64
        };

        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: '‖',
                background_color: None,
                foreground_color: Some(Rgb(200, 200, 200)),
                layer: Layer::below(&Layer::base()),
            }),
            Box::new(TerminalTransform {
                coords: IntCoords2d::new(support_x_pos, support_y_pos),
            }),
            Box::new(Identity {
                id: String::from(""),
                name: String::from(DISTANCE_MARKER_PIECE_NAME),
            }),
        ]))
    }

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(WorldText {
            coords: IntCoords2d::new(base_x_pos as i64, board_y_pos),
            offset: IntCoords2d::zero(),
            background_color: None,
            foreground_color: Some(Rgb(21, 77, 36)),
            justification: Alignment::Left,
            value: distance.to_string(),
        }),
        Box::new(Identity {
            id: String::from(""),
            name: String::from(DISTANCE_MARKER_PIECE_NAME),
        }),
    ]))
}
