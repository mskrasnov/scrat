//! Simple logging

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "log")]
    pub logs: Vec<LogEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    pub log_type: LogType,
    pub ctime: i64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum LogType {
    Debug,
    #[default]
    Info,
    Warning,
    Error,
    CriticalError,
}

impl Log {
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let contents = fs::read_to_string(&pth)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()> {
        let contents = toml::to_string(&self)?;
        fs::write(&pth, contents)?;

        Ok(())
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        self.logs.push(entry);
    }
}
