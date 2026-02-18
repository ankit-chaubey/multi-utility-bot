use tgbotrs::{Bot, MaybeInaccessibleMessage, Update};
use tgbotrs::gen_methods::{AnswerCallbackQueryParams, EditMessageTextParams};
use crate::cmd::{admin, fun, games, info, notes, polls, util};
use crate::state::SharedState;

/// Strip /command@botname → ("/command", "args")
fn parse_cmd(text: &str) -> (&str, &str) {
    let text = text.trim();
    if !text.starts_with('/') { return ("", text); }
    let (cmd_part, args) = match text.find(' ') {
        Some(i) => (&text[..i], text[i + 1..].trim()),
        None    => (text, ""),
    };
    let cmd = cmd_part.find('@').map(|i| &cmd_part[..i]).unwrap_or(cmd_part);
    (cmd, args)
}

pub async fn handle_update(bot: Bot, update: Update, state: SharedState) {
    // ── 1. Text messages ──────────────────────────────────────────────────────
    if let Some(msg) = update.message {
        if let Some(text) = msg.text.clone() {
            let (cmd, args) = parse_cmd(&text);
            match cmd {
                // ── Info / general
                "/start"    => info::cmd_start(&bot, &msg).await,
                "/help"     => info::cmd_help(&bot, &msg, "main").await,
                "/about"    => info::cmd_about(&bot, &msg).await,
                "/ping"     => info::cmd_ping(&bot, &msg).await,
                "/source"   => info::cmd_source(&bot, &msg).await,
                "/id"       => util::cmd_id(&bot, &msg).await,
                "/userinfo" => info::cmd_userinfo(&bot, &msg).await,
                "/chatinfo" => info::cmd_chatinfo(&bot, &msg).await,
                "/members"  => info::cmd_members(&bot, &msg).await,

                // ── Fun
                "/dice"       => fun::cmd_dice(&bot, &msg).await,
                "/roll"       => fun::cmd_roll(&bot, &msg, args).await,
                "/flip"       => fun::cmd_flip(&bot, &msg).await,
                "/joke"       => fun::cmd_joke(&bot, &msg).await,
                "/quote"      => fun::cmd_quote(&bot, &msg).await,
                "/fact"       => fun::cmd_fact(&bot, &msg).await,
                "/8ball"      => fun::cmd_8ball(&bot, &msg, args).await,
                "/rps"        => fun::cmd_rps(&bot, &msg).await,
                "/choose"     => fun::cmd_choose(&bot, &msg, args).await,
                "/rate"       => fun::cmd_rate(&bot, &msg, args).await,
                "/password"   => fun::cmd_password(&bot, &msg, args).await,
                "/mock"       => fun::cmd_mock(&bot, &msg, args).await,
                "/clap"       => fun::cmd_clap(&bot, &msg, args).await,
                "/shrug"      => fun::cmd_shrug(&bot, &msg).await,
                "/tableflip"  => fun::cmd_tableflip(&bot, &msg).await,
                "/unflip"     => fun::cmd_unflip(&bot, &msg).await,

                // ── Utility
                "/echo"    => util::cmd_echo(&bot, &msg, args).await,
                "/reverse" => util::cmd_reverse(&bot, &msg, args).await,
                "/upper"   => util::cmd_upper(&bot, &msg, args).await,
                "/lower"   => util::cmd_lower(&bot, &msg, args).await,
                "/count"   => util::cmd_count(&bot, &msg, args).await,
                "/calc"    => util::cmd_calc(&bot, &msg, args).await,
                "/b64"     => util::cmd_b64(&bot, &msg, args).await,
                "/repeat"  => util::cmd_repeat(&bot, &msg, args).await,
                "/ascii"   => util::cmd_ascii(&bot, &msg, args).await,
                "/binary"  => util::cmd_binary(&bot, &msg, args).await,
                "/time"    => util::cmd_time(&bot, &msg).await,

                // ── Admin
                "/ban"        => admin::cmd_ban(&bot, &msg).await,
                "/kick"       => admin::cmd_kick(&bot, &msg).await,
                "/mute"       => admin::cmd_mute(&bot, &msg).await,
                "/unmute"     => admin::cmd_unmute(&bot, &msg).await,
                "/warn"       => admin::cmd_warn(&bot, &msg, args, &state).await,
                "/warns"      => admin::cmd_warns(&bot, &msg, &state).await,
                "/clearwarns" => admin::cmd_clearwarns(&bot, &msg, &state).await,
                "/pin"        => admin::cmd_pin(&bot, &msg).await,
                "/unpin"      => admin::cmd_unpin(&bot, &msg).await,
                "/del"        => admin::cmd_del(&bot, &msg).await,
                "/promote"    => admin::cmd_promote(&bot, &msg).await,
                "/demote"     => admin::cmd_demote(&bot, &msg).await,
                "/invite"     => admin::cmd_invite(&bot, &msg).await,

                // ── Games
                "/guess"  => games::cmd_guess(&bot, &msg, args, &state).await,
                "/giveup" => games::cmd_giveup(&bot, &msg, &state).await,

                // ── Notes
                "/save"    => notes::cmd_save(&bot, &msg, args, &state).await,
                "/get"     => notes::cmd_get(&bot, &msg, args, &state).await,
                "/notes"   => notes::cmd_notes(&bot, &msg, &state).await,
                "/delnote" => notes::cmd_delnote(&bot, &msg, args, &state).await,

                // ── Polls
                "/poll" => polls::cmd_poll(&bot, &msg, args).await,
                "/quiz" => polls::cmd_quiz(&bot, &msg, args).await,

                _ => {}
            }
        }
    }

    // ── 2. Callback queries (inline buttons) ──────────────────────────────────
    if let Some(cbq) = update.callback_query {
        let query_id = cbq.id.clone();
        let data     = cbq.data.as_deref().unwrap_or("").to_string();

        let _ = bot.answer_callback_query(
            &query_id,
            Some(AnswerCallbackQueryParams::new()),
        ).await;

        if let Some(maybe_msg) = cbq.message {
            if let MaybeInaccessibleMessage::Message(m) = *maybe_msg {
                let chat_id = m.chat.id;
                let msg_id  = m.message_id;

                // Help section navigation
                if let Some(section) = data.strip_prefix("help:") {
                    info::send_help_section(&bot, chat_id, msg_id, section).await;
                    return;
                }

                // Rock-Paper-Scissors result
                if let Some(choice) = data.strip_prefix("rps:") {
                    let result = fun::rps_result(choice);
                    let params = EditMessageTextParams::new()
                        .chat_id(chat_id)
                        .message_id(msg_id)
                        .parse_mode("Markdown");
                    let _ = bot.edit_message_text(
                        format!("✊✌️🖐 *Rock Paper Scissors*\n\n{result}"),
                        Some(params),
                    ).await;
                    return;
                }
            }
        }
    }
}
