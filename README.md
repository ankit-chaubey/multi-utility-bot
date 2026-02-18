# 🦀 Multi-Purpose Telegram Bot v0.2.0

A feature-packed Telegram bot built with [`tgbotrs`](https://crates.io/crates/tgbotrs) — a fully auto-generated Rust library covering all 285 Telegram Bot API types and 165 methods.

## 📦 Library

| Link | URL |
|---|---|
| **crates.io** | https://crates.io/crates/tgbotrs |
| **docs.rs** | https://docs.rs/tgbotrs |
| **GitHub** | https://github.com/ankit-chaubey/tgbotrs |
| **Telegram Bot API** | https://core.telegram.org/bots/api |

```toml
[dependencies]
tgbotrs = { version = "0.1.4" }
```

---

## 🚀 Quick Start

```bash
cp .env.example .env
# Edit .env — paste your TOKEN from @BotFather
cargo run --bin bot
```

---

## 📋 All Commands (54 total)

### ℹ️ General
| Command | Description |
|---|---|
| `/start` | Welcome screen with library links |
| `/help` | Interactive help menu (browseable by category) |
| `/about` | About this bot & tgbotrs library details |
| `/ping` | Check bot response time |
| `/source` | tgbotrs library links |

### 🎉 Fun (16 commands)
| Command | Description |
|---|---|
| `/dice` | Animated Telegram dice 🎲 |
| `/roll [N]` | Roll N-sided die (default d6) |
| `/flip` | Flip a coin 🪙 |
| `/joke` | Random programming joke |
| `/quote` | Inspiring developer quote |
| `/fact` | Random tech/programming fact |
| `/8ball <question>` | Magic 8-ball 🎱 |
| `/rps` | Rock Paper Scissors with inline buttons |
| `/choose a \| b \| c` | Randomly pick from options |
| `/rate <anything>` | Rate something out of 10 ⭐ |
| `/password [length]` | Secure random password (6–64 chars) |
| `/mock <text>` | aLtErNaTiNg CaSe |
| `/clap <text>` | Add 👏 between words |
| `/shrug` | ¯\\_(ツ)_/¯ |
| `/tableflip` | (╯°□°）╯︵ ┻━┻ |
| `/unflip` | ┬─┬ノ( º _ ºノ) |

### 🔧 Utility (12 commands)
| Command | Description |
|---|---|
| `/echo <text>` | Echo text back |
| `/reverse <text>` | Reverse text |
| `/upper <text>` | UPPERCASE |
| `/lower <text>` | lowercase |
| `/count <text>` | Count characters, words, lines |
| `/calc <expr>` | Calculator — `+ - * / ^ ( )` + `sqrt() abs() floor() ceil() round()` |
| `/b64 encode/decode <text>` | Base64 encode or decode |
| `/repeat <N> <text>` | Repeat text N times (max 10) |
| `/ascii <text>` | Convert text to ASCII codes |
| `/binary <text>` | Convert text to binary |
| `/time` | Current UTC time + unix timestamp |
| `/id` | Your Telegram ID (reply to see another user's) |

### ℹ️ Info (3 commands)
| Command | Description |
|---|---|
| `/userinfo` | Your user info (reply to see another's) |
| `/chatinfo` | Current chat info + member count |
| `/members` | Show member count |

### 👮 Admin — groups only (13 commands)
> Reply to a user's message, then use the command. Bot + you must be admins.

| Command | Description |
|---|---|
| `/ban` | Permanently ban user 🔨 |
| `/kick` | Kick (ban + immediate unban) 👢 |
| `/mute` | Remove all send permissions 🔇 |
| `/unmute` | Restore all send permissions 🔊 |
| `/warn [reason]` | Warn user — auto-bans at 3 warnings ⚠️ |
| `/warns` | Check user's warning count & reasons |
| `/clearwarns` | Clear all user's warnings |
| `/pin` | Pin replied message 📌 |
| `/unpin` | Unpin latest pinned message |
| `/del` | Delete replied message 🗑️ |
| `/promote` | Grant admin rights ⬆️ |
| `/demote` | Remove admin rights ⬇️ |
| `/invite` | Generate new invite link 🔗 |

### 🎮 Games (2 commands)
| Command | Description |
|---|---|
| `/guess` | Start number guessing game (1–100, 7 attempts) |
| `/guess <number>` | Make a guess (with progress bar) |
| `/giveup` | Reveal the number and end the game |

### 📝 Notes (4 commands)
| Command | Description |
|---|---|
| `/save <name> <content>` | Save a note |
| `/get <name>` | Retrieve a saved note |
| `/notes` | List all notes in this chat |
| `/delnote <name>` | Delete a note |

### 📊 Polls (2 commands)
| Command | Description |
|---|---|
| `/poll <question> \| <opt1> \| <opt2>` | Create a poll (up to 10 options) |
| `/quiz <question> \| <correct> \| <wrong1>` | Create a quiz (first option = correct answer) |

---

## 🏗️ Project Structure

```
src/
├── main.rs         → entry point, polling setup, command registration
├── handler.rs      → routes every update to the right module
├── state.rs        → shared in-memory state (warnings, notes, games)
├── kb.rs           → inline keyboard builder helpers
└── cmd/
    ├── fun.rs      → 16 fun commands
    ├── util.rs     → 12 utility commands + calc evaluator + base64
    ├── info.rs     → start/help/about/ping/source/userinfo/chatinfo
    ├── admin.rs    → 13 group admin commands
    ├── games.rs    → number guessing game with progress bar
    ├── notes.rs    → persistent in-memory notes per chat
    └── polls.rs    → polls and quizzes
```

---

## ⚙️ Configuration

Only one variable required:

```env
TOKEN=your_bot_token_from_botfather
```

---

## 📦 Dependencies

```toml
tgbotrs = { version = "0.1.4" }   # Telegram Bot API (285 types, 165 methods)
tokio   = { version = "1", features = ["full"] }
dotenvy = "0.15"                   # .env loading
chrono  = { version = "0.4", features = ["clock"] }
rand    = "0.8"
```
