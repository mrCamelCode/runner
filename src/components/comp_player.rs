use thomas::{Component, Timer};

#[derive(Component)]
pub struct Player {
    pub jump_timer: Timer,
    pub num_times_jumped_since_landing: u8,
    pub gravity_timer :Timer,
    pub velocity_timer: Timer,
    pub vertical_velocity: i64,
    pub is_on_ground: bool,
    pub distance_traveled: u64,
    pub lives: u8,
}
