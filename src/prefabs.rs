use std::{ops::RangeInclusive, rc::Rc};

use rand::{thread_rng, Rng};
use thomas::{
    Alignment, Component, Dimensions2d, GameCommand, GameCommandsArg, Identity, IntCoords2d,
    IntVector2, Layer, Matrix, Rgb, TerminalCollider, TerminalRenderer, TerminalTransform, Text,
    Timer, UiAnchor, Vector2, WorldText,
};

use crate::{
    components::{CleanupOnScreenExit, FollowCamera, Moveable, SkylineBuilding},
    ALTERNATE_BUILDING_COLOR, BUILDING_COLOR, BUILDING_PIECE_NAME, DISTANCE_MARKER_PIECE_NAME,
    OBSTACLE_BACKGROUND_COLOR, OBSTACLE_COLLISION_LAYER, OBSTACLE_NAME, PAUSED_TEXT_NAME,
    PLAYER_Y_OFFSET, SCREEN_HEIGHT, SCREEN_WIDTH, SKYLINE_LAYER, START_PLAYING_TEXT_NAME,
    VICTORY_TEXT_NAME, WINDOW_COLOR, DEFEAT_TEXT_NAME,
};

const OBSTACLE_MOVE_INTERVAL_RANGE_MILLIS: RangeInclusive<u128> = 300..=800;

#[derive(PartialEq, Eq)]
pub enum ObstacleType {
    Ground,
    Air,
}

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
            Box::new(FollowCamera {
                base_position: coords,
                offset: IntCoords2d::zero(),
            }),
            Box::new(Identity {
                id: String::from(""),
                name: String::from(BUILDING_PIECE_NAME),
            }),
            Box::new(SkylineBuilding {
                last_distance_scrolled: 0,
            }),
        ]));
    }
}

pub fn make_obstacle(
    main_cam_transform: &TerminalTransform,
    typ: ObstacleType,
) -> Vec<Box<dyn Component>> {
    let mut comps: Vec<Box<dyn Component>> = vec![
        Box::new(TerminalRenderer {
            background_color: Some(OBSTACLE_BACKGROUND_COLOR),
            foreground_color: None,
            display: ' ',
            layer: Layer::base(),
        }),
        Box::new(TerminalTransform {
            coords: IntCoords2d::new(
                main_cam_transform.coords.x() + SCREEN_WIDTH as i64 + 1,
                match typ {
                    ObstacleType::Ground => SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET,
                    ObstacleType::Air => SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET - 2,
                },
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
        Box::new(CleanupOnScreenExit {}),
    ];

    if typ == ObstacleType::Air {
        comps.push(Box::new(Moveable {
            move_timer: Timer::start_new(),
            move_interval: thread_rng().gen_range(OBSTACLE_MOVE_INTERVAL_RANGE_MILLIS),
        }));
    }

    comps
}

pub fn add_distance_marker(commands: GameCommandsArg, distance: u64) {
    let board_matrix = Matrix::new(Dimensions2d::new(1, 5), || ());

    let base_x_pos = distance;

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
            Box::new(CleanupOnScreenExit {}),
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
            Box::new(CleanupOnScreenExit {}),
        ]))
    }

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(WorldText {
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
        Box::new(TerminalTransform {
            coords: IntCoords2d::new(base_x_pos as i64, board_y_pos),
        }),
        Box::new(CleanupOnScreenExit {}),
    ]))
}

pub fn add_start_playing_text(commands: GameCommandsArg) {
    add_fullscreen_text(
        commands,
        "RUNNER",
        "Press any key to start",
        START_PLAYING_TEXT_NAME,
    );
}

pub fn add_paused_text(commands: GameCommandsArg) {
    add_fullscreen_text(commands, "PAUSED", "Press ESC to resume", PAUSED_TEXT_NAME);
}

pub fn add_victory_text(commands: GameCommandsArg) {
    add_fullscreen_text(
        commands,
        "VICTORY!",
        "Press any key to play again",
        VICTORY_TEXT_NAME,
    );
}

pub fn add_defeat_text(commands: GameCommandsArg) {
    add_fullscreen_text(
        commands,
        "DEFEAT",
        "Press any key to play again",
        DEFEAT_TEXT_NAME,
    );
}

fn add_fullscreen_text(
    commands: GameCommandsArg,
    top_text: &str,
    bottom_text: &str,
    text_name: &str,
) {
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(Text {
            anchor: UiAnchor::Middle,
            justification: Alignment::Middle,
            offset: IntVector2::down(),
            value: String::from(top_text),
            background_color: None,
            foreground_color: Some(Rgb::white()),
        }),
        Box::new(Identity {
            id: String::from(""),
            name: String::from(text_name),
        }),
    ]));

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(Text {
            anchor: UiAnchor::Middle,
            justification: Alignment::Middle,
            offset: IntVector2::up(),
            value: String::from(bottom_text),
            background_color: None,
            foreground_color: Some(Rgb::white()),
        }),
        Box::new(Identity {
            id: String::from(""),
            name: String::from(text_name),
        }),
    ]));
}
