use rand::Rng;
use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::{SendDiceParams, SendMessageParams};
use crate::kb::{btn, row};

// ─── Content arrays ───────────────────────────────────────────────────────────

const JOKES: &[&str] = &[
    "Why do Rust programmers never get lost?\nBecause they always follow the ownership rules! 🦀",
    "Why did the Rust program crash?\nIt tried to borrow a reference after the owner left! 😅",
    "What's a JavaScript developer's favourite Rust feature?\nThe garbage collector… wait. 🤔",
    "How many Rust developers does it take to change a light bulb?\nNone — the borrow checker won't let you touch it! 💡",
    "I told a joke about memory leaks… nobody laughed, they just kept forgetting it. 💾",
    "Debugging is like being the detective in a crime movie where you're also the murderer. 🔍",
    "A programmer's partner says: 'Get milk; if they have eggs, get a dozen.'\nThey come back with 12 gallons of milk. 🥛",
    "Why do programmers prefer dark mode? Because light attracts bugs! 🐛",
    "A SQL query walks into a bar, walks up to two tables and asks: 'Can I join you?' 🍺",
    "Why did the Java developer wear glasses? Because they couldn't C#! 😎",
    "How do you comfort a JavaScript bug? You console it. 🖥️",
    "There are 10 kinds of people: those who understand binary and those who don't. 0️⃣1️⃣",
    "Why did the programmer quit? Because he didn't get arrays (a raise). 💸",
    "What's a computer's favourite snack? Microchips! 🍟",
    "Why do programmers always mix up Halloween and Christmas?\nOct 31 == Dec 25! 🎃",
    "Why was the developer unhappy at their job?\nThey wanted arrays but only got a list of complaints. 📋",
    "Why do Python programmers prefer snake_case?\nBecause they can't C camelCase without hissing. 🐍",
    "!false — it's funny because it's true. 😄",
];

const QUOTES: &[&str] = &[
    "\"The best code is no code at all.\" — Jeff Atwood",
    "\"First, solve the problem. Then, write the code.\" — John Johnson",
    "\"Code is like humor. When you have to explain it, it's bad.\" — Cory House",
    "\"Any fool can write code a computer understands. Good programmers write code humans understand.\" — Martin Fowler",
    "\"Programs must be written for people to read, and only incidentally for machines to execute.\" — Harold Abelson",
    "\"Talk is cheap. Show me the code.\" — Linus Torvalds",
    "\"The function of good software is to make the complex appear simple.\" — Grady Booch",
    "\"Make it work, make it right, make it fast.\" — Kent Beck",
    "\"Premature optimization is the root of all evil.\" — Donald Knuth",
    "\"The most dangerous phrase is: we've always done it this way.\" — Grace Hopper",
    "\"Measuring programming progress by lines of code is like measuring aircraft building progress by weight.\" — Bill Gates",
    "\"The only way to learn a new programming language is by writing programs in it.\" — Dennis Ritchie",
    "\"Simplicity is the soul of efficiency.\" — Austin Freeman",
    "\"Every great developer you know got there by solving problems they were unqualified to solve until they did it.\" — Patrick McKenzie",
];

const FACTS: &[&str] = &[
    "🦀 Rust has been voted the 'most loved language' by Stack Overflow every year since 2016!",
    "⚡ Rust achieves memory safety without a garbage collector — via ownership + borrowing at compile time.",
    "🔢 The first computer bug was a real insect — a moth found in the Harvard Mark II relay in 1947.",
    "🌐 The Internet began as ARPANET, created in 1969 by the US Department of Defense.",
    "💾 The first hard disk (IBM 350) stored just 3.75 MB and was the size of two refrigerators.",
    "🐧 Linux powers over 96% of the world's top 1 million web servers.",
    "🤖 The word 'robot' comes from Czech 'robota' meaning 'forced labor', coined in 1920.",
    "🔐 SHA-256 has more possible outputs than there are atoms in the observable universe.",
    "💡 Ada Lovelace was the world's first programmer, writing an algorithm for Babbage's engine in 1843.",
    "📡 Wi-Fi is not an acronym — it's a marketing name invented by a branding agency in 1999.",
    "🖱️ The computer mouse was invented by Douglas Engelbart in 1964 — and was made of wood.",
    "⌨️ The QWERTY keyboard was designed to slow typists down to prevent typewriter jams.",
    "🕹️ The first video game, 'Tennis for Two', was created in 1958 on an oscilloscope.",
    "🐍 Python was named after Monty Python's Flying Circus, not the snake.",
    "🌍 Approximately 90% of the world's data was created in the last two years.",
    "📦 The Rust package registry, crates.io, hosts over 140,000 crates.",
    "🔋 Approximately 3.5 billion Google searches happen every single day.",
    "🦁 The @ symbol was almost extinct before email saved it in 1971.",
];

const EIGHT_BALL: &[&str] = &[
    "✅ It is certain.",
    "✅ It is decidedly so.",
    "✅ Without a doubt.",
    "✅ Yes, definitely.",
    "✅ You may rely on it.",
    "✅ As I see it, yes.",
    "✅ Most likely.",
    "✅ Outlook good.",
    "✅ Yes.",
    "✅ Signs point to yes.",
    "🤷 Reply hazy, try again.",
    "🤷 Ask again later.",
    "🤷 Better not tell you now.",
    "🤷 Cannot predict now.",
    "🤷 Concentrate and ask again.",
    "❌ Don't count on it.",
    "❌ My reply is no.",
    "❌ My sources say no.",
    "❌ Outlook not so good.",
    "❌ Very doubtful.",
];

const RATE_COMMENTS: &[&str] = &[
    "💀 Absolute disaster.",
    "😬 Yikes. Just... yikes.",
    "😞 Pretty rough.",
    "😐 Not great, not terrible.",
    "🙂 Slightly below average.",
    "😊 Right in the middle!",
    "👍 Decent enough.",
    "😄 Pretty good actually!",
    "🔥 Impressive!",
    "🤩 Outstanding!",
    "💯 Absolutely perfect!",
];

// ─── Commands ─────────────────────────────────────────────────────────────────

pub async fn cmd_dice(bot: &Bot, msg: &Message) {
    let _ = bot.send_dice(msg.chat.id, Some(SendDiceParams::new())).await;
}

pub async fn cmd_roll(bot: &Bot, msg: &Message, args: &str) {
    let sides: u32 = args.trim().parse().unwrap_or(6).max(2).min(1000);
    let roll = rand::thread_rng().gen_range(1..=sides);
    let _ = bot.send_message(
        msg.chat.id,
        format!("🎲 Rolling a *d{sides}*…\n\nResult: *{roll}*"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_flip(bot: &Bot, msg: &Message) {
    let result = if rand::thread_rng().gen_bool(0.5) { "🪙 *Heads!*" } else { "🪙 *Tails!*" };
    let _ = bot.send_message(
        msg.chat.id,
        result,
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_joke(bot: &Bot, msg: &Message) {
    let joke = JOKES[rand::thread_rng().gen_range(0..JOKES.len())];
    let _ = bot.send_message(msg.chat.id, joke, None).await;
}

pub async fn cmd_quote(bot: &Bot, msg: &Message) {
    let quote = QUOTES[rand::thread_rng().gen_range(0..QUOTES.len())];
    let _ = bot.send_message(msg.chat.id, quote, None).await;
}

pub async fn cmd_fact(bot: &Bot, msg: &Message) {
    let fact = FACTS[rand::thread_rng().gen_range(0..FACTS.len())];
    let _ = bot.send_message(msg.chat.id, fact, None).await;
}

pub async fn cmd_8ball(bot: &Bot, msg: &Message, question: &str) {
    if question.trim().is_empty() {
        let _ = bot.send_message(
            msg.chat.id,
            "🎱 Ask me a question!\nUsage: `/8ball Will it rain today?`",
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
        return;
    }
    let answer = EIGHT_BALL[rand::thread_rng().gen_range(0..EIGHT_BALL.len())];
    let _ = bot.send_message(
        msg.chat.id,
        format!("🎱 *Question:* _{question}_\n\n{answer}"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_rps(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(
        msg.chat.id,
        "✊✌️🖐 *Rock Paper Scissors!*\n\nPick your move:",
        Some(SendMessageParams::new()
            .parse_mode("Markdown")
            .reply_markup(row(vec![
                btn("✊ Rock", "rps:rock"),
                btn("✌️ Scissors", "rps:scissors"),
                btn("🖐 Paper", "rps:paper"),
            ]))
        ),
    ).await;
}

pub fn rps_result(player: &str) -> String {
    let choices = ["rock", "scissors", "paper"];
    let bot_choice = choices[rand::thread_rng().gen_range(0..3)];
    let bot_emoji  = match bot_choice { "rock" => "✊", "scissors" => "✌️", _ => "🖐" };
    let user_emoji = match player      { "rock" => "✊", "scissors" => "✌️", _ => "🖐" };
    let outcome = match (player, bot_choice) {
        (p, b) if p == b => "🤝 It's a tie!",
        ("rock","scissors") | ("scissors","paper") | ("paper","rock") => "🎉 You win!",
        _ => "😅 Bot wins!",
    };
    format!("You: {user_emoji}  vs  Bot: {bot_emoji}\n\n{outcome}")
}

/// /choose option1 | option2 | option3
pub async fn cmd_choose(bot: &Bot, msg: &Message, args: &str) {
    let opts: Vec<&str> = args.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if opts.len() < 2 {
        let _ = bot.send_message(
            msg.chat.id,
            "Usage: `/choose pizza | sushi | tacos`",
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
        return;
    }
    let pick = opts[rand::thread_rng().gen_range(0..opts.len())];
    let _ = bot.send_message(
        msg.chat.id,
        format!("🎯 I choose: *{pick}*"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

/// /rate <anything>
pub async fn cmd_rate(bot: &Bot, msg: &Message, thing: &str) {
    if thing.trim().is_empty() {
        let _ = bot.send_message(
            msg.chat.id,
            "Usage: `/rate coffee`",
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
        return;
    }
    let score = rand::thread_rng().gen_range(0usize..=10);
    let comment = RATE_COMMENTS[score];
    let filled  = "█".repeat(score);
    let empty   = "░".repeat(10 - score);
    let _ = bot.send_message(
        msg.chat.id,
        format!("⭐ *{thing}*\n\n`[{filled}{empty}]` *{score}/10*\n\n{comment}"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

/// /password [length]
pub async fn cmd_password(bot: &Bot, msg: &Message, args: &str) {
    let len: usize = args.trim().parse().unwrap_or(16).max(6).min(64);
    const CHARSET: &[u8] =
        b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789!@#$%^&*";

    // generate password WITHOUT keeping rng alive
    let password: String = (0..len)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "🔐 *Generated password ({len} chars):*\n`{password}`\n\n_Delete this message after copying!_"
        ),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    )
    .await;
}

/// /mock <text> — aLtErNaTiNg CaSe
pub async fn cmd_mock(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /mock <text>", None).await;
        return;
    }
    let mocked: String = args.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 { c.to_uppercase().next().unwrap_or(c) }
        else          { c.to_lowercase().next().unwrap_or(c) }
    }).collect();
    let _ = bot.send_message(msg.chat.id, mocked, None).await;
}

/// /clap <text> — Add 👏 between words
pub async fn cmd_clap(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /clap <text>", None).await;
        return;
    }
    let clapd = args.split_whitespace().collect::<Vec<_>>().join(" 👏 ");
    let _ = bot.send_message(msg.chat.id, format!("👏 {clapd} 👏"), None).await;
}

/// /shrug
pub async fn cmd_shrug(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(msg.chat.id, r"¯\_(ツ)_/¯", None).await;
}

/// /tableflip
pub async fn cmd_tableflip(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(msg.chat.id, "(╯°□°）╯︵ ┻━┻", None).await;
}

/// /unflip
pub async fn cmd_unflip(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(msg.chat.id, "┬─┬ノ( º _ ºノ)", None).await;
}
