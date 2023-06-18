use thomas::{Component, Timer};

#[derive(Component)]
pub struct Player {
    pub jump_timer: Timer,
    pub gravity_timer :Timer,
    pub velocity_timer: Timer,
    pub vertical_velocity: i64,
    pub is_on_ground: bool,
}
