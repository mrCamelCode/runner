use thomas::{Layer, Rgb};

use crate::components::TimeOfDay;

pub type TimeOfDayColors = [(TimeOfDay, Option<Rgb>); 7];

pub const SKYLINE_LAYER: Layer = Layer(-1);
pub const STAR_LAYER: Layer = Layer(-2);

pub const SCREEN_HEIGHT: u16 = 10;
pub const SCREEN_WIDTH: u16 = 80;

pub const PLAYER_DISPLAY: char = '|';
pub const PLAYER_COLLISION_LAYER: Layer = Layer(1);
pub const PLAYER_X_OFFSET: i64 = 10;
pub const PLAYER_Y_OFFSET: i64 = 2;

pub const GROUND_COLLISION_LAYER: Layer = Layer(2);

pub const OBSTACLE_COLLISION_LAYER: Layer = Layer(3);

pub const STAR_NAME: &str = "star";
pub const STAR_DISPLAY: char = '•';

pub const SUN_ID: &str = "sun";
pub const SUN_PIECE_NAME: &str = "sun-piece";

pub const STAR_COLORS: TimeOfDayColors = [
    (TimeOfDay::Night, Some(Rgb(219, 219, 219))),
    (TimeOfDay::Dawn, Some(Rgb(54, 68, 112))),
    (TimeOfDay::Morning, None),
    (TimeOfDay::Noon, None),
    (TimeOfDay::Afternoon, None),
    (TimeOfDay::Evening, None),
    (TimeOfDay::Dusk, Some(Rgb(10, 68, 122))),
];
pub const SKY_COLORS: TimeOfDayColors = [
    (TimeOfDay::Night, Some(Rgb(23, 32, 59))),
    (TimeOfDay::Dawn, Some(Rgb(46, 58, 97))),
    (TimeOfDay::Morning, Some(Rgb(11, 128, 179))),
    (TimeOfDay::Noon, Some(Rgb(12, 140, 196))),
    (TimeOfDay::Afternoon, Some(Rgb(12, 140, 196))),
    (TimeOfDay::Evening, Some(Rgb(46, 58, 97))),
    (TimeOfDay::Dusk, Some(Rgb(7, 51, 92))),
];
pub const BUILDING_COLOR: Rgb = Rgb(143, 143, 143);

pub fn get_color<'a>(colors: &'a TimeOfDayColors, time_of_day: &TimeOfDay) -> &'a Option<Rgb> {
    &colors
        .iter()
        .find(|(tod, _)| tod == time_of_day)
        .expect("Colors collection has a color for every time of day.")
        .1
}
