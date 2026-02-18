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
            let _ = bot.send_message(
                chat_id,
                "Usage: `/save <name> <content>`\nExample: `/save rules No spam!`",
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
    }
}

pub async fn cmd_get(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let name    = args.trim().to_lowercase();
    if name.is_empty() {
        let _ = bot.send_message(chat_id, "Usage: `/get <name>`", Some(SendMessageParams::new().parse_mode("Markdown"))).await;
        return;
    }
    let st = state.lock().await;
    match st.notes.get(&(chat_id, name.clone())) {
        Some(content) => {
            let _ = bot.send_message(
                chat_id,
                format!("📝 <b>{name}:</b>\n\n{content}"),
                Some(SendMessageParams::new().parse_mode("HTML")),
            ).await;
        }
        None => {
            let _ = bot.send_message(
                chat_id,
                format!("❌ No note found with name `{name}`.\nUse /notes to list all notes."),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
    }
}

pub async fn cmd_notes(bot: &Bot, msg: &Message, state: &SharedState) {
    let chat_id = msg.chat.id;
    let st      = state.lock().await;
    let mut notes: Vec<String> = st.notes.keys()
        .filter(|(cid, _)| *cid == chat_id)
        .map(|(_, name)| format!("• <code>{name}</code>"))
        .collect();
    notes.sort();
    if notes.is_empty() {
        let _ = bot.send_message(
            chat_id,
            "📝 No notes saved in this chat.\nUse <code>/save &lt;name&gt; &lt;content&gt;</code> to add one.",
            Some(SendMessageParams::new().parse_mode("HTML")),
        ).await;
    } else {
        let list = notes.join("\n");
        let _ = bot.send_message(
            chat_id,
            format!("📝 <b>Notes in this chat:</b>\n\n{list}\n\nUse <code>/get &lt;name&gt;</code> to retrieve one."),
            Some(SendMessageParams::new().parse_mode("HTML")),
        ).await;
    }
}

pub async fn cmd_delnote(bot: &Bot, msg: &Message, args: &str, state: &SharedState) {
    let chat_id = msg.chat.id;
    let name    = args.trim().to_lowercase();
    if name.is_empty() {
        let _ = bot.send_message(chat_id, "Usage: `/delnote <name>`", Some(SendMessageParams::new().parse_mode("Markdown"))).await;
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
        let _ = bot.send_message(
            chat_id,
            format!("❌ No note found with name `{name}`."),
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
    }
}
