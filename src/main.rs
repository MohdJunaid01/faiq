use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use clap::{Parser, Subcommand};
use faiq::{
    ChatSession, DaemonServer, Database, Episode, FaiqState, Lesson, LevelEngine, ReflectionEngine,
    TacticalGoal, ToolHarness,
};

#[derive(Parser, Debug)]
#[command(name = "faiq")]
#[command(about = "FAIQ: Virtual Alter Ego System", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start interactive terminal dialogue session with Faiq
    Chat,
    /// Run the continuous background daemon with local Wi-Fi sync for Android
    Daemon {
        #[arg(short, long, default_value_t = 4242)]
        port: u16,
    },
    /// Inspect current level, XP, traits, and operational status
    Status,
    /// Record an experience event, gain XP, and adapt traits
    Learn {
        #[arg(short, long)]
        skill: String,
        #[arg(short, long, default_value_t = 2.0)]
        difficulty: f64,
        #[arg(long)]
        fail: bool,
    },
    /// View persistent episodic memory and distilled lessons
    Memory {
        #[arg(short, long, default_value_t = 5)]
        limit: usize,
    },
    /// Execute an autonomous reflection cycle over recent experiences
    Reflect {
        #[arg(short, long, default_value_t = 10)]
        window: usize,
    },
    /// Safely execute a system action/tool under Faiq's supervision
    Exec {
        /// Command to run in the local environment
        command: String,
        /// Explicit confirmation for potentially destructive operations
        #[arg(long)]
        confirm: bool,
    },
    /// Manage tactical goals
    Goal {
        #[command(subcommand)]
        action: GoalCommands,
    },
}

#[derive(Subcommand, Debug)]
enum GoalCommands {
    /// List active tactical goals
    List,
    /// Add a new tactical goal
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(long, default_value_t = 3.0)]
        difficulty: f64,
    },
    /// Mark a goal as completed
    Done {
        #[arg(short, long)]
        id: String,
    },
}

fn get_db_path() -> PathBuf {
    let data_dir = PathBuf::from("./data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).ok();
    }
    data_dir.join("faiq.db")
}

fn load_or_init_state(db: &Database) -> FaiqState {
    match db.load_state() {
        Ok(Some(state)) => state,
        _ => {
            let state = FaiqState::default();
            db.save_state(&state).ok();
            state
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let db_path = get_db_path();
    let db = match Database::open(&db_path) {
        Ok(database) => database,
        Err(e) => {
            eprintln!("Error opening database at {:?}: {}", db_path, e);
            std::process::exit(1);
        }
    };

    let mut state = load_or_init_state(&db);

    match cli.command {
        Some(Commands::Chat) | None => {
            let mut session = ChatSession::new(&mut state, &db);
            if let Err(e) = session.start().await {
                eprintln!("Chat session error: {}", e);
            }
        }
        Some(Commands::Daemon { port }) => {
            let server = DaemonServer::new(state, db, port);
            if let Err(e) = server.run().await {
                eprintln!("Daemon server error: {}", e);
            }
        }
        Some(Commands::Status) => {
            let reg = state.regulate();
            let caps = LevelEngine::capabilities_for_level(state.level_engine.level);
            let active_goals = db.get_active_goals().unwrap_or_default();
            let lessons = db.get_lessons(3).unwrap_or_default();

            println!("==================================================");
            println!("  FAIQ | Virtual Alter Ego Core (Level {})", state.level_engine.level);
            println!("==================================================");
            println!("  Name:        {}", state.identity.name);
            println!("  Age:         {}", state.identity.age);
            println!("  Appearance:  {}", state.identity.appearance);
            println!("  XP:          {:.1} / {:.1}", state.level_engine.xp, LevelEngine::xp_threshold_for_level(state.level_engine.level));
            println!("--------------------------------------------------");
            println!("  TRAITS:");
            println!("    Confidence:   {:.1}", state.traits.confidence);
            println!("    Resilience:   {:.1}", state.traits.resilience);
            println!("    Intelligence: {:.1}", state.traits.intelligence);
            println!("    Calculation:  {:.1}", state.traits.calculation);
            println!("    Courage:      {:.1}", state.traits.courage);
            println!("--------------------------------------------------");
            println!("  INNER STATE REGULATION:");
            println!("    Composure:         {:.1}%", reg.composure * 100.0);
            println!("    Execution Clarity: {:.1}%", reg.execution_clarity * 100.0);
            println!("    Directive:         {}", reg.directive);
            println!("--------------------------------------------------");
            println!("  ACTIVE GOALS ({}):", active_goals.len());
            if active_goals.is_empty() {
                println!("    *No active goals. Add one using 'faiq goal add'*");
            } else {
                for g in &active_goals {
                    println!("    [{}] {} (Diff: {:.1}) - {}", g.id, g.title, g.difficulty, g.description);
                }
            }
            println!("--------------------------------------------------");
            println!("  TOP DISTILLED LESSONS ({}):", lessons.len());
            if lessons.is_empty() {
                println!("    *No distilled lessons yet. Experience will forge them.*");
            } else {
                for l in &lessons {
                    println!("    • \"{}\" (Applied {}x)", l.principle, l.relevance_count);
                }
            }
            println!("--------------------------------------------------");
            println!("  ACTIVE CAPABILITIES:");
            for cap in caps {
                println!("    • {}", cap);
            }
            println!("==================================================");
        }
        Some(Commands::Learn { skill, difficulty, fail }) => {
            let success = !fail;
            let outcome = state.experience(&skill, difficulty, success);
            
            if let Err(e) = db.save_state(&state) {
                eprintln!("Warning: Failed to persist state: {}", e);
            }

            let ep_content = format!("Practiced skill '{}' with difficulty {:.1}", skill, difficulty);
            let ep_outcome = if success { "success" } else { "failure (amplified lesson)" };
            let episode = Episode::new("learn_cmd", &ep_content, ep_outcome, difficulty / 10.0);
            db.record_episode(&episode).ok();

            println!("Learned '{}' (difficulty: {:.1}, success: {}):", skill, difficulty, success);
            println!("  XP Gained:  +{:.1}", outcome.xp_gained);
            println!("  Current XP: {:.1} / {:.1}", state.level_engine.xp, LevelEngine::xp_threshold_for_level(state.level_engine.level));
            println!("  New Level:  {}", outcome.new_level);

            if outcome.leveled_up {
                println!("  [LEVEL UP!] Unlocked capabilities:");
                for cap in &outcome.unlocked_capabilities {
                    println!("    • {}", cap);
                }

                let lesson_principle = if !success {
                    format!("Calculated resilience in {}: failure accelerates tactical mastery", skill)
                } else {
                    format!("Precision execution in {}: consistency solidifies dominance", skill)
                };
                let lesson = Lesson::new(&lesson_principle, &format!("Forged during Level {} promotion", outcome.new_level));
                db.record_lesson(&lesson).ok();
                println!("  [DISTILLED LESSON] Logged: \"{}\"", lesson_principle);
            }
        }
        Some(Commands::Memory { limit }) => {
            let episodes = db.get_recent_episodes(limit).unwrap_or_default();
            let lessons = db.get_lessons(limit).unwrap_or_default();

            println!("==================================================");
            println!("  FAIQ | Memory & Distilled Lessons");
            println!("==================================================");
            println!("  DISTILLED LESSONS (Top {}):", lessons.len());
            for l in &lessons {
                println!("  • Principle: \"{}\"", l.principle);
                println!("    Context:   {}", l.context);
                println!("    Relevance: {}x | Date: {}", l.relevance_count, l.timestamp.format("%Y-%m-%d %H:%M"));
            }
            println!("--------------------------------------------------");
            println!("  RECENT EPISODES (Last {}):", episodes.len());
            for ep in &episodes {
                println!("  [{}] [{}] (Outcome: {}):", ep.timestamp.format("%Y-%m-%d %H:%M"), ep.source, ep.outcome);
                println!("    {}", ep.content);
            }
            println!("==================================================");
        }
        Some(Commands::Reflect { window }) => {
            let reflection_engine = ReflectionEngine::new();
            println!("Executing autonomous reflection cycle over last {} events...", window);
            match reflection_engine.reflect(&mut state, &db, window).await {
                Ok(summary) => {
                    println!("==================================================");
                    println!("  FAIQ | Reflection & Lesson Distillation");
                    println!("==================================================");
                    println!("  Events Analyzed:     {}", summary.episodes_analyzed);
                    println!("  Distilled Principle: {}", summary.distilled_principle);
                    println!("  Strategic Insight:   {}", summary.strategic_insight);
                    println!("  XP Awarded:          +{:.1}", summary.xp_awarded);
                    println!("  Level:               {}", summary.new_level);
                    if summary.leveled_up {
                        println!("  [LEVEL UP!] Unlocked higher tier autonomy!");
                    }
                    println!("==================================================");
                }
                Err(e) => {
                    eprintln!("Reflection failed: {}", e);
                }
            }
        }
        Some(Commands::Exec { command, confirm }) => {
            println!("Executing under Faiq supervision: '{}'", command);
            match ToolHarness::execute(&command, confirm, Some(&db)) {
                Ok(res) => {
                    println!("Status: {}", if res.success { "SUCCESS" } else { "FAILED" });
                    println!("Output:\n{}", res.output.trim());

                    let difficulty = if res.success { 2.0 } else { 3.0 };
                    let outcome = state.experience("tool_execution", difficulty, res.success);
                    db.save_state(&state).ok();
                    println!("Gained +{:.1} XP in 'tool_execution' (Current XP: {:.1})", outcome.xp_gained, state.level_engine.xp);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Some(Commands::Goal { action }) => match action {
            GoalCommands::List => {
                let goals = db.get_active_goals().unwrap_or_default();
                println!("Active Tactical Goals ({}):", goals.len());
                for g in goals {
                    println!("  [{}] {} (Diff: {:.1}) - {}", g.id, g.title, g.difficulty, g.description);
                }
            }
            GoalCommands::Add { title, description, difficulty } => {
                let id = format!("g-{}", Utc::now().timestamp_millis() % 10000);
                let goal = TacticalGoal::new(&id, &title, &description, difficulty);
                if let Err(e) = db.save_goal(&goal) {
                    eprintln!("Failed to save goal: {}", e);
                } else {
                    println!("Tactical goal added: [{}] {} (Difficulty: {:.1})", id, title, difficulty);
                    let ep = Episode::new("goal_add", &format!("Added goal '{}'", title), "active", difficulty / 10.0);
                    db.record_episode(&ep).ok();
                }
            }
            GoalCommands::Done { id } => {
                let goals = db.get_active_goals().unwrap_or_default();
                if let Some(mut goal) = goals.into_iter().find(|g| g.id == id) {
                    goal.mark_completed();
                    db.save_goal(&goal).ok();

                    let outcome = state.experience(&format!("goal_{}", goal.title), goal.difficulty, true);
                    db.save_state(&state).ok();

                    let ep = Episode::new("goal_completed", &format!("Completed goal '{}'", goal.title), "completed", 1.0);
                    db.record_episode(&ep).ok();

                    println!("Goal [{}] completed! Gained +{:.1} XP (Level: {})", id, outcome.xp_gained, state.level_engine.level);
                } else {
                    println!("No active goal found with id '{}'", id);
                }
            }
        },
    }
}
