//! Multi-Purpose Telegram Bot
//! Built with tgbotrs v0.1.4
//!
//! Library links:
//!   crates.io  — https://crates.io/crates/tgbotrs
//!   docs.rs    — https://docs.rs/tgbotrs
//!   GitHub     — https://github.com/ankit-chaubey/tgbotrs
//!   API spec   — https://core.telegram.org/bots/api

mod cmd;
mod handler;
mod kb;
mod state;

use std::sync::Arc;
use tgbotrs::{Bot, BotCommand, Poller, UpdateHandler};
use state::new_state;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let token = std::env::var("TOKEN").expect("TOKEN is not set — copy .env.example to .env");

    println!("╔══════════════════════════════════════════╗");
    println!("║   Multi-Purpose Bot  •  tgbotrs v0.1.4  ║");
    println!("║   https://crates.io/crates/tgbotrs       ║");
    println!("╚══════════════════════════════════════════╝");

    let bot = Bot::new(&token).await.expect("Failed to connect — check TOKEN");
    let username = bot.me.username.as_deref().unwrap_or("unknown");
    let name     = bot.me.first_name.as_str();
    println!("✅  Logged in as {name} (@{username})");

    register_commands(&bot).await;

    let state   = new_state();
    let handler: UpdateHandler = {
        let state = Arc::clone(&state);
        Box::new(move |bot, update| {
            let state = Arc::clone(&state);
            Box::pin(async move {
                handler::handle_update(bot, update, state).await;
            })
        })
    };

    println!("📡  Long polling started — bot is live!\n");

    Poller::new(bot, handler)
        .timeout(30)
        .limit(100)
        .start()
        .await
        .expect("Polling crashed");
}

async fn register_commands(bot: &Bot) {
    // Telegram only shows the first ~100 commands, so keep it curated
    let commands: Vec<BotCommand> = vec![
        // ── General
        BotCommand { command: "start".into(),      description: "Welcome screen with links".into() },
        BotCommand { command: "help".into(),        description: "Interactive help menu".into() },
        BotCommand { command: "about".into(),       description: "About this bot & tgbotrs library".into() },
        BotCommand { command: "ping".into(),        description: "Check bot response time".into() },
        BotCommand { command: "source".into(),      description: "tgbotrs library links".into() },
        // ── Fun
        BotCommand { command: "dice".into(),        description: "Animated Telegram dice 🎲".into() },
        BotCommand { command: "roll".into(),        description: "Roll an N-sided die (default d6)".into() },
        BotCommand { command: "flip".into(),        description: "Flip a coin 🪙".into() },
        BotCommand { command: "joke".into(),        description: "Random programming joke 😂".into() },
        BotCommand { command: "quote".into(),       description: "Inspiring developer quote 💭".into() },
        BotCommand { command: "fact".into(),        description: "Random tech/programming fact 🤓".into() },
        BotCommand { command: "8ball".into(),       description: "Magic 8-ball 🎱".into() },
        BotCommand { command: "rps".into(),         description: "Rock Paper Scissors ✊✌️🖐".into() },
        BotCommand { command: "choose".into(),      description: "Pick from options (a | b | c)".into() },
        BotCommand { command: "rate".into(),        description: "Rate anything out of 10 ⭐".into() },
        BotCommand { command: "password".into(),    description: "Generate a secure random password 🔐".into() },
        BotCommand { command: "mock".into(),        description: "mOcK tExT aLtErNaTiNg CaSe".into() },
        BotCommand { command: "clap".into(),        description: "Add 👏 between words".into() },
        BotCommand { command: "shrug".into(),       description: "¯\\_(ツ)_/¯".into() },
        BotCommand { command: "tableflip".into(),   description: "(╯°□°）╯︵ ┻━┻".into() },
        BotCommand { command: "unflip".into(),      description: "┬─┬ノ( º _ ºノ)".into() },
        // ── Utility
        BotCommand { command: "echo".into(),        description: "Echo text back".into() },
        BotCommand { command: "reverse".into(),     description: "Reverse text".into() },
        BotCommand { command: "upper".into(),       description: "UPPERCASE text".into() },
        BotCommand { command: "lower".into(),       description: "lowercase text".into() },
        BotCommand { command: "count".into(),       description: "Count chars / words / lines".into() },
        BotCommand { command: "calc".into(),        description: "Calculator with sqrt/abs/floor/ceil".into() },
        BotCommand { command: "b64".into(),         description: "Base64 encode or decode".into() },
        BotCommand { command: "repeat".into(),      description: "Repeat text N times (max 10)".into() },
        BotCommand { command: "ascii".into(),       description: "Text → ASCII codes".into() },
        BotCommand { command: "binary".into(),      description: "Text → binary".into() },
        BotCommand { command: "time".into(),        description: "Current UTC time + unix timestamp".into() },
        BotCommand { command: "id".into(),          description: "Your Telegram ID (reply to see another's)".into() },
        BotCommand { command: "userinfo".into(),    description: "User info (reply to see another's)".into() },
        BotCommand { command: "chatinfo".into(),    description: "Current chat info".into() },
        BotCommand { command: "members".into(),     description: "Member count".into() },
        // ── Games
        BotCommand { command: "guess".into(),       description: "Start number guessing game (1–100)".into() },
        BotCommand { command: "giveup".into(),      description: "Reveal the number and end the game".into() },
        // ── Notes
        BotCommand { command: "save".into(),        description: "Save a note".into() },
        BotCommand { command: "get".into(),         description: "Get a saved note".into() },
        BotCommand { command: "notes".into(),       description: "List all notes in this chat".into() },
        BotCommand { command: "delnote".into(),     description: "Delete a note".into() },
        // ── Polls
        BotCommand { command: "poll".into(),        description: "Create a poll".into() },
        BotCommand { command: "quiz".into(),        description: "Create a quiz (first opt = correct)".into() },
        // ── Admin (group)
        BotCommand { command: "ban".into(),         description: "Ban user (reply) 🔨".into() },
        BotCommand { command: "kick".into(),        description: "Kick user (reply) 👢".into() },
        BotCommand { command: "mute".into(),        description: "Mute user (reply) 🔇".into() },
        BotCommand { command: "unmute".into(),      description: "Unmute user (reply) 🔊".into() },
        BotCommand { command: "warn".into(),        description: "Warn user — auto-bans at 3 (reply) ⚠️".into() },
        BotCommand { command: "warns".into(),       description: "Check user's warnings (reply)".into() },
        BotCommand { command: "clearwarns".into(),  description: "Clear user's warnings (reply)".into() },
        BotCommand { command: "pin".into(),         description: "Pin replied message 📌".into() },
        BotCommand { command: "unpin".into(),       description: "Unpin latest message".into() },
        BotCommand { command: "del".into(),         description: "Delete replied message 🗑️".into() },
        BotCommand { command: "promote".into(),     description: "Promote user to admin (reply) ⬆️".into() },
        BotCommand { command: "demote".into(),      description: "Remove admin rights (reply) ⬇️".into() },
        BotCommand { command: "invite".into(),      description: "Generate new invite link 🔗".into() },
    ];

    match bot.set_my_commands(commands, None).await {
        Ok(_)  => println!("✅  Bot commands registered in Telegram UI"),
        Err(e) => eprintln!("⚠️   Failed to register commands: {e}"),
    }
}
