use rand::Rng;
use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::{SendDiceParams, SendMessageParams};


const JOKES: &[&str] = &[
    "Why do Rust programmers never get lost?\nBecause they always follow the ownership rules! 🦀",
    "Why did the Rust program crash?\nIt tried to borrow a reference after the owner left! 😅",
    "What's a JavaScript developer's favourite Rust feature?\nThe garbage collector… wait. 🤔",
    "Why don't Rust developers get into fights?\nBecause they know that battles lead to data races! 🔐",
    "How many Rust developers does it take to change a light bulb?\nNone — the borrow checker won't let you touch it! 💡",
    "I told a joke about memory leaks… but nobody laughed, they just kept forgetting it. 💾",
    "Debugging is like being the detective in a crime movie where you're also the murderer. 🔍",
    "A programmer's partner says 'go to the store and get a gallon of milk; if they have eggs, get a dozen.' They return with 12 gallons of milk. 🥛",
    "Why do programmers prefer dark mode? Because light attracts bugs! 🐛",
    "Why was the developer unhappy at their job? They wanted arrays but only got a list of complaints! 📋",
];

const QUOTES: &[&str] = &[
    "\"The best code is no code at all.\" — Jeff Atwood",
    "\"First, solve the problem. Then, write the code.\" — John Johnson",
    "\"Code is like humor. When you have to explain it, it's bad.\" — Cory House",
    "\"Any fool can write code that a computer can understand. Good programmers write code that humans can understand.\" — Martin Fowler",
    "\"Programs must be written for people to read, and only incidentally for machines to execute.\" — Harold Abelson",
    "\"Simplicity is the soul of efficiency.\" — Austin Freeman",
    "\"The most disastrous thing that you can ever learn is your first programming language.\" — Alan Kay",
    "\"The function of good software is to make the complex appear to be simple.\" — Grady Booch",
    "\"Talk is cheap. Show me the code.\" — Linus Torvalds",
    "\"Software is like entropy: It is difficult to grasp, weighs nothing, and obeys the Second Law of Thermodynamics; i.e., it always increases.\" — Norman Augustine",
];

const FACTS: &[&str] = &[
    "🦀 Rust has been voted the \"most loved programming language\" by Stack Overflow every year since 2016!",
    "⚡ Rust achieves memory safety without a garbage collector — through ownership and borrowing at compile time.",
    "🔢 The first computer bug was an actual bug — a moth found in a relay of the Harvard Mark II computer in 1947.",
    "🌐 The Internet was originally called ARPANET, created in 1969 by the US Department of Defense.",
    "💾 The first hard disk drive (IBM 350) stored 3.75 MB and was the size of two refrigerators.",
    "🐧 Linux powers over 96% of the world's top 1 million web servers.",
    "📱 There are more mobile phones on Earth than there are people.",
    "🤖 The term 'robot' comes from the Czech word 'robota', meaning 'forced labor', coined in 1920.",
    "🔐 The SHA-256 hashing algorithm has more possible outputs than there are atoms in the observable universe.",
    "💡 The first computer programmer was Ada Lovelace, who wrote an algorithm for Charles Babbage's Analytical Engine in 1843.",
    "🌍 Approximately 90% of the world's data was created in the last two years.",
    "📡 Wi-Fi stands for nothing — it's just a marketing name created by a branding company in 1999.",
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

pub async fn cmd_dice(bot: &Bot, msg: &Message) {
    let _ = bot.send_dice(msg.chat.id, Some(SendDiceParams::new())).await;
}

pub async fn cmd_roll(bot: &Bot, msg: &Message, args: &str) {
    let sides: u32 = args.trim().parse().unwrap_or(6).max(2).min(1000);
    let roll = rand::thread_rng().gen_range(1..=sides);
    let _ = bot.send_message(
        msg.chat.id,
        format!("🎲 Rolling a d{sides}…\n\n**Result: {roll}**"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_flip(bot: &Bot, msg: &Message) {
    let result = if rand::thread_rng().gen_bool(0.5) { "🪙 Heads!" } else { "🪙 Tails!" };
    let _ = bot.send_message(msg.chat.id, result, None).await;
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
        let _ = bot.send_message(msg.chat.id, "🎱 Ask me a question! Usage: /8ball Will it rain today?", None).await;
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
    use crate::kb::{btn, row};
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
    let emoji = match bot_choice {
        "rock"     => "✊",
        "scissors" => "✌️",
        "paper"    => "🖐",
        _          => "❓",
    };
    let outcome = match (player, bot_choice) {
        (p, b) if p == b                                        => "🤝 It's a tie!",
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => "🎉 You win!",
        _ => "😅 Bot wins!",
    };
    let player_emoji = match player {
        "rock"     => "✊",
        "scissors" => "✌️",
        "paper"    => "🖐",
        _          => "❓",
    };
    format!("You: {player_emoji}  vs  Bot: {emoji}\n\n{outcome}")
}
