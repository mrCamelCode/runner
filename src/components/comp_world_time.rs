use std::collections::HashMap;

use thomas::{Component, Timer};

pub const NOON_TIME: u8 = 12;
pub const SUNRISE_TIME: u8 = 5;
pub const SUNSET_TIME: u8 = 19;

#[derive(PartialEq, Eq, Hash)]
pub enum TimeOfDay {
    Night,
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
}

#[derive(Component)]
pub struct WorldTime {
    pub current_time: u8,
    pub advance_time_timer: Timer,
    pub color_transition_timers: HashMap<&'static str, Timer>,
}
impl WorldTime {
    pub fn time_of_day(&self) -> TimeOfDay {
        const HOUR_AFTER_SUNRISE: u8 = SUNRISE_TIME + 1;
        const HOUR_BEFORE_SUNSET: u8 = SUNSET_TIME - 1;

        match self.current_time {
            SUNSET_TIME..=23 | 0..=SUNRISE_TIME => TimeOfDay::Night,
            HOUR_AFTER_SUNRISE..=8 => TimeOfDay::Dawn,
            9..=11 => TimeOfDay::Morning,
            NOON_TIME => TimeOfDay::Noon,
            13..=16 => TimeOfDay::Afternoon,
            17..=HOUR_BEFORE_SUNSET => TimeOfDay::Dusk,
            _ => TimeOfDay::Night,
        }
    }

    pub fn is_light(&self) -> bool {
        match self.time_of_day() {
            TimeOfDay::Morning | TimeOfDay::Noon | TimeOfDay::Afternoon => {
                true
            }
            _ => false,
        }
    }
}
