use tgbotrs::{Bot, Message, InputPollOption};
use tgbotrs::gen_methods::SendPollParams;

pub async fn cmd_poll(bot: &Bot, msg: &Message, args: &str) {
    let chat_id = msg.chat.id;
    let parts: Vec<&str> = args.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if parts.len() < 3 {
        let _ = bot.send_message(
            chat_id,
            "Usage: /poll <question> | <option1> | <option2> ...\nExample:\n/poll Favourite language? | Rust | Python | Go | C++",
            None,
        ).await;
        return;
    }
    let question = parts[0];
    let options: Vec<InputPollOption> = parts[1..].iter()
        .map(|o| InputPollOption { text: o.to_string(), text_parse_mode: None, text_entities: None })
        .collect();
    if options.len() > 10 {
        let _ = bot.send_message(chat_id, "❌ Maximum 10 options allowed.", None).await;
        return;
    }
    match bot.send_poll(chat_id, question, options, None).await {
        Ok(_) => {}
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to create poll: {e}"), None).await; }
    }
}

pub async fn cmd_quiz(bot: &Bot, msg: &Message, args: &str) {
    let chat_id = msg.chat.id;
    let parts: Vec<&str> = args.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if parts.len() < 3 {
        let _ = bot.send_message(
            chat_id,
            "Usage: /quiz <question> | <correct answer> | <wrong1> | <wrong2> ...\nThe FIRST option after the question is the correct answer!\nExample:\n/quiz Rust was created by? | Mozilla | Google | Microsoft",
            None,
        ).await;
        return;
    }
    let question = parts[0];
    // Shuffle options but remember correct_option_id = 0 (first one), so correct stays at 0
    // For simplicity, correct answer is always parts[1] = index 0
    let options: Vec<InputPollOption> = parts[1..].iter()
        .map(|o| InputPollOption { text: o.to_string(), text_parse_mode: None, text_entities: None })
        .collect();
    if options.len() > 10 {
        let _ = bot.send_message(chat_id, "❌ Maximum 10 options allowed.", None).await;
        return;
    }
    let params = SendPollParams::new()
        .r#type("quiz".to_string())
        .correct_option_id(0i64)
        .is_anonymous(false);
    match bot.send_poll(chat_id, question, options, Some(params)).await {
        Ok(_) => {}
        Err(e) => { let _ = bot.send_message(chat_id, format!("❌ Failed to create quiz: {e}"), None).await; }
    }
}
