use tgbotrs::{Bot, InputPollOption, Message};
use tgbotrs::gen_methods::SendPollParams;

pub async fn cmd_poll(bot: &Bot, msg: &Message, args: &str) {
    let chat_id = msg.chat.id;
    let parts: Vec<&str> = args.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if parts.len() < 3 {
        let _ = bot.send_message(
            chat_id,
            "Usage: <code>/poll &lt;question&gt; | &lt;option1&gt; | &lt;option2&gt; ...</code>\n\
             Example:\n<code>/poll Best language? | Rust | Python | Go | C++</code>",
            Some(tgbotrs::gen_methods::SendMessageParams::new().parse_mode("HTML")),
        ).await;
        return;
    }
    let question = parts[0];
    let options: Vec<InputPollOption> = parts[1..].iter()
        .take(10)
        .map(|o| InputPollOption { text: o.to_string(), text_parse_mode: None, text_entities: None })
        .collect();
    if let Err(e) = bot.send_poll(chat_id, question, options, None).await {
        let _ = bot.send_message(chat_id, format!("❌ Failed to create poll: {e}"), None).await;
    }
}

pub async fn cmd_quiz(bot: &Bot, msg: &Message, args: &str) {
    let chat_id = msg.chat.id;
    let parts: Vec<&str> = args.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if parts.len() < 3 {
        let _ = bot.send_message(
            chat_id,
            "Usage: <code>/quiz &lt;question&gt; | &lt;correct answer&gt; | &lt;wrong1&gt; | &lt;wrong2&gt; ...</code>\n\
             The <b>first option after the question</b> is the correct answer.\n\
             Example:\n<code>/quiz Rust was created by? | Mozilla | Google | Microsoft | Apple</code>",
            Some(tgbotrs::gen_methods::SendMessageParams::new().parse_mode("HTML")),
        ).await;
        return;
    }
    let question = parts[0];
    let options: Vec<InputPollOption> = parts[1..].iter()
        .take(10)
        .map(|o| InputPollOption { text: o.to_string(), text_parse_mode: None, text_entities: None })
        .collect();
    let params = SendPollParams::new()
        .r#type("quiz".to_string())
        .correct_option_id(0i64)
        .is_anonymous(false);
    if let Err(e) = bot.send_poll(chat_id, question, options, Some(params)).await {
        let _ = bot.send_message(chat_id, format!("❌ Failed to create quiz: {e}"), None).await;
    }
}
