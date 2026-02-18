mod cmd;
mod handler;
mod kb;
mod state;

use tgbotrs::{Bot, Poller, UpdateHandler};
use state::new_state;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load .env
    let _ = dotenvy::dotenv();

    let token = std::env::var("TOKEN").expect("TOKEN not set in .env");

    println!("🦀 multipurpose-bot starting (tgbotrs v0.1.4)");

    let bot = Bot::new(&token).await.expect("Failed to create bot — check your TOKEN");
    let username = bot.me.username.as_deref().unwrap_or("unknown");
    println!("✅ Logged in as @{username}");
    println!("📡 Starting long polling…");

    // Register bot commands in Telegram UI
    register_commands(&bot).await;

    // Shared state
    let state = new_state();

    // Build the update handler
    let handler: UpdateHandler = {
        let state = Arc::clone(&state);
        Box::new(move |bot, update| {
            let state = Arc::clone(&state);
            Box::pin(async move {
                handler::handle_update(bot, update, state).await;
            })
        })
    };

    // Start polling
    Poller::new(bot, handler)
        .timeout(30)
        .limit(100)
        .start()
        .await
        .expect("Polling failed");
}

async fn register_commands(bot: &Bot) {
    use tgbotrs::BotCommand;


    let commands = vec![
        // General
        BotCommand { command: "start".into(),    description: "Welcome message".into() },
        BotCommand { command: "help".into(),     description: "Show all commands".into() },
        BotCommand { command: "about".into(),    description: "About this bot".into() },
        BotCommand { command: "ping".into(),     description: "Check bot latency".into() },
        // Fun
        BotCommand { command: "dice".into(),     description: "Roll a Telegram dice 🎲".into() },
        BotCommand { command: "roll".into(),     description: "Roll an N-sided die".into() },
        BotCommand { command: "flip".into(),     description: "Flip a coin 🪙".into() },
        BotCommand { command: "joke".into(),     description: "Random joke 😂".into() },
        BotCommand { command: "quote".into(),    description: "Inspiring quote 💭".into() },
        BotCommand { command: "fact".into(),     description: "Random tech fact 🤓".into() },
        BotCommand { command: "8ball".into(),    description: "Magic 8-ball 🎱".into() },
        BotCommand { command: "rps".into(),      description: "Rock Paper Scissors ✊".into() },
        // Utility
        BotCommand { command: "echo".into(),     description: "Echo text back".into() },
        BotCommand { command: "reverse".into(),  description: "Reverse text".into() },
        BotCommand { command: "upper".into(),    description: "Uppercase text".into() },
        BotCommand { command: "lower".into(),    description: "Lowercase text".into() },
        BotCommand { command: "count".into(),    description: "Count chars/words/lines".into() },
        BotCommand { command: "calc".into(),     description: "Calculator (with ^ and parens)".into() },
        BotCommand { command: "b64".into(),      description: "Base64 encode/decode".into() },
        BotCommand { command: "repeat".into(),   description: "Repeat text N times".into() },
        BotCommand { command: "time".into(),     description: "Current UTC time".into() },
        BotCommand { command: "id".into(),       description: "Show Telegram user/chat ID".into() },
        // Info
        BotCommand { command: "userinfo".into(), description: "User info (reply to see theirs)".into() },
        BotCommand { command: "chatinfo".into(), description: "Chat info".into() },
        BotCommand { command: "members".into(),  description: "Member count".into() },
        // Games
        BotCommand { command: "guess".into(),    description: "Number guessing game".into() },
        BotCommand { command: "giveup".into(),   description: "Give up current game".into() },
        // Notes
        BotCommand { command: "save".into(),     description: "Save a note".into() },
        BotCommand { command: "get".into(),      description: "Get a note".into() },
        BotCommand { command: "notes".into(),    description: "List all notes".into() },
        BotCommand { command: "delnote".into(),  description: "Delete a note".into() },
        // Polls
        BotCommand { command: "poll".into(),     description: "Create a poll".into() },
        BotCommand { command: "quiz".into(),     description: "Create a quiz".into() },
        // Admin (group)
        BotCommand { command: "ban".into(),      description: "Ban user (reply)".into() },
        BotCommand { command: "kick".into(),     description: "Kick user (reply)".into() },
        BotCommand { command: "mute".into(),     description: "Mute user (reply)".into() },
        BotCommand { command: "unmute".into(),   description: "Unmute user (reply)".into() },
        BotCommand { command: "warn".into(),     description: "Warn user (reply)".into() },
        BotCommand { command: "warns".into(),    description: "Check warnings (reply)".into() },
        BotCommand { command: "clearwarns".into(), description: "Clear warnings (reply)".into() },
        BotCommand { command: "pin".into(),      description: "Pin message (reply)".into() },
        BotCommand { command: "unpin".into(),    description: "Unpin last message".into() },
        BotCommand { command: "del".into(),      description: "Delete message (reply)".into() },
        BotCommand { command: "promote".into(),  description: "Promote user to admin (reply)".into() },
        BotCommand { command: "demote".into(),   description: "Remove admin rights (reply)".into() },
    ];

    match bot.set_my_commands(commands, None).await {
        Ok(_)  => println!("✅ Bot commands registered"),
        Err(e) => eprintln!("⚠️  Failed to register commands: {e}"),
    }
}
