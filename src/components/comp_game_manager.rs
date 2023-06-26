use thomas::{Component, Timer};

#[derive(PartialEq, Eq)]
pub enum GameState {
    WaitingToStart,
    Playing,
    Paused,
    Victory,
    Defeat,
}

#[derive(Component)]
pub struct GameManager {
    pub camera_scroll_timer: Timer,
    pub score: u64,
    pub game_state: GameState,
}
impl GameManager {
    pub fn is_waiting_to_start(&self) -> bool {
        self.game_state == GameState::WaitingToStart
    }

    pub fn is_playing(&self) -> bool {
        self.game_state == GameState::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.game_state == GameState::Paused
    }

    pub fn did_win(&self) -> bool {
        self.game_state == GameState::Victory
    }

    pub fn did_lose(&self) -> bool {
        self.game_state == GameState::Defeat
    }

    /// Game over is when the game state is either Victory or Defeat.
    pub fn is_game_over(&self) -> bool {
        match self.game_state {
            GameState::Victory | GameState::Defeat => true,
            _ => false,
        }
    }
}
