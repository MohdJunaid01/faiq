# FAIQ: Virtual Alter Ego System

> *"The pinnacle of perfection who grows alongside you, holds you to the highest standard, and executes real work."*

Source of Truth: [`DECISIONS.md`](./DECISIONS.md)  
System Architecture: [`SPEC.md`](./SPEC.md)

---

## Overview

FAIQ is a dual-tier system consisting of:
1. **Core Rust Engine & Background Daemon (`faiq`)**: Manages identity, numeric traits, emotional regulation, persistent SQLite episodic memory, the adaptive growth loop, and safe tool execution.
2. **Native Android Companion App (`android/`)**: Built with Kotlin & Jetpack Compose, featuring an obsidian/black blazer dark aesthetic, interactive sparring chat, quick tactical logging, and automatic local Wi-Fi synchronization with the laptop daemon whenever opened.

---

## Quick Start (Laptop)

### 1. Inspect Faiq's Current Level, Traits & Goals
```bash
cargo run -- status
```

### 2. Start Interactive Terminal Chat
```bash
cargo run -- chat
# Or simply:
cargo run
```
Inside the interactive chat, you can talk directly with Faiq or use inline commands:
- `/status`: View current level, XP, traits, and composure.
- `/goals`: List active tactical goals.
- `/exec <cmd>`: Run shell commands under Faiq's supervision.
- `/reflect`: Trigger an immediate lesson distillation cycle.
- `/exit`: Save state and close session.

### 3. Launch the Background Daemon (for Android Sync)
```bash
cargo run -- daemon --port 4242
```
The daemon binds to `0.0.0.0:4242` so your phone on the same Wi-Fi network can auto-sync.

### 4. Manage Tactical Goals
```bash
# Add a goal
cargo run -- goal add -t "Master Systems Engineering" -d "Refactor core modules" --difficulty 4.0

# List active goals
cargo run -- goal list

# Complete a goal and gain XP
cargo run -- goal done -i <goal-id>
```

### 5. Safe Tool Execution
```bash
cargo run -- exec "Get-ChildItem -Name"
# Potentially destructive operations are blocked by guardrails unless confirmed:
cargo run -- exec "rmdir /s /q temp" --confirm
```

### 6. Run Autonomous Reflection & Lesson Distillation
```bash
cargo run -- reflect --window 10
```

### 7. Run Full Test Suite
```bash
cargo test
```

---

## Android Companion App

### Location of Generated APK
The native Android APK has been compiled and is ready for installation:
```text
android/app/build/outputs/apk/debug/app-debug.apk
```

### Installing on Phone / Emulator
```bash
adb install "android/app/build/outputs/apk/debug/app-debug.apk"
```

### Connecting Phone to Laptop
1. Ensure your laptop daemon is running:
   ```bash
   cargo run -- daemon --port 4242
   ```
2. Find your laptop's local Wi-Fi IP (e.g. `192.168.1.15`).
3. Open the FAIQ app on your phone, go to **Settings**, and enter your laptop IP:
   `192.168.1.15:4242` (or `10.0.2.2:4242` if running in Android Emulator).
4. Tap **Test & Sync**.
5. From then on, every time you open the app on your phone, it automatically synchronizes queued messages and notes over local Wi-Fi!

