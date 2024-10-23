use anyhow::Result;
use clap::Parser;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum ArgumentError {
    #[error("Invalid RO OARA Root: {0}")]
    InvalidROOARARoot(String),
    #[error("Invalid RW OARA Root: {0}")]
    InvalidRWOARARoot(String),
    #[error("Invalid OARA Config: {0}")]
    InvalidOARAConfig(String),
}

#[derive(Parser, Debug)]
#[command(name = "EM", version = "1.0", about = "Execution management")]
pub struct EMArgument {
    #[arg(long, default_value = "/usr/bin/oara", help = "read-only root path")]
    pub ro_oara_root: String,
    #[arg(long, default_value = "", help = "r/w root path like /opt/oara")]
    pub rw_oara_root: String,
    #[arg(short, long, default_value = "/etc/oara", help = "configuration path")]
    pub config: String,
}

pub fn parse() -> Result<EMArgument> {
    let arg = EMArgument::parse();

    let path = Path::new(arg.ro_oara_root.as_str());
    if !path.exists() {
        return Err(ArgumentError::InvalidROOARARoot(arg.ro_oara_root).into());
    }
    if !arg.rw_oara_root.is_empty() {
        let path = Path::new(arg.rw_oara_root.as_str());
        if !path.exists() {
            return Err(ArgumentError::InvalidRWOARARoot(arg.rw_oara_root).into());
        }
    }
    let path = Path::new(arg.config.as_str());
    if !path.exists() {
        return Err(ArgumentError::InvalidOARAConfig(arg.config).into());
    }

    Ok(arg)
}
