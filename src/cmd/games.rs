use rand::Rng;
use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::SendMessageParams;
use crate::state::{NumberGame, SharedState};

pub async fn cmd_guess(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let player_id = match msg.from.as_ref() {
        Some(u) => u.id,
        None => return,
    };
    let player_name = msg
        .from
        .as_ref()
        .map(|u| u.first_name.clone())
        .unwrap_or_else(|| "Player".into());

    // ─── Start a new game ───────────────────────────────────────────────
    if args.trim().is_empty() {
        let mut st = state.lock().await;

        if st.games.contains_key(&chat_id) {
            let _ = bot
                .send_message(
                    chat_id,
                    "🎮 A game is already running! Use `/guess <number>` to guess, or /giveup to quit.",
                    Some(SendMessageParams::new().parse_mode("Markdown")),
                )
                .await;
            return;
        }

        let secret = rand::thread_rng().gen_range(1u32..=100);
        st.games.insert(
            chat_id,
            NumberGame {
                secret,
                attempts: 0,
                max_attempts: 7,
                _player_id: player_id,
                player_name: player_name.clone(),
            },
        );

        let _ = bot
            .send_message(
                chat_id,
                format!(
                    "🎮 *Number Guessing Game started!*\n\n\
                     {player_name} is playing!\n\
                     I'm thinking of a number between *1* and *100*.\n\
                     You have *7 attempts*.\n\n\
                     Use `/guess <number>` to guess!"
                ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            )
            .await;
        return;
    }

    // ─── Parse guess ────────────────────────────────────────────────────
    let guess: u32 = match args.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            let _ = bot
                .send_message(chat_id, "❌ Please enter a valid number between 1 and 100.", None)
                .await;
            return;
        }
    };

    if !(1..=100).contains(&guess) {
        let _ = bot
            .send_message(chat_id, "❌ Number must be between 1 and 100!", None)
            .await;
        return;
    }

    // ─── Game logic (NO await inside lock) ───────────────────────────────
    let result = {
        let mut st = state.lock().await;
        let game = match st.games.get_mut(&chat_id) {
            Some(g) => g,
            None => {
                return;
            }
        };

        game.attempts += 1;
        let attempts = game.attempts;
        let max = game.max_attempts;
        let secret = game.secret;
        let name = game.player_name.clone();

        if guess == secret {
            st.games.remove(&chat_id);
            format!(
                "🎉 *Correct!* The number was *{secret}*!\n{name} got it in *{attempts}/{max}* attempts! 🏆"
            )
        } else if attempts >= max {
            st.games.remove(&chat_id);
            format!(
                "💀 *Game over!* You used all {max} attempts.\nThe number was *{secret}*. Better luck next time!"
            )
        } else {
            let hint = if guess < secret { "📈 Too low!" } else { "📉 Too high!" };
            let remaining = max - attempts;
            let bar_fill = "█".repeat(attempts as usize);
            let bar_empty = "░".repeat(remaining as usize);
            format!(
                "{hint} Guess {attempts}/{max}\n`[{bar_fill}{bar_empty}]`\n\n{remaining} attempt(s) left. `/guess <number>`"
            )
        }
    };

    let _ = bot
        .send_message(
            chat_id,
            result,
            Some(SendMessageParams::new().parse_mode("Markdown")),
        )
        .await;
}

pub async fn cmd_giveup(bot: &Bot, msg: &Message, state: &SharedState) {
    let chat_id = msg.chat.id;

    match state.lock().await.games.remove(&chat_id) {
        Some(game) => {
            let _ = bot
                .send_message(
                    chat_id,
                    format!("🏳️ Game over! The number was *{}*.", game.secret),
                    Some(SendMessageParams::new().parse_mode("Markdown")),
                )
                .await;
        }
        None => {
            let _ = bot
                .send_message(chat_id, "❌ No game is currently running.", None)
                .await;
        }
    }
}
