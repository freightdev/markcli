// source: src/libs/marker.rs
// src/libs/setup.rs

use std::fs::{self, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use walkdir::WalkDir;

pub async fn run(dir: &str) {
    println!("\n🔍 --marker: Syncing .mstp paths and checking .marks/.md...");

    let base = Path::new(dir).join(".mark");
    let mut updated = 0;

    // Parse master path
    let mstp_path = base.join("mark.mstp");
    let Ok(lines) = fs::read_to_string(&mstp_path) else {
        println!("❌ Could not read mark.mstp");
        return;
    };

    let paths: Vec<PathBuf> = lines
        .lines()
        .filter(|l| l.trim().starts_with("- "))
        .map(|l| base.join(l.trim().trim_start_matches("- ").trim()))
        .collect();

    let mut log = String::new();

    for path in paths {
        if path.ends_with("agent.marks") {
            log.push_str(&check_agents(&path));
            updated += 1;
        } else if path.ends_with("tool.marks") {
            log.push_str(&check_tools(&path));
            updated += 1;
        }
    }

    let cache = base.join("cache");
    create_dir_all(&cache).unwrap();
    fs::write(cache.join("sync.log"), log).unwrap();

    println!("✅ Marker sync complete. {} paths walked.\n📁 Log written to .mark/cache/sync.log\n", updated);
}

fn check_agents(agent_mstp: &Path) -> String {
    let mut log = String::from("\n--- Agent Sync ---\n");
    if let Ok(data) = fs::read_to_string(agent_mstp) {
        for line in data.lines().filter(|l| l.trim().starts_with("- ")) {
            let path = line.trim().trim_start_matches("- ").trim();
            let full = agent_mstp.parent().unwrap().join(path);

            let agent_dir = full.parent().unwrap();
            let agent_name = agent_dir.file_name().unwrap().to_str().unwrap();

            // Ensure markers.* and md.* exist
            let markers = agent_dir.join(format!("markers.{}", agent_name));
            let md = agent_dir.join(format!("md.{}", agent_name));

            if !markers.exists() {
                fs::write(&markers, format!("# Marker: AutoGenerated\n\n### Trigger\nUnknown\n\n### Effect\nUnspecified\n")).unwrap();
                log.push_str(&format!("🧠 Created: {}\n", markers.display()));
            }
            if !md.exists() {
                fs::write(&md, format!("# Agent: {}\n\n## Capabilities\n- Unknown\n\n## Ethics\n- TBD\n", agent_name)).unwrap();
                log.push_str(&format!("🧬 Created: {}\n", md.display()));
            }
        }
    }
    log
}

fn check_tools(tool_mstp: &Path) -> String {
    let mut log = String::from("\n--- Tool Sync ---\n");
    if let Ok(data) = fs::read_to_string(tool_mstp) {
        for line in data.lines().filter(|l| l.trim().starts_with("- ")) {
            let path = line.trim().trim_start_matches("- ").trim();
            let full = tool_mstp.parent().unwrap().join(path);

            let tool_dir = full.parent().unwrap();
            let tool_name = tool_dir.file_name().unwrap().to_str().unwrap();

            let markers = tool_dir.join(format!("markers.{}", tool_name));
            let md = tool_dir.join(format!("md.{}", tool_name));

            if !markers.exists() {
                fs::write(&markers, format!("# Marker: AutoGenerated\n\n## Trigger\nIdle\n\n## Effect\nTool Activation\n")).unwrap();
                log.push_str(&format!("🔖 Created: {}\n", markers.display()));
            }
            if !md.exists() {
                fs::write(&md, format!("# Tool: {}\n\n## Intent\nDescribe use here\n\n## Example Output\nExample\n", tool_name)).unwrap();
                log.push_str(&format!("🛠️ Created: {}\n", md.display()));
            }
        }
    }
    log
}