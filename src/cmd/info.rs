use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::SendMessageParams;
use crate::kb::{btn, url_btn, kb};

pub async fn cmd_start(bot: &Bot, msg: &Message) {
    let name = msg.from.as_ref().map(|u| u.first_name.as_str()).unwrap_or("there");
    let text = format!(
        "👋 Hello, *{name}*\\!\n\n\
        I'm a *multi\\-purpose bot* built with `tgbotrs v0.1.4` 🦀\n\n\
        I can help with fun, utilities, group admin, games, notes, polls, and more\\!\n\n\
        Use /help to see all commands\\."
    );
    let _ = bot.send_message(
        msg.chat.id,
        text,
        Some(SendMessageParams::new()
            .parse_mode("MarkdownV2")
            .reply_markup(kb(vec![
                vec![btn("📋 Help", "help:main"), btn("ℹ️ About", "help:about")],
                vec![
                    url_btn("📦 tgbotrs", "https://github.com/ankit-chaubey/tgbotrs"),
                    url_btn("👨‍💻 Dev", "https://t.me/ankify"),
                ],
            ]))
        ),
    ).await;
}

pub async fn cmd_help(bot: &Bot, msg: &Message, section: &str) {
    let (title, body) = match section {
        "fun"    => help_fun(),
        "util"   => help_util(),
        "admin"  => help_admin(),
        "games"  => help_games(),
        "notes"  => help_notes(),
        "polls"  => help_polls(),
        "info"   => help_info(),
        _ => help_main(),
    };
    let _ = bot.send_message(
        msg.chat.id,
        format!("{title}\n\n{body}"),
        Some(SendMessageParams::new()
            .parse_mode("HTML")
            .reply_markup(help_kb(section))
        ),
    ).await;
}

pub async fn send_help_section(bot: &Bot, chat_id: i64, message_id: i64, section: &str) {
    use tgbotrs::gen_methods::EditMessageTextParams;
    let (title, body) = match section {
        "fun"   => help_fun(),
        "util"  => help_util(),
        "admin" => help_admin(),
        "games" => help_games(),
        "notes" => help_notes(),
        "polls" => help_polls(),
        "info"  => help_info(),
        _       => help_main(),
    };
    let params = EditMessageTextParams::new()
        .chat_id(chat_id)
        .message_id(message_id)
        .parse_mode("HTML")
        .reply_markup(Box::new(match help_kb(section) {
            tgbotrs::ReplyMarkup::InlineKeyboard(kb) => kb,
            _ => unreachable!(),
        }));
    let _ = bot.edit_message_text(format!("{title}\n\n{body}"), Some(params)).await;
}

fn help_kb(section: &str) -> tgbotrs::ReplyMarkup {
    let back = btn("⬅️ Back", "help:main");
    match section {
        "main" => kb(vec![
            vec![btn("🎉 Fun", "help:fun"),     btn("🔧 Utility", "help:util")],
            vec![btn("👮 Admin", "help:admin"), btn("🎮 Games", "help:games")],
            vec![btn("📝 Notes", "help:notes"), btn("📊 Polls", "help:polls")],
            vec![btn("ℹ️ Info", "help:info")],
        ]),
        _ => kb(vec![
            vec![btn("🎉 Fun", "help:fun"),     btn("🔧 Utility", "help:util")],
            vec![btn("👮 Admin", "help:admin"), btn("🎮 Games", "help:games")],
            vec![btn("📝 Notes", "help:notes"), btn("📊 Polls", "help:polls")],
            vec![back],
        ]),
    }
}

fn help_main() -> (&'static str, String) {
    ("📚 <b>Command Categories</b>", "Pick a category below to see available commands.".into())
}

fn help_fun() -> (&'static str, String) {
    ("🎉 <b>Fun Commands</b>", "\
/dice — Roll a Telegram dice 🎲
/roll [N] — Roll a N-sided die (default: d6)
/flip — Flip a coin 🪙
/joke — Get a random joke 😂
/quote — Get an inspiring quote 💭
/fact — Get a random tech fact 🤓
/8ball &lt;question&gt; — Ask the magic 8-ball 🎱
/rps — Play Rock Paper Scissors ✊✌️🖐".into())
}

fn help_util() -> (&'static str, String) {
    ("🔧 <b>Utility Commands</b>", "\
/echo &lt;text&gt; — Echo text back
/reverse &lt;text&gt; — Reverse text
/upper &lt;text&gt; — UPPERCASE text
/lower &lt;text&gt; — lowercase text
/count &lt;text&gt; — Count chars/words/lines
/calc &lt;expr&gt; — Calculator (supports +−×÷^ and parentheses)
/b64 encode/decode &lt;text&gt; — Base64 encode/decode
/repeat &lt;N&gt; &lt;text&gt; — Repeat text N times (max 10)
/time — Current UTC time
/id — Show your Telegram ID".into())
}

fn help_admin() -> (&'static str, String) {
    ("👮 <b>Admin Commands</b> <i>(group only)</i>", "\
Reply to a user's message, then:
/ban — Permanently ban user
/kick — Kick (remove but can rejoin)
/mute — Mute user (remove send rights)
/unmute — Restore user's send rights
/warn [reason] — Warn a user
/warns — Check user's warnings
/clearwarns — Clear all user's warnings
/pin — Pin replied message
/unpin — Unpin latest pinned message
/del — Delete replied message
/promote — Promote user to admin
/demote — Remove admin rights
/members — Show member count".into())
}

fn help_games() -> (&'static str, String) {
    ("🎮 <b>Game Commands</b>", "\
/guess — Start a number guessing game (1–100)
/guess &lt;number&gt; — Make a guess
/giveup — Give up current game".into())
}

fn help_notes() -> (&'static str, String) {
    ("📝 <b>Notes Commands</b>", "\
/save &lt;name&gt; &lt;content&gt; — Save a note
/get &lt;name&gt; — Retrieve a note
/notes — List all notes in this chat
/delnote &lt;name&gt; — Delete a note".into())
}

fn help_polls() -> (&'static str, String) {
    ("📊 <b>Poll Commands</b>", "\
/poll &lt;question&gt; | &lt;opt1&gt; | &lt;opt2&gt; ... — Create a poll
/quiz &lt;question&gt; | &lt;correct answer&gt; | &lt;wrong1&gt; | &lt;wrong2&gt; ... — Create a quiz
<i>Separate question and options with</i> <code>|</code>".into())
}

fn help_info() -> (&'static str, String) {
    ("ℹ️ <b>Info Commands</b>", "\
/start — Welcome message
/help — This help menu
/about — About this bot
/ping — Check bot latency
/id — Your Telegram ID (reply to see another user's)
/userinfo — Info about you (reply to see another user's)
/chatinfo — Info about this chat".into())
}

pub async fn cmd_about(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(
        msg.chat.id,
        "🤖 <b>Multi-Purpose Bot</b>\n\n\
        Built with <code>tgbotrs v0.1.4</code> — a fully auto-generated \
        Rust Telegram Bot API library covering all 285 types and 165 methods.\n\n\
        📦 <a href=\"https://crates.io/crates/tgbotrs\">crates.io/crates/tgbotrs</a>\n\
        🔗 <a href=\"https://github.com/ankit-chaubey/tgbotrs\">GitHub</a>\n\
        👨‍💻 Dev: <a href=\"https://t.me/ankify\">@ankify</a>\n\n\
        <i>Features: fun, utilities, admin tools, games, notes, polls and more!</i>",
        Some(SendMessageParams::new().parse_mode("HTML")),
    ).await;
}

pub async fn cmd_ping(bot: &Bot, msg: &Message) {
    let start = std::time::Instant::now();
    let sent = bot.send_message(msg.chat.id, "🏓 Pinging...", None).await;
    let elapsed = start.elapsed().as_millis();
    if let Ok(sent_msg) = sent {
        use tgbotrs::gen_methods::EditMessageTextParams;
        let params = EditMessageTextParams::new()
            .chat_id(msg.chat.id)
            .message_id(sent_msg.message_id);
        let _ = bot.edit_message_text(
            format!("🏓 Pong! `{elapsed}ms`"),
            Some(params.parse_mode("Markdown")),
        ).await;
    }
}

pub async fn cmd_userinfo(bot: &Bot, msg: &Message) {
    let target = msg.reply_to_message.as_deref().unwrap_or(msg);
    let user = match target.from.as_deref() {
        Some(u) => u,
        None => {
            let _ = bot.send_message(msg.chat.id, "❌ Could not find user info.", None).await;
            return;
        }
    };
    let name = format!("{}{}", user.first_name,
        user.last_name.as_deref().map(|l| format!(" {l}")).unwrap_or_default());
    let username = user.username.as_deref()
        .map(|u| format!("@{u}"))
        .unwrap_or_else(|| "none".to_string());
    let is_bot = if user.is_bot { "✅ Yes" } else { "❌ No" };
    let premium = user.is_premium.map(|p| if p { "✅ Yes" } else { "❌ No" }).unwrap_or("❌ No");
    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "👤 <b>User Info</b>\n\n\
            🔹 <b>Name:</b> {name}\n\
            🔹 <b>Username:</b> {username}\n\
            🔹 <b>User ID:</b> <code>{}</code>\n\
            🔹 <b>Is Bot:</b> {is_bot}\n\
            🔹 <b>Premium:</b> {premium}",
            user.id
        ),
        Some(SendMessageParams::new().parse_mode("HTML")),
    ).await;
}

pub async fn cmd_chatinfo(bot: &Bot, msg: &Message) {
    let chat = &msg.chat;
    let title = chat.title.as_deref()
        .or(chat.first_name.as_deref())
        .unwrap_or("Unknown");
    let username = chat.username.as_deref()
        .map(|u| format!("@{u}"))
        .unwrap_or_else(|| "none".to_string());
    let is_forum = chat.is_forum.unwrap_or(false);
    // Get member count
    let members = bot.get_chat_member_count(chat.id).await
        .map(|n| n.to_string())
        .unwrap_or_else(|_| "N/A".to_string());
    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "💬 <b>Chat Info</b>\n\n\
            🔹 <b>Title:</b> {title}\n\
            🔹 <b>Username:</b> {username}\n\
            🔹 <b>Chat ID:</b> <code>{}</code>\n\
            🔹 <b>Type:</b> {}\n\
            🔹 <b>Forum:</b> {}\n\
            🔹 <b>Members:</b> {members}",
            chat.id, chat.r#type,
            if is_forum { "✅ Yes" } else { "❌ No" }
        ),
        Some(SendMessageParams::new().parse_mode("HTML")),
    ).await;
}

pub async fn cmd_members(bot: &Bot, msg: &Message) {
    match bot.get_chat_member_count(msg.chat.id).await {
        Ok(count) => {
            let _ = bot.send_message(
                msg.chat.id,
                format!("👥 This chat has *{count}* members.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(_) => {
            let _ = bot.send_message(msg.chat.id, "❌ Could not get member count.", None).await;
        }
    }
}
