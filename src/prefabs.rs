use thomas::{
    Component, Dimensions2d, GameCommand, GameCommandsArg, IntCoords2d, Layer, Matrix, Rgb,
    TerminalRenderer, TerminalTransform,
};

use crate::{PLAYER_Y_OFFSET, SCREEN_HEIGHT, BUILDING_COLOR};

pub fn add_building(commands: GameCommandsArg, x_coord: i64, size: Dimensions2d) {
    let building_shape_matrix = Matrix::new(size, || ());

    let start_coords = IntCoords2d::new(x_coord, SCREEN_HEIGHT as i64 - PLAYER_Y_OFFSET - size.height() as i64 + 1);

    for cell in &building_shape_matrix {
        commands.borrow_mut().issue(GameCommand::AddEntity(vec![
            Box::new(TerminalRenderer {
                display: ' ',
                layer: Layer::above(&Layer::furthest_background()),
                foreground_color: None,
                background_color: Some(BUILDING_COLOR),
            }),
            Box::new(TerminalTransform {
                coords: start_coords + *cell.location(),
            }),
        ]));
    }
}
