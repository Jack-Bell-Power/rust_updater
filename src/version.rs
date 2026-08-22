use std::process::Command;

use gpui::http_client::github::GithubRelease;
use reqwest::{Error, blocking::Client};

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

    let release: GithubRelease = client.get(url).send()?.json()?;

    Ok(release.tag_name)
}
