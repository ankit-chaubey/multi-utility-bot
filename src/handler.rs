use tgbotrs::{Bot, MaybeInaccessibleMessage, Update};
use crate::cmd::{admin, fun, games, info, notes, polls, util};
use crate::state::SharedState;

/// Strip the bot-username suffix from commands like /ban@mybot → /ban
fn parse_command(text: &str) -> (&str, &str) {
    let text = text.trim();
    if !text.starts_with('/') {
        return ("", text);
    }
    // /command[@botname] [args...]
    let (cmd_part, args) = match text.find(' ') {
        Some(i) => (&text[..i], text[i+1..].trim()),
        None    => (text, ""),
    };
    // Strip @botname
    let cmd = match cmd_part.find('@') {
        Some(i) => &cmd_part[..i],
        None    => cmd_part,
    };
    (cmd, args)
}

pub async fn handle_update(bot: Bot, update: Update, state: SharedState) {
    // ── Text messages ─────────────────────────────────────────────────────────
    if let Some(msg) = update.message {
        if let Some(ref text) = msg.text.clone() {
            let (cmd, args) = parse_command(text);
            match cmd {
                // Info / general
                "/start"    => info::cmd_start(&bot, &msg).await,
                "/help"     => info::cmd_help(&bot, &msg, "main").await,
                "/about"    => info::cmd_about(&bot, &msg).await,
                "/ping"     => info::cmd_ping(&bot, &msg).await,
                "/id"       => util::cmd_id(&bot, &msg).await,
                "/userinfo" => info::cmd_userinfo(&bot, &msg).await,
                "/chatinfo" => info::cmd_chatinfo(&bot, &msg).await,
                "/members"  => info::cmd_members(&bot, &msg).await,

                // Fun
                "/dice"  => fun::cmd_dice(&bot, &msg).await,
                "/roll"  => fun::cmd_roll(&bot, &msg, args).await,
                "/flip"  => fun::cmd_flip(&bot, &msg).await,
                "/joke"  => fun::cmd_joke(&bot, &msg).await,
                "/quote" => fun::cmd_quote(&bot, &msg).await,
                "/fact"  => fun::cmd_fact(&bot, &msg).await,
                "/8ball" => fun::cmd_8ball(&bot, &msg, args).await,
                "/rps"   => fun::cmd_rps(&bot, &msg).await,

                // Utility
                "/echo"    => util::cmd_echo(&bot, &msg, args).await,
                "/reverse" => util::cmd_reverse(&bot, &msg, args).await,
                "/upper"   => util::cmd_upper(&bot, &msg, args).await,
                "/lower"   => util::cmd_lower(&bot, &msg, args).await,
                "/count"   => util::cmd_count(&bot, &msg, args).await,
                "/calc"    => util::cmd_calc(&bot, &msg, args).await,
                "/b64"     => util::cmd_b64(&bot, &msg, args).await,
                "/repeat"  => util::cmd_repeat(&bot, &msg, args).await,
                "/time"    => util::cmd_time(&bot, &msg).await,

                // Admin
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

                // Games
                "/guess"  => games::cmd_guess(&bot, &msg, args, &state).await,
                "/giveup" => games::cmd_giveup(&bot, &msg, &state).await,

                // Notes
                "/save"    => notes::cmd_save(&bot, &msg, args, &state).await,
                "/get"     => notes::cmd_get(&bot, &msg, args, &state).await,
                "/notes"   => notes::cmd_notes(&bot, &msg, &state).await,
                "/delnote" => notes::cmd_delnote(&bot, &msg, args, &state).await,

                // Polls
                "/poll" => polls::cmd_poll(&bot, &msg, args).await,
                "/quiz" => polls::cmd_quiz(&bot, &msg, args).await,

                _ => {}
            }
        }
    }

    // ── Callback queries (inline buttons) ─────────────────────────────────────
    if let Some(cbq) = update.callback_query {
        let query_id = cbq.id.clone();
        let data = cbq.data.as_deref().unwrap_or("").to_string();

        // Always ack
        let _ = bot.answer_callback_query(
            &query_id,
            Some(tgbotrs::gen_methods::AnswerCallbackQueryParams::new()),
        ).await;

        if let Some(maybe_msg) = cbq.message {
            if let MaybeInaccessibleMessage::Message(cbq_msg) = *maybe_msg {
                let chat_id  = cbq_msg.chat.id;
                let msg_id   = cbq_msg.message_id;

                // Help navigation
                if let Some(section) = data.strip_prefix("help:") {
                    info::send_help_section(&bot, chat_id, msg_id, section).await;
                    return;
                }

                // RPS result
                if let Some(choice) = data.strip_prefix("rps:") {
                    let result = fun::rps_result(choice);
                    use tgbotrs::gen_methods::EditMessageTextParams;
                    let params = EditMessageTextParams::new()
                        .chat_id(chat_id)
                        .message_id(msg_id);
                    let _ = bot.edit_message_text(
                        format!("✊✌️🖐 *Rock Paper Scissors*\n\n{result}"),
                        Some(params.parse_mode("Markdown")),
                    ).await;
                    return;
                }
            }
        }
    }
}
