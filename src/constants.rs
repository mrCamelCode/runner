use thomas::Layer;

pub const SCREEN_HEIGHT: u16 = 10;
pub const SCREEN_WIDTH: u16 = 80;

pub const PLAYER_DISPLAY: char = '|';
pub const PLAYER_COLLISION_LAYER: Layer = Layer(1);
pub const PLAYER_X_OFFSET: i64 = 3;
pub const PLAYER_Y_OFFSET: i64 = 2;

pub const GROUND_COLLISION_LAYER: Layer = Layer(2);

pub const OBSTACLE_COLLISION_LAYER: Layer = Layer(3);