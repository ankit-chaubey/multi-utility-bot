use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// ── Shared mutable state ──────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct BotState {
    /// (chat_id, user_id) → list of warning reasons
    pub warnings: HashMap<(i64, i64), Vec<String>>,
    /// (chat_id, note_name) → note content
    pub notes: HashMap<(i64, String), String>,
    /// chat_id → active number-guessing game
    pub games: HashMap<i64, NumberGame>,
}

#[derive(Debug)]
pub struct NumberGame {
    pub secret: u32,
    pub attempts: u32,
    pub max_attempts: u32,
    pub _player_id: i64,
    pub player_name: String,
}

pub type SharedState = Arc<Mutex<BotState>>;

pub fn new_state() -> SharedState {
    Arc::new(Mutex::new(BotState::default()))
}
