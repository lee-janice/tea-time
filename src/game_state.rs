use std::time::Instant;

pub struct GameState {
  pub player_won: bool,
  pub player_lost: bool,
  pub start_time: Instant,
  pub tea_time: Option<Instant>,
}
