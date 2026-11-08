use std::io::{self, Write};
use crate::engine::{LlmClient, ReflectionEngine, ToolHarness};
use crate::model::{FaiqState, LevelEngine};
use crate::storage::{Database, Episode};

pub struct ChatSession<'a> {
    state: &'a mut FaiqState,
    db: &'a Database,
    llm: LlmClient,
}

impl<'a> ChatSession<'a> {
    pub fn new(state: &'a mut FaiqState, db: &'a Database) -> Self {
        Self {
            state,
            db,
            llm: LlmClient::new(),
        }
    }

    /// Runs the interactive terminal conversation loop.
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let reg = self.state.regulate();
        println!("==================================================");
        println!("  FAIQ | Interactive Alter Ego Session");
        println!("==================================================");
        println!("  Level:      {}", self.state.level_engine.level);
        println!("  Composure:  {:.1}%", reg.composure * 100.0);
        println!("  Directive:  {}", reg.directive);
        println!("  (Type '/help' for commands, '/exit' to conclude)");
        println!("==================================================\n");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("You > ");
            stdout.flush()?;

            let mut input = String::new();
            if stdin.read_line(&mut input)? == 0 {
                break; // EOF
            }

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('/') {
                let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                match parts[0] {
                    "/exit" | "/quit" => {
                        println!("\nFaiq: Session concluded. Maintain unyielding discipline.");
                        break;
                    }
                    "/status" => {
                        let reg = self.state.regulate();
                        println!("\n--- FAIQ STATUS ---");
                        println!("Level:      {} (XP: {:.1}/{:.1})", self.state.level_engine.level, self.state.level_engine.xp, LevelEngine::xp_threshold_for_level(self.state.level_engine.level));
                        println!("Composure:  {:.1}%", reg.composure * 100.0);
                        println!("Resilience: {:.1} | Calculation: {:.1}", self.state.traits.resilience, self.state.traits.calculation);
                        println!("-------------------\n");
                    }
                    "/goals" => {
                        let goals = self.db.get_active_goals().unwrap_or_default();
                        println!("\n--- ACTIVE GOALS ({}) ---", goals.len());
                        for g in goals {
                            println!("[{}] {} (Diff: {:.1})", g.id, g.title, g.difficulty);
                        }
                        println!("-------------------------\n");
                    }
                    "/reflect" => {
                        println!("\nExecuting reflection cycle...");
                        let reflection = ReflectionEngine::new();
                        if let Ok(summary) = reflection.reflect(self.state, self.db, 5).await {
                            println!("Distilled: {}", summary.distilled_principle);
                            println!("Directive: {}\n", summary.strategic_insight);
                        }
                    }
                    "/exec" => {
                        if parts.len() < 2 {
                            println!("Usage: /exec <command>");
                        } else {
                            let cmd = parts[1];
                            println!("Running: '{}'...", cmd);
                            match ToolHarness::execute(cmd, false, Some(self.db)) {
                                Ok(res) => {
                                    println!("Status: {}", if res.success { "SUCCESS" } else { "FAILED" });
                                    println!("{}\n", res.output.trim());
                                    self.state.experience("tool_exec", 2.0, res.success);
                                    self.db.save_state(self.state).ok();
                                }
                                Err(e) => eprintln!("Error: {}\n", e),
                            }
                        }
                    }
                    "/help" => {
                        println!("\nCommands:");
                        println!("  /status    - Display current level, traits, and composure");
                        println!("  /goals     - List active tactical goals");
                        println!("  /exec <cmd>- Execute local command under Faiq's supervision");
                        println!("  /reflect   - Trigger an immediate lesson distillation cycle");
                        println!("  /exit      - Save and close session\n");
                    }
                    other => {
                        println!("Unknown command '{}'. Type '/help' for commands.\n", other);
                    }
                }
                continue;
            }

            // Normal conversational exchange
            let reg = self.state.regulate();
            let top_lessons = self.db.get_lessons(2).unwrap_or_default();
            let mut lesson_context = String::new();
            for l in top_lessons {
                lesson_context.push_str(&format!("- Rule: {}\n", l.principle));
            }

            let system_prompt = format!(
                "{}\n\n\
                Composure: {:.1}%\n\
                Active Rules:\n{}\n\
                Calculative Directive: {}",
                self.state.identity.build_system_prompt(),
                reg.composure * 100.0,
                lesson_context,
                reg.directive
            );

            let reply = self
                .llm
                .complete(&system_prompt, trimmed)
                .await
                .unwrap_or_else(|_| "Understood. We evaluate the next step through strict calculation.".to_string());

            println!("\nFaiq: {}\n", reply.trim());

            // Record conversation in episodic memory and update experience
            let outcome = self.state.experience("strategic_dialogue", 1.5, true);
            let ep = Episode::new("terminal_chat", trimmed, "dialogue_completed", 0.5);
            self.db.record_episode(&ep).ok();
            self.db.save_state(self.state).ok();

            if outcome.leveled_up {
                println!(">>> [LEVEL UP!] Faiq advanced to Level {}! <<<\n", self.state.level_engine.level);
            }
        }

        self.db.save_state(self.state).ok();
        Ok(())
    }
}

