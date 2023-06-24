use thomas::{Timer, Component};

#[derive(Component)]
pub struct GameManager {
  pub camera_scroll_timer: Timer,
}