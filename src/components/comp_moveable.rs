use thomas::{Timer, Component};

#[derive(Component)]
pub struct Moveable {
    pub move_timer: Timer,
    pub move_interval: u128,
}
