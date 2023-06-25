use thomas::{Layer, Rgb};

use crate::components::TimeOfDay;

pub type TimeOfDayColors = [(TimeOfDay, Option<Rgb>); 6];

pub const EVENT_TIME_OF_DAY_CHANGE: &str = "event-time-change";

pub const SKYLINE_LAYER: Layer = Layer(-2);
pub const STAR_LAYER: Layer = Layer(-4);
pub const SUN_LAYER: Layer = Layer(-3);

pub const SCREEN_HEIGHT: u16 = 10;
pub const SCREEN_WIDTH: u16 = 80;

pub const PLAYER_DISPLAY: char = '|';
pub const PLAYER_COLLISION_LAYER: Layer = Layer(1);
pub const PLAYER_X_OFFSET: i64 = 10;
pub const PLAYER_Y_OFFSET: i64 = 2;

pub const GROUND_COLLISION_LAYER: Layer = Layer(2);

pub const OBSTACLE_NAME: &str = "obstacle";
pub const OBSTACLE_BACKGROUND_COLOR: Rgb = Rgb(255, 0, 0);
pub const OBSTACLE_COLLISION_LAYER: Layer = Layer(3);

pub const DISTANCE_MARKER_COLLISION_LAYER: Layer = Layer(4);

pub const STAR_NAME: &str = "star";
pub const STAR_DISPLAY: char = '•';

pub const WINDOW_NAME: &str = "window";
pub const WINDOW_DISPLAY: char = '▪';

pub const SUN_ID: &str = "sun";
pub const SUN_PIECE_NAME: &str = "sun-piece";

pub const BUILDING_PIECE_NAME: &str = "building-piece";

pub const DISTANCE_MARKER_PIECE_NAME: &str = "distance-marker-piece";

pub const STAR_COLORS: TimeOfDayColors = [
    (TimeOfDay::Night, Some(Rgb(219, 219, 219))),
    (TimeOfDay::Dawn, Some(Rgb(54, 68, 112))),
    (TimeOfDay::Morning, None),
    (TimeOfDay::Noon, None),
    (TimeOfDay::Afternoon, None),
    (TimeOfDay::Dusk, Some(Rgb(10, 68, 122))),
];
pub const SKY_COLORS: TimeOfDayColors = [
    (TimeOfDay::Night, Some(Rgb(23, 32, 59))),
    (TimeOfDay::Dawn, Some(Rgb(46, 58, 97))),
    (TimeOfDay::Morning, Some(Rgb(11, 128, 179))),
    (TimeOfDay::Noon, Some(Rgb(12, 140, 196))),
    (TimeOfDay::Afternoon, Some(Rgb(12, 140, 196))),
    (TimeOfDay::Dusk, Some(Rgb(7, 51, 92))),
];
pub const SUN_COLORS: TimeOfDayColors = [
    (TimeOfDay::Night, Some(Rgb(23, 32, 59))),
    (TimeOfDay::Dawn, Some(Rgb(166, 39, 0))),
    (TimeOfDay::Morning, Some(Rgb(224, 166, 4))),
    (TimeOfDay::Noon, Some(Rgb(252, 186, 3))),
    (TimeOfDay::Afternoon, Some(Rgb(252, 186, 3))),
    (TimeOfDay::Dusk, Some(Rgb(166, 39, 0))),
];
pub const BUILDING_COLOR: Rgb = Rgb(143, 143, 143);
pub const ALTERNATE_BUILDING_COLOR: Rgb = Rgb(135, 135, 135);
pub const WINDOW_COLOR: Rgb = Rgb(245, 195, 32);

pub const SKY_COLOR_TRANSITION_TIMER_NAME: &str = "sky-color";
pub const STAR_COLOR_TRANSITION_TIMER_NAME: &str = "star-color";
pub const SUN_COLOR_TRANSITION_TIMER_NAME: &str = "sun-color";
pub const WINDOW_COLOR_TRANSITION_TIMER_NAME: &str = "window-color";

pub fn get_color<'a>(colors: &'a TimeOfDayColors, time_of_day: &TimeOfDay) -> &'a Option<Rgb> {
    &colors
        .iter()
        .find(|(tod, _)| tod == time_of_day)
        .expect("Colors collection has a color for every time of day.")
        .1
}
