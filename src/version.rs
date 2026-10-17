use std::process::Command;

use reqwest::{Error, blocking::Client};
use serde_json::Value;

pub fn get_current_version() -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");

    let version_output = String::from_utf8_lossy(&output.stdout);

    version_output
        .split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

pub fn get_latest_version() -> Result<String, Error> {
    let client = Client::builder()
        .user_agent("RustVersionChecker/1.0")
        .build()?;

    let url = "https://api.github.com/repos/rust-lang/rust/releases/latest";

    let release: Value = client.get(url).send()?.json()?;

    let tag_name = release["tag_name"].as_str().ok_or("Missing tag_name").unwrap_or("unknown");

    Ok(tag_name.to_string())
}
