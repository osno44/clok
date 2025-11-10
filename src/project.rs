use std::fs;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::metadata::get_cargo_root;

const CLOK_FILE: &str = ".clok.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    title: String,
    pub sessions: Option<Vec<Session>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            started_at: Utc::now(),
            finished_at: None,
        }
    }
}

impl Project {
    pub fn new(title: String) -> Self {
        Self {
            title,
            sessions: None,
        }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_root = get_cargo_root()?;
        let clok_file = cargo_root.join(".clok.json");
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(clok_file, json_data)?;
        Ok(())
    }
}

pub fn is_already_initialized() -> Result<bool, Box<dyn std::error::Error>> {
    let cargo_root = get_cargo_root()?;
    Ok(cargo_root.join(CLOK_FILE).exists())
}

pub fn load_project() -> Result<Project, Box<dyn std::error::Error>> {
    let cargo_root = get_cargo_root()?;
    let clok_file = cargo_root.join(".clok.json");

    if clok_file.exists() {
        let json_data = fs::read_to_string(clok_file)?;
        let project: Project = serde_json::from_str(&json_data)?;
        Ok(project)
    } else {
        Err("Project not initialized. Run 'clok init' first.".into())
    }
}
