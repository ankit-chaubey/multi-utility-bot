use tgbotrs::{Bot, Message, InlineKeyboardMarkup};
use tgbotrs::gen_methods::{EditMessageTextParams, SendMessageParams};
use crate::kb::{btn, url_btn, kb};

// ─── /start ───────────────────────────────────────────────────────────────────

pub async fn cmd_start(bot: &Bot, msg: &Message) {
    let name = msg.from.as_ref().map(|u| u.first_name.as_str()).unwrap_or("there");
    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "👋 Hello, <b>{name}</b>!\n\n\
             I'm a <b>multi-purpose bot</b> powered by \
             <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs v0.1.4</a> 🦀\n\n\
             I can help with fun, utilities, group admin, games, notes, polls and more!\n\n\
             📦 <a href=\"https://crates.io/crates/tgbotrs\">crates.io</a>  \
             📖 <a href=\"https://docs.rs/tgbotrs\">docs.rs</a>  \
             🔗 <a href=\"https://github.com/ankit-chaubey/tgbotrs\">GitHub</a>\n\n\
             Use /help to see all commands."
        ),
        Some(SendMessageParams::new()
            .parse_mode("HTML")
            .reply_markup(kb(vec![
                vec![btn("📋 Help", "help:main"), btn("ℹ️ About", "help:about")],
                vec![
                    url_btn("📦 tgbotrs", "https://crates.io/crates/tgbotrs"),
                    url_btn("💻 GitHub",  "https://github.com/ankit-chaubey/tgbotrs"),
                ],
                vec![url_btn("📖 docs.rs", "https://docs.rs/tgbotrs")],
            ]))
        ),
    ).await;
}

// ─── /help ────────────────────────────────────────────────────────────────────

pub async fn cmd_help(bot: &Bot, msg: &Message, section: &str) {
    let (title, body) = help_content(section);
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
    let (title, body) = help_content(section);
    let kb_markup = match help_kb(section) {
        tgbotrs::ReplyMarkup::InlineKeyboard(k) => k,
        _ => InlineKeyboardMarkup { inline_keyboard: vec![] },
    };
    let params = EditMessageTextParams::new()
        .chat_id(chat_id)
        .message_id(message_id)
        .parse_mode("HTML")
        .reply_markup(Box::new(kb_markup));
    let _ = bot.edit_message_text(format!("{title}\n\n{body}"), Some(params)).await;
}

fn help_kb(section: &str) -> tgbotrs::ReplyMarkup {
    let nav = vec![
        vec![btn("🎉 Fun", "help:fun"),     btn("🔧 Utility", "help:util")],
        vec![btn("👮 Admin", "help:admin"), btn("🎮 Games", "help:games")],
        vec![btn("📝 Notes", "help:notes"), btn("📊 Polls", "help:polls")],
        vec![btn("ℹ️ Info", "help:info")],
    ];
    match section {
        "main" => kb(nav),
        _ => {
            let mut rows = nav;
            rows.push(vec![btn("⬅️ Back", "help:main")]);
            kb(rows)
        }
    }
}

fn help_content(section: &str) -> (&'static str, &'static str) {
    match section {
        "fun"   => ("🎉 <b>Fun Commands</b>",
            "/dice — Animated Telegram dice 🎲\n\
             /roll [N] — Roll N-sided die (default d6)\n\
             /flip — Flip a coin 🪙\n\
             /joke — Random programming joke 😂\n\
             /quote — Inspiring developer quote 💭\n\
             /fact — Random tech/programming fact 🤓\n\
             /8ball &lt;question&gt; — Magic 8-ball 🎱\n\
             /rps — Rock Paper Scissors (inline buttons)\n\
             /choose &lt;a&gt; | &lt;b&gt; | &lt;c&gt; — Pick randomly from options\n\
             /rate &lt;anything&gt; — Rate something out of 10\n\
             /password [length] — Secure random password\n\
             /mock &lt;text&gt; — aLtErNaTiNg CaSe\n\
             /clap &lt;text&gt; — Add 👏 between words\n\
             /shrug — ¯\\_(ツ)_/¯\n\
             /tableflip — (╯°□°）╯︵ ┻━┻\n\
             /unflip — ┬─┬ノ( º _ ºノ)"),

        "util"  => ("🔧 <b>Utility Commands</b>",
            "/echo &lt;text&gt; — Echo text\n\
             /reverse &lt;text&gt; — Reverse text\n\
             /upper &lt;text&gt; — UPPERCASE\n\
             /lower &lt;text&gt; — lowercase\n\
             /count &lt;text&gt; — Count chars/words/lines\n\
             /calc &lt;expr&gt; — Calculator (+−×÷^ with sqrt/abs/floor/ceil/round)\n\
             /b64 encode/decode &lt;text&gt; — Base64\n\
             /repeat &lt;N&gt; &lt;text&gt; — Repeat N times (max 10)\n\
             /ascii &lt;text&gt; — ASCII codes of characters\n\
             /binary &lt;text&gt; — Text to binary\n\
             /time — Current UTC time + unix timestamp\n\
             /id — Your Telegram ID (reply to see another user's)"),

        "admin" => ("👮 <b>Admin Commands</b> <i>(groups only, reply to a user)</i>",
            "/ban — Permanently ban user\n\
             /kick — Kick (ban + immediate unban)\n\
             /mute — Remove all send permissions\n\
             /unmute — Restore all send permissions\n\
             /warn [reason] — Warn user (auto-bans at 3 warnings)\n\
             /warns — Check user's warning count &amp; reasons\n\
             /clearwarns — Clear all user's warnings\n\
             /pin — Pin replied message\n\
             /unpin — Unpin latest pinned message\n\
             /del — Delete replied message\n\
             /promote — Grant admin rights\n\
             /demote — Remove admin rights\n\
             /members — Show member count\n\
             /invite — Generate new invite link"),

        "games" => ("🎮 <b>Game Commands</b>",
            "/guess — Start a number guessing game (1–100, 7 attempts)\n\
             /guess &lt;number&gt; — Make a guess\n\
             /giveup — Reveal the number and end the game"),

        "notes" => ("📝 <b>Notes Commands</b>",
            "/save &lt;name&gt; &lt;content&gt; — Save a note\n\
             /get &lt;name&gt; — Retrieve a note\n\
             /notes — List all notes in this chat\n\
             /delnote &lt;name&gt; — Delete a note"),

        "polls" => ("📊 <b>Poll Commands</b>",
            "/poll &lt;question&gt; | &lt;opt1&gt; | &lt;opt2&gt; ... — Create a poll\n\
             /quiz &lt;question&gt; | &lt;correct&gt; | &lt;wrong1&gt; ... — Create a quiz\n\
             <i>Separate everything with</i> <code>|</code>"),

        "info" | "about" => ("ℹ️ <b>Info Commands</b>",
            "/start — Welcome screen with links\n\
             /help — Interactive help menu\n\
             /about — About this bot &amp; library\n\
             /ping — Check bot response time\n\
             /id — Your user/chat ID\n\
             /userinfo — User info (reply to see another's)\n\
             /chatinfo — Current chat info\n\
             /members — Member count\n\
             /source — Library source links"),

        _ => ("📚 <b>Command Categories</b>",
            "Pick a category below to browse commands."),
    }
}

// ─── /about ───────────────────────────────────────────────────────────────────

pub async fn cmd_about(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(
        msg.chat.id,
        "🤖 <b>Multi-Purpose Bot v0.2.0</b>\n\n\
         Built with <code>tgbotrs v0.1.4</code> — a fully auto-generated \
         Rust Telegram Bot API library.\n\n\
         <b>Library details:</b>\n\
         • 285 types · 165 methods\n\
         • Auto-generated from the official Telegram Bot API spec\n\
         • Zero unsafe code · Full async/await\n\
         • MIT licensed\n\n\
         <b>Links:</b>\n\
         📦 <a href=\"https://crates.io/crates/tgbotrs\">crates.io/crates/tgbotrs</a>\n\
         💻 <a href=\"https://github.com/ankit-chaubey/tgbotrs\">github.com/ankit-chaubey/tgbotrs</a>\n\
         📖 <a href=\"https://docs.rs/tgbotrs\">docs.rs/tgbotrs</a>\n\
         🌐 <a href=\"https://core.telegram.org/bots/api\">Telegram Bot API</a>",
        Some(SendMessageParams::new()
            .parse_mode("HTML")
            .reply_markup(kb(vec![vec![
                url_btn("📦 crates.io", "https://crates.io/crates/tgbotrs"),
                url_btn("💻 GitHub",    "https://github.com/ankit-chaubey/tgbotrs"),
                url_btn("📖 docs.rs",   "https://docs.rs/tgbotrs"),
            ]]))
        ),
    ).await;
}

// ─── /source ─────────────────────────────────────────────────────────────────

pub async fn cmd_source(bot: &Bot, msg: &Message) {
    let _ = bot.send_message(
        msg.chat.id,
        "🔗 <b>Library Source Links</b>\n\n\
         <b>tgbotrs</b> — Rust Telegram Bot API\n\
         📦 <a href=\"https://crates.io/crates/tgbotrs\">crates.io/crates/tgbotrs</a>\n\
         💻 <a href=\"https://github.com/ankit-chaubey/tgbotrs\">github.com/ankit-chaubey/tgbotrs</a>\n\
         📖 <a href=\"https://docs.rs/tgbotrs\">docs.rs/tgbotrs</a>\n\
         🌐 <a href=\"https://core.telegram.org/bots/api\">Telegram Bot API spec</a>\n\n\
         <code>tgbotrs = { version = \"0.1.4\" }</code>",
        Some(SendMessageParams::new()
            .parse_mode("HTML")
            .reply_markup(kb(vec![vec![
                url_btn("📦 crates.io", "https://crates.io/crates/tgbotrs"),
                url_btn("💻 GitHub",    "https://github.com/ankit-chaubey/tgbotrs"),
            ]]))
        ),
    ).await;
}

// ─── /ping ────────────────────────────────────────────────────────────────────

pub async fn cmd_ping(bot: &Bot, msg: &Message) {
    let start = std::time::Instant::now();
    let sent  = bot.send_message(msg.chat.id, "🏓 Pinging...", None).await;
    let ms    = start.elapsed().as_millis();
    if let Ok(sent_msg) = sent {
        let params = EditMessageTextParams::new()
            .chat_id(msg.chat.id)
            .message_id(sent_msg.message_id)
            .parse_mode("Markdown");
        let _ = bot.edit_message_text(format!("🏓 Pong! `{ms}ms`"), Some(params)).await;
    }
}

// ─── /userinfo ────────────────────────────────────────────────────────────────

pub async fn cmd_userinfo(bot: &Bot, msg: &Message) {
    let target = msg.reply_to_message.as_deref().unwrap_or(msg);
    let user   = match target.from.as_deref() {
        Some(u) => u,
        None => {
            let _ = bot.send_message(msg.chat.id, "❌ Could not find user info.", None).await;
            return;
        }
    };
    let name     = format!("{}{}", user.first_name,
        user.last_name.as_deref().map(|l| format!(" {l}")).unwrap_or_default());
    let username = user.username.as_deref()
        .map(|u| format!("@{u}"))
        .unwrap_or_else(|| "none".into());
    let is_bot   = if user.is_bot { "✅ Yes" } else { "❌ No" };
    let premium  = user.is_premium
        .map(|p| if p { "✅ Yes" } else { "❌ No" })
        .unwrap_or("❌ No");
    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "👤 <b>User Info</b>\n\n\
             🔹 <b>Name:</b> {name}\n\
             🔹 <b>Username:</b> {username}\n\
             🔹 <b>User ID:</b> <code>{}</code>\n\
             🔹 <b>Bot:</b> {is_bot}\n\
             🔹 <b>Premium:</b> {premium}",
            user.id
        ),
        Some(SendMessageParams::new().parse_mode("HTML")),
    ).await;
}

// ─── /chatinfo ────────────────────────────────────────────────────────────────

pub async fn cmd_chatinfo(bot: &Bot, msg: &Message) {
    let chat  = &msg.chat;
    let title = chat.title.as_deref()
        .or(chat.first_name.as_deref())
        .unwrap_or("Unknown");
    let username = chat.username.as_deref()
        .map(|u| format!("@{u}"))
        .unwrap_or_else(|| "none".into());
    let members = bot.get_chat_member_count(chat.id).await
        .map(|n| n.to_string())
        .unwrap_or_else(|_| "N/A".into());
    let is_forum = chat.is_forum.unwrap_or(false);
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

// ─── /members ────────────────────────────────────────────────────────────────

pub async fn cmd_members(bot: &Bot, msg: &Message) {
    match bot.get_chat_member_count(msg.chat.id).await {
        Ok(n)  => {
            let _ = bot.send_message(
                msg.chat.id,
                format!("👥 This chat has *{n}* members."),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => {
            let _ = bot.send_message(msg.chat.id, format!("❌ Could not get member count: {e}"), None).await;
        }
    }
}
