use std::io::{BufRead, BufReader};

#[derive(serde::Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
}

/// Get the directory where log files are stored.
fn log_dir() -> std::path::PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.memorypalace.app")
}

/// Find the latest log file in the log directory.
fn find_latest_log() -> Option<std::path::PathBuf> {
    let dir = log_dir();
    if !dir.exists() {
        return None;
    }

    let mut logs: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("memory_palace.log"))
        })
        .collect();

    logs.sort();
    logs.pop() // latest by name (date-sorted)
}

/// Parse a tracing log line into a LogEntry.
/// Expected format: `2024-03-26T10:30:00.123456Z  INFO target: message`
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Try to find the timestamp (ISO 8601 format at the start)
    let rest = line;

    // Find the level keyword
    let levels = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];
    let mut level = String::new();
    let mut after_level_pos = 0;

    for l in &levels {
        if let Some(pos) = rest.find(l) {
            // Ensure it's a standalone word (preceded by whitespace)
            if pos > 0 && rest.as_bytes().get(pos - 1).map_or(false, |b| b.is_ascii_whitespace()) {
                level = l.to_string();
                after_level_pos = pos + l.len();
                break;
            }
        }
    }

    if level.is_empty() {
        // Can't parse level; return as raw message
        return Some(LogEntry {
            timestamp: String::new(),
            level: "INFO".to_string(),
            target: String::new(),
            message: line.to_string(),
        });
    }

    let timestamp = rest[..after_level_pos]
        .trim()
        .trim_end_matches(&level)
        .trim()
        .to_string();

    let after_level = &rest[after_level_pos..];

    // The target is usually followed by a colon, e.g. " memory_palace_lib::commands::import: message"
    let (target, message) = if let Some(colon_pos) = after_level.find(": ") {
        let t = after_level[..colon_pos].trim().to_string();
        let m = after_level[colon_pos + 2..].trim().to_string();
        (t, m)
    } else {
        (String::new(), after_level.trim().to_string())
    };

    Some(LogEntry {
        timestamp,
        level,
        target,
        message,
    })
}

#[tauri::command]
pub fn get_app_logs(
    limit: Option<usize>,
    level_filter: Option<String>,
) -> Result<Vec<LogEntry>, String> {
    let limit = limit.unwrap_or(200);

    let log_path = find_latest_log().ok_or_else(|| "No log files found".to_string())?;

    let file = std::fs::File::open(&log_path).map_err(|e| format!("Failed to open log file: {}", e))?;
    let reader = BufReader::new(file);

    let level_filter = level_filter.map(|l| l.to_uppercase());

    let mut entries: Vec<LogEntry> = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if let Some(entry) = parse_log_line(&line) {
            if let Some(ref filter) = level_filter {
                if filter != "ALL" && entry.level != *filter {
                    continue;
                }
            }
            entries.push(entry);
        }
    }

    // Return the last N entries
    let start = entries.len().saturating_sub(limit);
    Ok(entries[start..].to_vec())
}

#[tauri::command]
pub fn get_log_path() -> Result<String, String> {
    match find_latest_log() {
        Some(path) => Ok(path.display().to_string()),
        None => {
            let dir = log_dir();
            Ok(dir.join("memory_palace.log").display().to_string())
        }
    }
}

// Derive Clone for LogEntry so we can use to_vec() on slices
impl Clone for LogEntry {
    fn clone(&self) -> Self {
        LogEntry {
            timestamp: self.timestamp.clone(),
            level: self.level.clone(),
            target: self.target.clone(),
            message: self.message.clone(),
        }
    }
}
