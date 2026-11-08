use std::process::Command;
use crate::storage::{Database, Episode};

/// Result of a tool execution.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub command: String,
}

/// Tool execution harness with strict guardrails to prevent destructive actions without confirmation.
pub struct ToolHarness;

impl ToolHarness {
    /// Detects whether a command is destructive and requires explicit owner confirmation.
    pub fn is_destructive(cmd: &str) -> bool {
        let lower = cmd.to_lowercase();
        let dangerous_patterns = [
            "del /f /s /q",
            "rmdir /s /q",
            "rm -rf",
            "format ",
            "drop table",
            "drop database",
            "truncate table",
            "diskpart",
            "reg delete",
        ];

        dangerous_patterns.iter().any(|pattern| lower.contains(pattern))
    }

    /// Executes a system command in the local shell and records the outcome in episodic memory.
    pub fn execute(cmd: &str, confirmed: bool, db: Option<&Database>) -> Result<ToolResult, String> {
        if Self::is_destructive(cmd) && !confirmed {
            let err_msg = "BLOCKED BY GUARDRAIL: Destructive command detected. Requires explicit owner confirmation."
                .to_string();
            if let Some(database) = db {
                let ep = Episode::new("tool_guardrail", cmd, "blocked", 0.8);
                database.record_episode(&ep).ok();
            }
            return Err(err_msg);
        }

        #[cfg(target_os = "windows")]
        let output_res = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(cmd)
            .output();

        #[cfg(not(target_os = "windows"))]
        let output_res = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output();

        let (success, output_str) = match output_res {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let combined = if stderr.is_empty() {
                    stdout
                } else if stdout.is_empty() {
                    stderr
                } else {
                    format!("{}\n[STDERR]: {}", stdout, stderr)
                };
                (output.status.success(), combined)
            }
            Err(e) => (false, format!("Failed to execute process: {}", e)),
        };

        if let Some(database) = db {
            let outcome_label = if success { "success" } else { "failure" };
            let ep = Episode::new("tool_execution", cmd, outcome_label, 0.6);
            database.record_episode(&ep).ok();
        }

        Ok(ToolResult {
            success,
            output: output_str,
            command: cmd.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guardrails_detect_destructive_commands() {
        assert!(ToolHarness::is_destructive("rm -rf /"));
        assert!(ToolHarness::is_destructive("rmdir /s /q C:\\"));
        assert!(ToolHarness::is_destructive("del /f /s /q *.*"));
        assert!(!ToolHarness::is_destructive("cargo test"));
        assert!(!ToolHarness::is_destructive("Get-ChildItem"));
    }

    #[test]
    fn test_execution_blocks_destructive_without_confirmation() {
        let res = ToolHarness::execute("rm -rf /test", false, None);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("BLOCKED BY GUARDRAIL"));
    }

    #[test]
    fn test_benign_command_execution() {
        let res = ToolHarness::execute("echo test_output", false, None).unwrap();
        assert!(res.success);
        assert!(res.output.contains("test_output"));
    }
}

