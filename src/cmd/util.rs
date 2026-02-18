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
        format!("📊 *Text Stats*\n• Characters: `{chars}`\n• Words: `{words}`\n• Lines: `{lines}`"),
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
    let user_id   = msg.from.as_ref().map(|u| u.id).unwrap_or(0);
    let username  = msg.from.as_ref().and_then(|u| u.username.as_deref()).unwrap_or("none");
    let chat_id   = msg.chat.id;
    let chat_type = &msg.chat.r#type;

    let reply = if let Some(r) = &msg.reply_to_message {
        let rid   = r.from.as_ref().map(|u| u.id).unwrap_or(0);
        let rname = r.from.as_ref().and_then(|u| u.username.as_deref()).unwrap_or("none");
        format!(
            "🆔 *Your ID:* `{user_id}` (@{username})\n\
             🆔 *Replied user:* `{rid}` (@{rname})\n\
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
        let _ = bot.send_message(
            msg.chat.id,
            "🧮 *Calculator*\n\
             Usage: `/calc <expression>`\n\n\
             Supports: `+ - * / ^ ( )`\n\
             Functions: `sqrt( )` `abs( )` `floor( )` `ceil( )` `round( )`\n\n\
             Examples:\n\
             `/calc 2^10`\n\
             `/calc sqrt(144) + 5`\n\
             `/calc (3+4) * (2^3) - 1`",
            Some(SendMessageParams::new().parse_mode("Markdown")),
        ).await;
        return;
    }
    match eval_expr(expr.trim()) {
        Ok(result) => {
            let display = if result.fract() == 0.0 && result.abs() < 1e15 {
                format!("{}", result as i64)
            } else {
                let s = format!("{result:.10}");
                s.trim_end_matches('0').trim_end_matches('.').to_string()
            };
            let _ = bot.send_message(
                msg.chat.id,
                format!("🧮 `{expr}` = `{display}`"),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        Err(e) => {
            let _ = bot.send_message(msg.chat.id, format!("❌ {e}"), None).await;
        }
    }
}

pub async fn cmd_b64(bot: &Bot, msg: &Message, args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    match parts.as_slice() {
        ["encode" | "enc", text] => {
            let encoded = base64_encode(text.as_bytes());
            let _ = bot.send_message(
                msg.chat.id,
                format!("🔐 *Base64 encoded:*\n`{encoded}`"),
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
        ["decode" | "dec", text] => {
            match base64_decode(text.trim()) {
                Ok(d) => {
                    let _ = bot.send_message(
                        msg.chat.id,
                        format!("🔓 *Base64 decoded:*\n`{d}`"),
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
                "Usage:\n`/b64 encode <text>`\n`/b64 decode <text>`",
                Some(SendMessageParams::new().parse_mode("Markdown")),
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
            let _ = bot.send_message(
                msg.chat.id,
                "Usage: `/repeat <N> <text>` (max 10)",
                Some(SendMessageParams::new().parse_mode("Markdown")),
            ).await;
        }
    }
}

/// /ascii <text> — show ASCII codes
pub async fn cmd_ascii(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /ascii <text>", None).await;
        return;
    }
    let result: String = args.chars()
        .map(|c| format!("{}", c as u32))
        .collect::<Vec<_>>()
        .join(" ");
    let preview = if result.len() > 300 {
        format!("{}…", &result[..300])
    } else {
        result
    };
    let _ = bot.send_message(
        msg.chat.id,
        format!("🔢 *ASCII codes:*\n`{preview}`"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

/// /binary <text> — convert to binary
pub async fn cmd_binary(bot: &Bot, msg: &Message, args: &str) {
    if args.trim().is_empty() {
        let _ = bot.send_message(msg.chat.id, "Usage: /binary <text>", None).await;
        return;
    }
    let result: String = args.chars()
        .map(|c| format!("{:08b}", c as u32))
        .collect::<Vec<_>>()
        .join(" ");
    let preview = if result.len() > 300 {
        format!("{}…", &result[..300])
    } else {
        result
    };
    let _ = bot.send_message(
        msg.chat.id,
        format!("💾 *Binary:*\n`{preview}`"),
        Some(SendMessageParams::new().parse_mode("Markdown")),
    ).await;
}

// ─── Expression evaluator (supports +−×÷^ and sqrt/abs/floor/ceil/round) ──────

fn eval_expr(input: &str) -> Result<f64, String> {
    // Replace named functions with single-char tokens
    let s = input
        .replace("sqrt", "√")
        .replace("abs",  "±")
        .replace("floor","⌊")
        .replace("ceil", "⌈")
        .replace("round","○");
    let tokens = tokenize(&s)?;
    let mut pos = 0usize;
    let result  = parse_expr(&tokens, &mut pos)?;
    if pos != tokens.len() {
        return Err("Unexpected token at end of expression".into());
    }
    Ok(result)
}

#[derive(Clone)]
enum Tok { Num(f64), Add, Sub, Mul, Div, Pow, LP, RP, Sqrt, Abs, Floor, Ceil, Round }

fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); }
            '0'..='9' | '.' => {
                let mut n = String::new();
                while chars.peek().map_or(false, |&x| x.is_ascii_digit() || x == '.') {
                    n.push(chars.next().unwrap());
                }
                out.push(Tok::Num(n.parse().map_err(|_| "Invalid number".to_string())?));
            }
            '+' => { out.push(Tok::Add);   chars.next(); }
            '-' => { out.push(Tok::Sub);   chars.next(); }
            '*' => { out.push(Tok::Mul);   chars.next(); }
            '/' => { out.push(Tok::Div);   chars.next(); }
            '^' => { out.push(Tok::Pow);   chars.next(); }
            '(' => { out.push(Tok::LP);    chars.next(); }
            ')' => { out.push(Tok::RP);    chars.next(); }
            '√' => { out.push(Tok::Sqrt);  chars.next(); }
            '±' => { out.push(Tok::Abs);   chars.next(); }
            '⌊' => { out.push(Tok::Floor); chars.next(); }
            '⌈' => { out.push(Tok::Ceil);  chars.next(); }
            '○' => { out.push(Tok::Round); chars.next(); }
            _   => return Err(format!("Unknown character: '{c}'")),
        }
    }
    Ok(out)
}

fn parse_expr(t: &[Tok], p: &mut usize) -> Result<f64, String> {
    let mut v = parse_term(t, p)?;
    while *p < t.len() {
        match &t[*p] {
            Tok::Add => { *p += 1; v += parse_term(t, p)?; }
            Tok::Sub => { *p += 1; v -= parse_term(t, p)?; }
            _ => break,
        }
    }
    Ok(v)
}

fn parse_term(t: &[Tok], p: &mut usize) -> Result<f64, String> {
    let mut v = parse_pow(t, p)?;
    while *p < t.len() {
        match &t[*p] {
            Tok::Mul => { *p += 1; v *= parse_pow(t, p)?; }
            Tok::Div => {
                *p += 1;
                let r = parse_pow(t, p)?;
                if r == 0.0 { return Err("Division by zero".into()); }
                v /= r;
            }
            _ => break,
        }
    }
    Ok(v)
}

fn parse_pow(t: &[Tok], p: &mut usize) -> Result<f64, String> {
    let base = parse_unary(t, p)?;
    if *p < t.len() {
        if let Tok::Pow = &t[*p] {
            *p += 1;
            return Ok(base.powf(parse_unary(t, p)?));
        }
    }
    Ok(base)
}

fn parse_unary(t: &[Tok], p: &mut usize) -> Result<f64, String> {
    if *p < t.len() {
        if let Tok::Sub = &t[*p] { *p += 1; return Ok(-parse_primary(t, p)?); }
    }
    parse_primary(t, p)
}

fn parse_primary(t: &[Tok], p: &mut usize) -> Result<f64, String> {
    if *p >= t.len() { return Err("Unexpected end of expression".into()); }
    match &t[*p].clone() {
        Tok::Num(n) => { let v = *n; *p += 1; Ok(v) }
        Tok::LP => {
            *p += 1;
            let v = parse_expr(t, p)?;
            if *p < t.len() {
                if let Tok::RP = &t[*p] { *p += 1; return Ok(v); }
            }
            Err("Missing closing ')'".into())
        }
        Tok::Sqrt  => { *p += 1; let v = parse_primary(t, p)?; Ok(v.sqrt()) }
        Tok::Abs   => { *p += 1; let v = parse_primary(t, p)?; Ok(v.abs()) }
        Tok::Floor => { *p += 1; let v = parse_primary(t, p)?; Ok(v.floor()) }
        Tok::Ceil  => { *p += 1; let v = parse_primary(t, p)?; Ok(v.ceil()) }
        Tok::Round => { *p += 1; let v = parse_primary(t, p)?; Ok(v.round()) }
        _ => Err("Expected number or '('".into()),
    }
}

// ─── Tiny base64 (no external dep) ───────────────────────────────────────────

const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(input: &[u8]) -> String {
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let c  = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64[((c >> 18) & 63) as usize] as char);
        out.push(B64[((c >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { B64[((c >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { B64[(c & 63) as usize] as char } else { '=' });
    }
    out
}

fn b64v(c: u8) -> Option<u32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u32),
        b'a'..=b'z' => Some((c - b'a' + 26) as u32),
        b'0'..=b'9' => Some((c - b'0' + 52) as u32),
        b'+'        => Some(62),
        b'/'        => Some(63),
        _           => None,
    }
}

fn base64_decode(s: &str) -> Result<String, ()> {
    let s = s.trim_end_matches('=');
    let mut out = Vec::new();
    let bytes: Vec<u8> = s.bytes().collect();
    for chunk in bytes.chunks(4) {
        let v: Vec<u32> = chunk.iter().filter_map(|&b| b64v(b)).collect();
        if v.len() < 2 { break; }
        out.push(((v[0] << 2) | (v[1] >> 4)) as u8);
        if v.len() > 2 { out.push(((v[1] << 4) | (v[2] >> 2)) as u8); }
        if v.len() > 3 { out.push(((v[2] << 6) | v[3]) as u8); }
    }
    String::from_utf8(out).map_err(|_| ())
}
