use thomas::{Timer, Component};

#[derive(Component)]
pub struct GameObjectManager {
  pub obstacle_generation_timer: Timer,
  pub extra_life_generation_timer: Timer,
  pub next_obstacle_wait_time: u128,
  pub scroll_timer: Timer,
}