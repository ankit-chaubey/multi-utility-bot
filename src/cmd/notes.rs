use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::SendMessageParams;
use crate::state::SharedState;

pub async fn cmd_save(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    match parts.as_slice() {
        [name, content] if !name.trim().is_empty() && !content.trim().is_empty() => {
            let key = (chat_id, name.trim().to_lowercase());
            state.lock().await.notes.insert(key, content.trim().to_string());
            let _ = bot.send_message(
                chat_id,
                format!("📝 Note *{}* saved!", name.trim()),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        _ => {
            let _ = bot.send_message(chat_id, "Usage: /save <name> <content>", None).await;
        }
    }
}

pub async fn cmd_get(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let name = args.trim().to_lowercase();
    if name.is_empty() {
        let _ = bot.send_message(chat_id, "Usage: /get <name>", None).await;
        return;
    }
    let st = state.lock().await;
    match st.notes.get(&(chat_id, name.clone())) {
        Some(content) => {
            let _ = bot.send_message(
                chat_id,
                format!("📝 *{name}:*\n\n{content}"),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        None => {
            let _ = bot.send_message(chat_id, format!("❌ No note found with name `{name}`."), Some(SendMessageParams::new().parse_mode("Markdown"))).await;
        }
    }
}

pub async fn cmd_notes(bot: &Bot, msg: &Message, state: &SharedState) {
    let chat_id = msg.chat.id;
    let st = state.lock().await;
    let notes: Vec<String> = st.notes.keys()
        .filter(|(cid, _)| *cid == chat_id)
        .map(|(_, name)| format!("• `{name}`"))
        .collect();
    if notes.is_empty() {
        let _ = bot.send_message(chat_id, "📝 No notes saved in this chat. Use /save <name> <content> to add one.", None).await;
    } else {
        let list = notes.join("\n");
        let _ = bot.send_message(
            chat_id,
            format!("📝 *Notes in this chat:*\n\n{list}\n\nUse /get <name> to retrieve one."),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
    }
}

pub async fn cmd_delnote(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let name = args.trim().to_lowercase();
    if name.is_empty() {
        let _ = bot.send_message(chat_id, "Usage: /delnote <name>", None).await;
        return;
    }
    let removed = state.lock().await.notes.remove(&(chat_id, name.clone()));
    if removed.is_some() {
        let _ = bot.send_message(
            chat_id,
            format!("🗑️ Note `{name}` deleted."),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
    } else {
        let _ = bot.send_message(chat_id, format!("❌ No note found with name `{name}`."), Some(SendMessageParams::new().parse_mode("Markdown"))).await;
    }
}
