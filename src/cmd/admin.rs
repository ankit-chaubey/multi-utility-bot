use tgbotrs::{Bot, Message, ChatMember, ChatPermissions};
use tgbotrs::gen_methods::{
    UnbanChatMemberParams,
    PromoteChatMemberParams, PinChatMessageParams, SendMessageParams,
};
use crate::state::SharedState;

// ── Helpers ───────────────────────────────────────────────────────────────────

async fn is_admin(bot: &Bot, chat_id: i64, user_id: i64) -> bool {
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(ChatMember::ChatMemberOwner(_)) | Ok(ChatMember::ChatMemberAdministrator(_)) => true,
        _ => false,
    }
}

/// Returns (chat_id, sender_id, target_user) or sends error and returns None
async fn require_reply_target(bot: &Bot, msg: &Message) -> Option<(i64, i64, Box<tgbotrs::User>)> {
    let sender_id = msg.from.as_ref()?.id;
    let chat_id = msg.chat.id;
    let reply = match msg.reply_to_message.as_deref() {
        Some(r) => r,
        None => {
            let _ = bot.send_message(chat_id, "❌ Reply to a user's message to use this command.", None).await;
            return None;
        }
    };
    let target = match reply.from.as_deref() {
        Some(u) => Box::new(u.clone()),
        None => {
            let _ = bot.send_message(chat_id, "❌ Could not identify the target user.", None).await;
            return None;
        }
    };
    Some((chat_id, sender_id, target))
}

async fn require_admin(bot: &Bot, msg: &Message) -> Option<(i64, i64, Box<tgbotrs::User>)> {
    let (chat_id, sender_id, target) = require_reply_target(bot, msg).await?;
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to use this command.", None).await;
        return None;
    }
    if is_admin(bot, chat_id, target.id).await {
        let _ = bot.send_message(chat_id, "❌ You can't use admin commands on another admin.", None).await;
        return None;
    }
    Some((chat_id, sender_id, target))
}

// ── Commands ──────────────────────────────────────────────────────────────────

pub async fn cmd_ban(bot: &Bot, msg: &Message) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    match bot.ban_chat_member(chat_id, target.id, None).await {
        Ok(_) => {
            let name = &target.first_name;
            let _ = bot.send_message(
                chat_id,
                format!("🔨 *{name}* has been banned.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to ban: {e}"), None).await; }
    }
}

pub async fn cmd_kick(bot: &Bot, msg: &Message) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    let name = target.first_name.clone();
    // Ban then immediately unban = kick
    if bot.ban_chat_member(chat_id, target.id, None).await.is_ok() {
        let _ = bot.unban_chat_member(chat_id, target.id, Some(UnbanChatMemberParams::new().only_if_banned(true))).await;
        let _ = bot.send_message(
            chat_id,
            format!("👢 *{name}* has been kicked.", ),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
    } else {
        let _ = bot.send_message(chat_id, "❌ Failed to kick user.", None).await;
    }
}

pub async fn cmd_mute(bot: &Bot, msg: &Message) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    let no_perms = ChatPermissions {
        can_send_messages: Some(false),
        can_send_audios: Some(false),
        can_send_documents: Some(false),
        can_send_photos: Some(false),
        can_send_videos: Some(false),
        can_send_video_notes: Some(false),
        can_send_voice_notes: Some(false),
        can_send_polls: Some(false),
        can_send_other_messages: Some(false),
        can_add_web_page_previews: Some(false),
        can_change_info: None,
        can_invite_users: None,
        can_pin_messages: None,
        can_manage_topics: None,
    };
    match bot.restrict_chat_member(chat_id, target.id, no_perms, None).await {
        Ok(_) => {
            let name = &target.first_name;
            let _ = bot.send_message(
                chat_id,
                format!("🔇 *{name}* has been muted.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to mute: {e}"), None).await; }
    }
}

pub async fn cmd_unmute(bot: &Bot, msg: &Message) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    let all_perms = ChatPermissions {
        can_send_messages: Some(true),
        can_send_audios: Some(true),
        can_send_documents: Some(true),
        can_send_photos: Some(true),
        can_send_videos: Some(true),
        can_send_video_notes: Some(true),
        can_send_voice_notes: Some(true),
        can_send_polls: Some(true),
        can_send_other_messages: Some(true),
        can_add_web_page_previews: Some(true),
        can_change_info: None,
        can_invite_users: None,
        can_pin_messages: None,
        can_manage_topics: None,
    };
    match bot.restrict_chat_member(chat_id, target.id, all_perms, None).await {
        Ok(_) => {
            let name = &target.first_name;
            let _ = bot.send_message(
                chat_id,
                format!("🔊 *{name}* has been unmuted.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to unmute: {e}"), None).await; }
    }
}

pub async fn cmd_warn(bot: &Bot, msg: &Message, reason: &str, state: &SharedState) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    let name = target.first_name.clone();
    let key = (chat_id, target.id);
    let (count, max) = {
        let mut st = state.lock().await;
        let warns = st.warnings.entry(key).or_default();
        warns.push(if reason.trim().is_empty() { "No reason given".to_string() } else { reason.trim().to_string() });
        (warns.len(), 3usize)
    };
    if count >= max {
        // Auto-ban at max warns
        let _ = bot.ban_chat_member(chat_id, target.id, None).await;
        let _ = bot.send_message(
            chat_id,
            format!("⚠️ *{name}* has been warned {count}/{max} times and has been *banned*!", ),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
        state.lock().await.warnings.remove(&key);
    } else {
        let _ = bot.send_message(
            chat_id,
            format!("⚠️ *{name}* has been warned! ({count}/{max})\nReason: {}", if reason.trim().is_empty() { "No reason" } else { reason.trim() }),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
    }
}

pub async fn cmd_warns(bot: &Bot, msg: &Message, state: &SharedState) {
    let Some((chat_id, _, target)) = require_reply_target(bot, msg).await else { return };
    let name = &target.first_name;
    let key = (chat_id, target.id);
    let st = state.lock().await;
    let warns = st.warnings.get(&key);
    match warns {
        None => {
            let _ = bot.send_message(chat_id, format!("✅ *{name}* has no warnings."), Some(SendMessageParams::new().parse_mode("Markdown"))).await;
        }
        Some(w) if w.is_empty() => {
            let _ = bot.send_message(chat_id, format!("✅ *{name}* has no warnings."), Some(SendMessageParams::new().parse_mode("Markdown"))).await;
        }
        Some(w) => {
            let list = w.iter().enumerate()
                .map(|(i, r)| format!("{}. {r}", i + 1))
                .collect::<Vec<_>>().join("\n");
            let _ = bot.send_message(
                chat_id,
                format!("⚠️ *{name}* has {}/{} warnings:\n{list}", w.len(), 3),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
    }
}

pub async fn cmd_clearwarns(bot: &Bot, msg: &Message, state: &SharedState) {
    let Some((chat_id, _, target)) = require_admin(bot, msg).await else { return };
    let name = &target.first_name;
    let key = (chat_id, target.id);
    state.lock().await.warnings.remove(&key);
    let _ = bot.send_message(
        chat_id,
        format!("✅ Warnings cleared for *{name}*.", ),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_pin(bot: &Bot, msg: &Message) {
    let sender_id = match msg.from.as_ref() { Some(u) => u.id, None => return };
    let chat_id = msg.chat.id;
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to pin messages.", None).await;
        return;
    }
    let reply_id = match msg.reply_to_message.as_ref() {
        Some(r) => r.message_id,
        None => { let _ = bot.send_message(chat_id, "❌ Reply to the message you want to pin.", None).await; return; }
    };
    match bot.pin_chat_message(chat_id, reply_id, Some(PinChatMessageParams::new().disable_notification(false))).await {
        Ok(_) => { let _ = bot.send_message(chat_id, "📌 Message pinned!", None).await; }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to pin: {e}"), None).await; }
    }
}

pub async fn cmd_unpin(bot: &Bot, msg: &Message) {
    let sender_id = match msg.from.as_ref() { Some(u) => u.id, None => return };
    let chat_id = msg.chat.id;
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to unpin messages.", None).await;
        return;
    }
    match bot.unpin_chat_message(chat_id, None).await {
        Ok(_) => { let _ = bot.send_message(chat_id, "📌 Message unpinned!", None).await; }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to unpin: {e}"), None).await; }
    }
}

pub async fn cmd_del(bot: &Bot, msg: &Message) {
    let sender_id = match msg.from.as_ref() { Some(u) => u.id, None => return };
    let chat_id = msg.chat.id;
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to delete messages.", None).await;
        return;
    }
    let reply_id = match msg.reply_to_message.as_ref() {
        Some(r) => r.message_id,
        None => { let _ = bot.send_message(chat_id, "❌ Reply to the message you want to delete.", None).await; return; }
    };
    // Delete the command message too
    let _ = bot.delete_message(chat_id, msg.message_id).await;
    let _ = bot.delete_message(chat_id, reply_id).await;
}

pub async fn cmd_promote(bot: &Bot, msg: &Message) {
    let Some((chat_id, sender_id, target)) = require_reply_target(bot, msg).await else { return };
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to promote users.", None).await;
        return;
    }
    let name = &target.first_name;
    let params = PromoteChatMemberParams::new()
        .can_manage_chat(true)
        .can_delete_messages(true)
        .can_restrict_members(true)
        .can_invite_users(true)
        .can_pin_messages(true)
        .can_manage_video_chats(true);
    match bot.promote_chat_member(chat_id, target.id, Some(params)).await {
        Ok(_) => {
            let _ = bot.send_message(
                chat_id,
                format!("⬆️ *{name}* has been promoted to admin.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to promote: {e}"), None).await; }
    }
}

pub async fn cmd_demote(bot: &Bot, msg: &Message) {
    let Some((chat_id, sender_id, target)) = require_reply_target(bot, msg).await else { return };
    if !is_admin(bot, chat_id, sender_id).await {
        let _ = bot.send_message(chat_id, "❌ You need to be an admin to demote users.", None).await;
        return;
    }
    let name = &target.first_name;
    // Demote by setting all privileges to false
    let params = PromoteChatMemberParams::new()
        .can_manage_chat(false)
        .can_delete_messages(false)
        .can_restrict_members(false)
        .can_invite_users(false)
        .can_pin_messages(false)
        .can_manage_video_chats(false);
    match bot.promote_chat_member(chat_id, target.id, Some(params)).await {
        Ok(_) => {
            let _ = bot.send_message(
                chat_id,
                format!("⬇️ *{name}* has been demoted.", ),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to demote: {e}"), None).await; }
    }
}
