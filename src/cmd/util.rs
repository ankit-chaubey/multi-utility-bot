use chrono::Utc;
use tgbotrs::{Bot, Message};
use tgbotrs::gen_methods::SendMessageParams;

pub async fn cmd_echo(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /echo <text>", None).await;
        return;
    }
    let _ = bot.send_message(msg.chat.id, args, None).await;
}

pub async fn cmd_reverse(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /reverse <text>", None).await;
        return;
    }
    let reversed: String = args.chars().rev().collect();
    let _ = bot.send_message(msg.chat.id, reversed, None).await;
}

pub async fn cmd_upper(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /upper <text>", None).await;
        return;
    }
    let _ = bot.send_message(msg.chat.id, args.to_uppercase(), None).await;
}

pub async fn cmd_lower(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /lower <text>", None).await;
        return;
    }
    let _ = bot.send_message(msg.chat.id, args.to_lowercase(), None).await;
}

pub async fn cmd_count(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /count <text>", None).await;
        return;
    }
    let chars = args.chars().count();
    let words = args.split_whitespace().count();
    let lines = args.lines().count();
    let _ = bot.send_message(
        msg.chat.id,
        format!("📊 *Text stats:*\n• Characters: `{chars}`\n• Words: `{words}`\n• Lines: `{lines}`"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_time(bot: &Bot, msg: &Message) {
    let now = Utc::now();
    let _ = bot.send_message(
        msg.chat.id,
        format!(
            "🕐 *Current UTC Time*\n\n`{}`\n\nUnix timestamp: `{}`",
            now.format("%Y-%m-%d %H:%M:%S UTC"),
            now.timestamp()
        ),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_id(bot: &Bot, msg: &Message) {
    let user_id = msg.from.as_ref().map(|u| u.id).unwrap_or(0);
    let username = msg.from.as_ref().and_then(|u| u.username.as_deref()).unwrap_or("none");
    let chat_id = msg.chat.id;
    let chat_type = &msg.chat.r#type;

    let reply = if let Some(reply) = &msg.reply_to_message {
        let rid = reply.from.as_ref().map(|u| u.id).unwrap_or(0);
        let rname = reply.from.as_ref().and_then(|u| u.username.as_deref()).unwrap_or("none");
        format!(
            "🆔 *Your ID:* `{user_id}` (@{username})\n\
             🆔 *Replied user ID:* `{rid}` (@{rname})\n\
             💬 *Chat ID:* `{chat_id}` ({chat_type})"
        )
    } else {
        format!(
            "🆔 *Your ID:* `{user_id}` (@{username})\n\
             💬 *Chat ID:* `{chat_id}` ({chat_type})"
        )
    };

    let _ = bot.send_message(
        msg.chat.id,
        reply,
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

pub async fn cmd_calc(bot: &Bot, msg: &Message, expr: &str) {
    if expr.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /calc <expression>\nExample: /calc 2 + 2 * 3", None).await;
        return;
    }
    match eval_expr(expr.trim()) {
        Ok(result) => {
            let _ = bot.send_message(
                msg.chat.id,
                format!("🧮 `{expr}` = `{result}`"),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => {
            let _ = bot.send_message(msg.chat.id, format!("❌ Error: {e}"), None).await;
        }
    }
}

pub async fn cmd_b64(bot: &Bot, msg: &Message, args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    match parts.as_slice() {
        ["encode", text] | ["enc", text] => {
            let encoded = base64_encode(text.as_bytes());
            let _ = bot.send_message(
                msg.chat.id,
                format!("🔐 *Base64 Encoded:*\n`{encoded}`"),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        ["decode", text] | ["dec", text] => {
            match base64_decode(text.trim()) {
                Ok(decoded) => {
                    let _ = bot.send_message(
                        msg.chat.id,
                        format!("🔓 *Base64 Decoded:*\n`{decoded}`"),
                        Some(SendMessageParams::new().parse_mode("Markdown")),
                    ).await;
                }
                Err(_) => {
                    let _ = bot.send_message(msg.chat.id, "❌ Invalid base64 input.", None).await;
                }
            }
        }
        _ => {
            let _ = bot.send_message(
                msg.chat.id,
                "Usage:\n/b64 encode <text>\n/b64 decode <text>",
                None
            ).await;
        }
    }
}

pub async fn cmd_repeat(bot: &Bot, msg: &Message, args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    match parts.as_slice() {
        [n_str, text] => {
            let n: usize = n_str.parse().unwrap_or(1).min(10).max(1);
            let repeated = std::iter::repeat(*text).take(n).collect::<Vec<_>>().join("\n");
            let _ = bot.send_message(msg.chat.id, repeated, None).await;
        }
        _ => {
            let _ = bot.send_message(msg.chat.id, "Usage: /repeat <N> <text>  (max 10)", None).await;
        }
    }
}

// ── Simple expression evaluator ───────────────────────────────────────────────

fn eval_expr(expr: &str) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let mut pos = 0;
    let result = parse_expr(&tokens, &mut pos)?;
    if pos != tokens.len() {
        return Err("Unexpected token".to_string());
    }
    Ok(result)
}

#[derive(Debug, Clone)]
enum Token { Num(f64), Plus, Minus, Star, Slash, Caret, LParen, RParen }

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); }
            '0'..='9' | '.' => {
                let mut num = String::new();
                while chars.peek().map_or(false, |c| c.is_ascii_digit() || *c == '.') {
                    num.push(chars.next().unwrap());
                }
                tokens.push(Token::Num(num.parse().map_err(|_| "Bad number".to_string())?));
            }
            '+' => { tokens.push(Token::Plus);   chars.next(); }
            '-' => { tokens.push(Token::Minus);  chars.next(); }
            '*' => { tokens.push(Token::Star);   chars.next(); }
            '/' => { tokens.push(Token::Slash);  chars.next(); }
            '^' => { tokens.push(Token::Caret);  chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            _ => return Err(format!("Unknown character: '{c}'")),
        }
    }
    Ok(tokens)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_term(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Plus  => { *pos += 1; left += parse_term(tokens, pos)?; }
            Token::Minus => { *pos += 1; left -= parse_term(tokens, pos)?; }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_power(tokens, pos)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Star  => { *pos += 1; left *= parse_power(tokens, pos)?; }
            Token::Slash => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                if right == 0.0 { return Err("Division by zero".to_string()); }
                left /= right;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_power(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    let base = parse_unary(tokens, pos)?;
    if *pos < tokens.len() {
        if let Token::Caret = tokens[*pos] {
            *pos += 1;
            let exp = parse_unary(tokens, pos)?;
            return Ok(base.powf(exp));
        }
    }
    Ok(base)
}

fn parse_unary(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    if *pos < tokens.len() {
        if let Token::Minus = tokens[*pos] {
            *pos += 1;
            return Ok(-parse_primary(tokens, pos)?);
        }
    }
    parse_primary(tokens, pos)
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of expression".to_string());
    }
    match &tokens[*pos] {
        Token::Num(n) => { let v = *n; *pos += 1; Ok(v) }
        Token::LParen => {
            *pos += 1;
            let v = parse_expr(tokens, pos)?;
            if *pos < tokens.len() {
                if let Token::RParen = tokens[*pos] {
                    *pos += 1;
                    return Ok(v);
                }
            }
            Err("Missing closing parenthesis".to_string())
        }
        _ => Err("Expected number or '('".to_string()),
    }
}

// ── Simple Base64 (no external dep) ──────────────────────────────────────────

const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(input: &[u8]) -> String {
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let combined = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_CHARS[((combined >> 18) & 63) as usize] as char);
        out.push(B64_CHARS[((combined >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { B64_CHARS[((combined >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { B64_CHARS[(combined & 63) as usize] as char } else { '=' });
    }
    out
}

fn b64_val(c: u8) -> Option<u32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u32),
        b'a'..=b'z' => Some((c - b'a' + 26) as u32),
        b'0'..=b'9' => Some((c - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn base64_decode(s: &str) -> Result<String, ()> {
    let s = s.trim_end_matches('=');
    let mut out = Vec::new();
    let bytes: Vec<u8> = s.bytes().collect();
    for chunk in bytes.chunks(4) {
        let v: Vec<u32> = chunk.iter().filter_map(|&b| b64_val(b)).collect();
        if v.len() < 2 { break; }
        out.push(((v[0] << 2) | (v[1] >> 4)) as u8);
        if v.len() > 2 { out.push(((v[1] << 4) | (v[2] >> 2)) as u8); }
        if v.len() > 3 { out.push(((v[2] << 6) | v[3]) as u8); }
    }
    String::from_utf8(out).map_err(|_| ())
}
