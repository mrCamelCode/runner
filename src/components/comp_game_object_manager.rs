use thomas::{Timer, Component};

#[derive(Component)]
pub struct GameObjectManager {
  pub obstacle_generation_timer: Timer,
  pub scroll_timer: Timer,
}