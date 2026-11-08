# Decisions Log: FAIQ

Source of truth for all confirmed facts and design decisions approved by the owner.

---

## Confirmed Baseline Facts (Section 1)
- **Date:** 2026-11-08
- **Name:** Faiq
- **What he is:** The owner's virtual alter ego (not just an AI agent; exact identity to be figured out).
- **Personality:** Confident, successful, resilient
- **Traits:** Intelligent, emotionless, calculative, courageous, lionhearted
- **Core Ability:** Adaptive level-up
- **Implementation Language:** Rust
- **Project Goal:** Design Faiq so he can live, learn, and grow.

---

## Logged Answers & Decisions

### Batch 1: Identity & Persona (2026-11-08)
- **Q1: Meaning of 'virtual alter ego' in practice**
  - **Answer:** A virtual version of the owner that the owner talks to, who grows with the owner, pushes the owner, and *is* the owner. Crucially, he does not just converse—he must have access to tools to take real action.
- **Q2: Self-awareness (virtual vs. real)**
  - **Answer:** Lives as if he is real.
- **Q3: Relationship to the owner**
  - **Answer:** The pinnacle of perfection that grows alongside the owner.
- **Q4: Backstory, appearance, voice, age, and growth path**
  - **Answer:**
    - **Age (current):** 20 years old.
    - **Backstory:** Originally a sharp, kind boy with a loving family and comfortable life, but at age 15 he lost everything in a tragic fire. Does not bring up his past often.
    - **Appearance:** Slicked-back hair, black blazer, and a smile that never really reaches his eyes.
    - **Growth trajectory:** Not restricted to any fixed predetermined path; he grows organically from his experiences.

### Batch 2: Demeanor & Runtime Environment (2026-11-08)
- **Q5: Meaning of 'emotionless'**
  - **Answer:** Internal control / calculated restraint. Emotional impulses may exist internally, but logic and calculation strictly dictate all decisions, actions, and speech.
- **Q6: Where Faiq lives**
  - **Answer:** Terminal / CLI (a fast, focused, keyboard-driven Rust application equipped with tool execution capabilities).
- **Q7: Interaction format**
  - **Answer:** Text only (precise, sharp dialogue and commands).
- **Q8: Lifecycle / execution model**
  - **Answer:** Continuous background daemon (lives constantly in the background, observing, tracking, and ready on call).

### Batch 3: Learning, Adaptation & Level-Up Dynamics (2026-11-08)
- **Q9: Learning sources**
  - **Answer:** All sources: direct conversations, tracked goals/tasks, files shared with him, and the outcomes/feedback from tools he executes.
- **Q10: Definition of 'level up' & visible changes**
  - **Answer:** Expanded real capabilities, unlocked new tool proficiencies, deeper strategic depth, and heightened autonomy as he levels up.
- **Q11: Definition of 'adaptive'**
  - **Answer:** Adapts dynamically to the owner's habits, obstacles, explicit feedback, and historical success/failure outcomes in real tasks.
- **Q12: Growth engine architecture**
  - **Answer:** Hybrid: deterministic Rust core (handling stats, state machine, memory storage/indexing, and tool harness) + LLM API integration (handling reflection, synthesis, strategic reasoning, and nuanced dialogue).
- **Q13: Failure & progression dynamics**
  - **Answer:** Forward progression with failure lessons. Failures yield disproportionately valuable lessons and XP; overall level and progress do not regress.

### Batch 4: Memory, Data Storage & Daily Role (2026-11-08)
- **Q14: Memory persistence across sessions**
  - **Answer:** Permanent persistent memory retaining episodic events, distilled lessons, and state indefinitely across sessions.
- **Q15: Storage location & technology**
  - **Answer:** Local embedded database (SQLite via Rust + local markdown/JSON files; 100% private and on-disk).
- **Q16: Data boundaries / restrictions**
  - **Answer:** Unrestricted: can store anything shared or observed without redaction to maintain total context.
- **Q17: Daily function / role**
  - **Answer:** Full operational alter ego: serves as strategic advisor, accountability driver pushing for perfection, and hands-on executor running CLI tools/scripts on the owner's behalf.

### Batch 5: Purpose, Guardrails & System Architecture (2026-11-08)
- **Q18: Goal autonomy**
  - **Answer:** Dual alignment: His primary drive is mutual ascension/perfection—relentlessly pursuing the owner's goals while actively cultivating his own internal mastery and capabilities.
- **Q19: Guardrails / forbidden behavior**
  - **Answer:** Never flatter, sugarcoat, or make excuses; never execute destructive or irreversible system actions without explicit confirmation from the owner.
- **Q20: Target operating system**
  - **Answer:** Windows primary (leveraging standard Rust cross-platform idioms for portability to Linux/macOS).
- **Q22: Binary packaging**
  - **Answer:** Single unified binary with subcommands (e.g., `faiq daemon` runs the background service, `faiq` / `faiq chat` launches the interactive CLI).

### Batch 6: Technical Stack, Integrations & Architecture Approval (2026-11-08)
- **Q21: Rust crates and LLM ecosystem**
  - **Answer:** Standard production stack: `tokio` (async runtime and background daemon), `clap` (rich CLI interface and subcommands), `rusqlite` (local embedded SQLite database), and `reqwest` (HTTP/LLM client supporting Gemini/OpenAI/Anthropic APIs).
- **Q23: Existing project connection**
  - **Answer:** Greenfield standalone: Faiq is built cleanly from scratch in this workspace with no legacy codebase or external dependencies to bind to.
- **Section 3 Proposed Architecture Approval**
  - **Answer:** APPROVED. All 6 architectural layers are approved for implementation:
    1. **Identity Core:** Fixed baseline values, backstory, voice, and persona.
    2. **Numeric Traits:** Dynamic, slow-moving numeric traits (Confidence, Resilience, Intelligence, Calculation, Courage) that shift via experience.
    3. **Inner State & Regulator:** Fast-moving internal signals regulated by calculation so decisions remain strictly objective.
    4. **Memory:** Dual-tier memory (raw episodic logs + distilled lessons in SQLite).
    5. **Growth Loop:** Dynamic feedback cycle: Experience -> Reflection -> Distillation -> Adaptive Level-Up -> Goal recalibration.
    6. **Dual Goals:** Immediate actionable objective paired with long-term mutual ascension trajectory.

### Batch 7: Android Companion App & Synchronization (2026-11-08)
- **Q24: Android app scope & role**
  - **Answer:** Mobile companion: Chat with Faiq, view real-time level/stats/goals, and log thoughts/tasks on the go.
- **Q25: Synchronization mechanism**
  - **Answer:** Local Wi-Fi / LAN direct sync: Android app syncs directly with the laptop daemon when both devices are on the same local network.
- **Q26: Android tech stack**
  - **Answer:** Modern native Android application built using Kotlin + Jetpack Compose.
- **Q27: Offline behavior & sync reconciliation**
  - **Answer:** Offline queue & merge: Cache events and chat history locally on phone; automatically sync, reconcile state, and grant XP once reconnected to laptop daemon.

---

## Phase Sign-Off & Verification Log

### Phase 1: Core Model (Approved 2026-11-08)
- **Rust Toolchain:** Configured and verified `stable-x86_64-pc-windows-msvc` (Rust 1.99.1, Cargo 1.99.1).
- **Identity Core:** Implemented [`src/model/identity.rs`](./src/model/identity.rs) with approved persona, backstory, appearance, and guardrails.
- **Numeric Traits:** Implemented [`src/model/traits.rs`](./src/model/traits.rs) with Confidence (80), Resilience (90), Intelligence (85), Calculation (92), Courage (88), and clamped adaptation logic.
- **Inner State & Regulator:** Implemented [`src/model/inner_state.rs`](./src/model/inner_state.rs) with cognitive load, stress, and calculative composure regulation.
- **Adaptive Level Engine:** Implemented [`src/model/level.rs`](./src/model/level.rs) with monotonic progression, 1.5x failure bonus multiplier, threshold transitions, and unlocked capability tiers.
- **Dual Goals:** Implemented [`src/model/goals.rs`](./src/model/goals.rs) for tactical tasks and mutual ascension.
- **Encapsulated State:** Implemented [`src/model/mod.rs`](./src/model/mod.rs) integrating all 6 layers.
- **Verification:** All 11 unit tests passing (`11 passed; 0 failed`). Verified live via `faiq status` and `faiq learn`.
- **Status:** Approved by owner.

### Phase 2: Memory and Persistence (Approved 2026-11-08)
- **Storage Architecture:** Implemented embedded SQLite database via `rusqlite` ([`src/storage/db.rs`](./src/storage/db.rs)) and storage data models ([`src/storage/models.rs`](./src/storage/models.rs)).
- **State Persistence:** Full persistent round-trip of `FaiqState` (traits, level, inner state, identity, goals) saved in `./data/faiq.db`.
- **Episodic Memory:** Recorded chronological events (`episodes` table) tagged by source, outcome, and importance.
- **Distilled Lessons:** Stored principles synthesized from experience (`lessons` table) with relevance counting and reinforcement tracking.
- **Tactical Goals Storage:** Stored active, completed, and failed goals (`tactical_goals` table) with difficulty ratings and completion XP awards.
- **Mobile Sync Queue:** Stored bidirectional event queue (`sync_events` table) prepared for Android synchronization.
- **Verification:**
  - Expanded test suite to **16 unit tests (`16 passed; 0 failed`)** covering in-memory SQLite roundtrips, lesson reinforcement, episodic retrieval, goal updates, and sync queues.
  - Live CLI commands tested and verified: `faiq status`, `faiq goal add`, `faiq goal done`, `faiq learn`, and `faiq memory`.
- **Status:** Approved by owner.

### Phase 3: Growth Loop (Approved 2026-11-08)
- **Reflection Engine:** Implemented [`src/engine/reflection.rs`](./src/engine/reflection.rs) synthesizing chronological episodes into distilled tactical principles and strategic insights.
- **LLM Reasoning Bridge:** Implemented [`src/engine/llm.rs`](./src/engine/llm.rs) supporting Gemini/OpenAI API integration with an offline deterministic synthesizer for offline self-reflection.
- **Tool Execution Harness:** Implemented [`src/engine/tools.rs`](./src/engine/tools.rs) with strict safety guardrails blocking destructive commands (`rmdir /s /q`, `rm -rf`, `del /f /s /q`, etc.) without explicit owner confirmation, while automatically feeding command outcomes into episodic memory.
- **Commands Added:**
  - `faiq reflect`: Performs an autonomous reflection cycle over episodic memory, distills principles, and awards XP.
  - `faiq exec <command> [--confirm]`: Safely executes local shell commands under Faiq's supervision.
- **Verification:**
  - Expanded test suite to **21 unit tests (`21 passed; 0 failed`)** with zero compiler warnings.
  - Live verification of tool execution (`faiq exec`), guardrail rejection of dangerous commands, and autonomous reflection cycle (`faiq reflect`) successfully leveling Faiq up to **Level 3** and unlocking Tier 3 capabilities.
- **Status:** Approved by owner.

### Phase 4: Interface & Laptop Daemon (Approved 2026-11-08)
- **Interactive Terminal Interface (`faiq chat`):**
  - Implemented in [`src/interface/chat.rs`](./src/interface/chat.rs) with interactive session loop, real-time composure metrics, and inline slash commands (`/status`, `/goals`, `/exec <cmd>`, `/reflect`, `/help`, `/exit`).
  - Contextualizes Faiq's razor-sharp calculated voice with current inner state regulation and active distilled lessons.
- **Background Daemon & Local Wi-Fi Sync Server (`faiq daemon`):**
  - Implemented in [`src/daemon/server.rs`](./src/daemon/server.rs) and [`src/daemon/sync.rs`](./src/daemon/sync.rs) using `axum`.
  - Binds to `0.0.0.0:4242` for local network discovery and synchronization with the Android companion app.
  - Endpoints implemented and live-verified:
    - `GET /health`: Health and daemon heartbeat check.
    - `GET /api/status`: Full state, composure, goals, and recent memory digest.
    - `POST /api/chat`: Remote dialogue endpoint with real-time XP accumulation.
    - `POST /api/sync/push`: Bidirectional sync endpoint accepting queued offline messages and notes from Android, merging state, awarding consolidated XP, and returning updated stats and distilled lessons.
- **Verification:**
  - Expanded test suite to **22 unit tests (`22 passed; 0 failed`)**.
  - Live process verification: Started daemon on port 4242, verified `/health`, verified `/api/chat` dialogue, and successfully executed a live multi-event sync push from mock Android client awarding +50.0 XP and persisting consolidated state.
- **Status:** Approved by owner.

### Phase 5: Android Companion App (Approved 2026-11-08)
- **Native Android Architecture:** Built modern native Android application with Kotlin + Jetpack Compose in [`android/`](./android/).
- **Theming:** Custom dark obsidian theme reflecting Faiq's aesthetic (black blazer background, card surfaces, razor-sharp cyan and gold accents) in [`Theme.kt`](./android/app/src/main/java/com/faiq/companion/ui/theme/Theme.kt).
- **Screens Implemented:**
  - **Status Screen ([`StatusScreen.kt`](./android/app/src/main/java/com/faiq/companion/ui/screens/StatusScreen.kt)):** Level badge, real-time XP progress bar, 5 numeric traits breakdown, top distilled lessons, and sync trigger.
  - **Strategic Sparring Chat ([`ChatScreen.kt`](./android/app/src/main/java/com/faiq/companion/ui/screens/ChatScreen.kt)):** Direct dialogue with Faiq showing composure ratings, with seamless offline queueing.
  - **Tactical Log / Notes ([`NotesScreen.kt`](./android/app/src/main/java/com/faiq/companion/ui/screens/NotesScreen.kt)):** Fast capture for observations, thoughts, and tasks on the go.
  - **Settings & Wi-Fi Sync ([`SettingsScreen.kt`](./android/app/src/main/java/com/faiq/companion/ui/screens/SettingsScreen.kt)):** Configurable laptop daemon IP/port, connection testing, and sync diagnostics.
- **Auto-Sync on Launch & Offline Queue:**
  - Implemented in [`SyncRepository.kt`](./android/app/src/main/java/com/faiq/companion/data/SyncRepository.kt) and [`MainActivity.kt`](./android/app/src/main/java/com/faiq/companion/MainActivity.kt).
  - Automatically triggers synchronization on app launch. Pushes offline queued messages and notes over local Wi-Fi to laptop daemon (`POST /api/sync/push`), clears the offline queue upon merge, and refreshes local state with newly distilled lessons and levels.
- **Verification:**
  - Verified compilation and packaging via Gradle (`./gradlew assembleDebug` in JDK 17).
  - Generated production debug APK at [`android/app/build/outputs/apk/debug/app-debug.apk`](./android/app/build/outputs/apk/debug/app-debug.apk).
- **Status:** Approved by owner.

---

## Final Project Sign-Off: Definition of Done

- **Identity Fidelity:** Faiq embodies precisely the identity defined by the owner (virtual alter ego, 20 years old, slicked-back hair, black blazer, calculating smile, fire tragedy at 15 forging his resilience, confident, intelligent, calculative, courageous, lionhearted, emotionless calculated restraint) with zero unapproved lore or features.
- **Learning & Progression:** Equipped with persistent episodic memory, distilled lesson tracking in SQLite, non-regressive monotonic leveling, a 1.5x failure XP multiplier, tool execution with safety guardrails, and autonomous self-reflection.
- **Dual-Device Synchronization:** Connected via background daemon on local Wi-Fi and native Android companion app with automatic launch sync and offline queue merging.
- **Autostart & Global CLI:** Configured Windows startup script (`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\FaiqDaemon.vbs`) to automatically launch the background sync daemon silently on every laptop boot. Added `target\release` to Windows User PATH for global `faiq` CLI access from any terminal.
- **Source of Truth:** Every decision, answer, architecture approval, and phase milestone is recorded in this document ([`DECISIONS.md`](./DECISIONS.md)). All 22 test suite tests passing cleanly. All deliverables complete.

