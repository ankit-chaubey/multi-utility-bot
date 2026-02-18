use tgbotrs::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

/// Make a single callback button
pub fn btn(text: &str, data: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.to_string(),
        callback_data: Some(data.to_string()),
        ..Default::default()
    }
}

/// Make a URL button
pub fn url_btn(text: &str, url: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.to_string(),
        url: Some(url.to_string()),
        ..Default::default()
    }
}

/// Build an inline keyboard from rows of buttons
pub fn kb(rows: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
    ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: rows })
}

/// One-liner for a single row of buttons
pub fn row(buttons: Vec<InlineKeyboardButton>) -> ReplyMarkup {
    kb(vec![buttons])
}
