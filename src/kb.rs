use tgbotrs::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

/// Callback button
pub fn btn(text: &str, data: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.to_string(),
        callback_data: Some(data.to_string()),
        ..Default::default()
    }
}

/// URL button
pub fn url_btn(text: &str, url: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.to_string(),
        url: Some(url.to_string()),
        ..Default::default()
    }
}

/// Multi-row inline keyboard
pub fn kb(rows: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
    ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })
}

/// Single-row inline keyboard
pub fn row(buttons: Vec<InlineKeyboardButton>) -> ReplyMarkup {
    kb(vec![buttons])
}
