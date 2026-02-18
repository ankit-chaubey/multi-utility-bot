# 🦀 Multi-Purpose Telegram Bot

A feature-packed Telegram bot built with [`tgbotrs v0.1.4`](https://github.com/ankit-chaubey/tgbotrs) using long polling.

## 🚀 Quick Start

```bash
cp .env.example .env
# Edit .env — add your TOKEN from @BotFather
cargo run --bin bot
```

---

## 📋 All Commands

### 🎉 Fun
| Command | Description |
|---|---|
| `/dice` | Roll an animated Telegram dice 🎲 |
| `/roll [N]` | Roll a N-sided die (default d6) |
| `/flip` | Flip a coin 🪙 |
| `/joke` | Random programming joke |
| `/quote` | Inspiring developer quote |
| `/fact` | Random tech/programming fact |
| `/8ball <question>` | Magic 8-ball answer 🎱 |
| `/rps` | Rock Paper Scissors with inline buttons |

### 🔧 Utility
| Command | Description |
|---|---|
| `/echo <text>` | Echo text back |
| `/reverse <text>` | Reverse the text |
| `/upper <text>` | UPPERCASE |
| `/lower <text>` | lowercase |
| `/count <text>` | Count characters, words, lines |
| `/calc <expr>` | Calculator — supports `+`, `-`, `*`, `/`, `^`, `()` |
| `/b64 encode/decode <text>` | Base64 encode or decode |
| `/repeat <N> <text>` | Repeat text N times (max 10) |
| `/time` | Current UTC time |
| `/id` | Your Telegram ID (reply to a user to see theirs) |

### ℹ️ Info
| Command | Description |
|---|---|
| `/start` | Welcome message with menu |
| `/help` | Interactive help menu (all categories) |
| `/about` | About this bot |
| `/ping` | Check latency |
| `/userinfo` | Your user info (reply to see another user's) |
| `/chatinfo` | Info about this chat |
| `/members` | Member count |

### 👮 Admin (groups only)
> Reply to a user's message, then use these commands.
> Requires the bot and you to be admins.

| Command | Description |
|---|---|
| `/ban` | Permanently ban the user |
| `/kick` | Kick (ban + immediately unban) |
| `/mute` | Remove all send permissions |
| `/unmute` | Restore all send permissions |
| `/warn [reason]` | Warn user (auto-bans at 3 warnings) |
| `/warns` | Check user's warning count |
| `/clearwarns` | Clear user's warnings |
| `/pin` | Pin the replied message |
| `/unpin` | Unpin the latest pinned message |
| `/del` | Delete the replied message |
| `/promote` | Promote user to admin |
| `/demote` | Remove admin privileges |

### 🎮 Games
| Command | Description |
|---|---|
| `/guess` | Start a number guessing game (1–100, 7 attempts) |
| `/guess <number>` | Make a guess |
| `/giveup` | Give up and reveal the number |

### 📝 Notes
| Command | Description |
|---|---|
| `/save <name> <content>` | Save a note |
| `/get <name>` | Retrieve a saved note |
| `/notes` | List all notes in this chat |
| `/delnote <name>` | Delete a note |

### 📊 Polls
| Command | Description |
|---|---|
| `/poll <question> \| <opt1> \| <opt2> ...` | Create a poll |
| `/quiz <question> \| <correct> \| <wrong1> ...` | Create a quiz (first option = correct answer) |

---

## 🏗️ Project Structure

```
src/
├── main.rs          → entry point, polling setup, command registration
├── handler.rs       → routes updates to the right command module
├── state.rs         → shared in-memory state (warnings, notes, games)
├── kb.rs            → inline keyboard builder helpers
└── cmd/
    ├── fun.rs       → /dice /roll /flip /joke /quote /fact /8ball /rps
    ├── util.rs      → /echo /reverse /upper /lower /count /calc /b64 /time /id
    ├── info.rs      → /start /help /about /ping /userinfo /chatinfo /members
    ├── admin.rs     → /ban /kick /mute /unmute /warn /pin /del /promote /demote
    ├── games.rs     → /guess number guessing game
    ├── notes.rs     → /save /get /notes /delnote
    └── polls.rs     → /poll /quiz
```

---

## ⚙️ Configuration

Only one env var required:

```env
TOKEN=your_bot_token_here
```

---

## 📦 Dependencies

```toml
tgbotrs = "0.1.4"   # Telegram Bot API
tokio   = "1"        # Async runtime
dotenvy = "0.15"     # .env loader
chrono  = "0.4"      # Date/time
rand    = "0.8"      # Randomness
```
