#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

pub fn find_or_start_kitty(socket_path: &str) -> Result<KittyWindowInfo, String> {
    let output = std::process::Command::new("kitty")
        .args(["@", "--to", socket_path, "ls"])
        .output()
        .expect("failed to execute kitty ls");

    if output.status.success() {
        let data: Result<Vec<KittyWindowInfo>, _> =
            serde_json::from_str(String::from_utf8_lossy(&output.stdout).as_ref());

        match data {
            Ok(d) if d.len() == 1 => return Ok(d[0].clone()),
            Ok(_) => launch_tab(&socket_path, "hrllo")?,
            Err(e) => return Err(e.to_string()),
        }
    } else {
        if which::which("setsid").is_ok() {
            std::process::Command::new("setsid")
                .args(&["kitty", "--listen-on", socket_path])
                .arg("-o")
                .arg("allow_remote_control=yes")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("failed to launch kitty");
        } else {
            std::process::Command::new("open")
                .args([
                    "-na",
                    "kitty",
                    "--args",
                    "--listen-on",
                    socket_path,
                    "-o",
                    "allow_remote_control=yes",
                ])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("failed to launch kitty");
        }
    }

    for _ in 0..100 {
        let output = std::process::Command::new("kitty")
            .args(["@", "--to", socket_path, "ls"])
            .output()
            .expect("failed to execute kitty ls");

        if output.status.success() {
            let data: Result<Vec<KittyWindowInfo>, _> =
                serde_json::from_str(String::from_utf8_lossy(&output.stdout).as_ref());

            match data {
                Ok(d) if d.len() == 1 => return Ok(d[0].clone()),
                Ok(d) => return Err(format!("unexpected window length: {}", d.len())),
                Err(e) => return Err(e.to_string()),
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    Err(format!("error starting kitty"))
}

pub fn launch_tab(socket_path: &str, title: &str) -> Result<(), String> {
    match std::process::Command::new("kitten")
        .args(&[
            "@",
            "launch",
            "--to",
            socket_path,
            "--type=tab",
            "--tab-title",
            title,
        ])
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn focus_tab(socket_path: &str, title: &str) -> Result<(), String> {
    match std::process::Command::new("kitten")
        .args(&[
            "@",
            "focus-tab",
            "--to",
            socket_path,
            "--match",
            format!("title:{}", title).as_str(),
        ])
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn send_text(socket_path: &str, text: &str) -> Result<(), String> {
    match std::process::Command::new("kitten")
        .args(&[
            "@",
            "send-text",
            "--to",
            socket_path,
            &format!("{}\\n", text),
        ])
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KittyWindowInfo {
    pub background_opacity: f64,
    pub id: u64,
    pub is_active: bool,
    pub is_focused: bool,
    pub last_focused: bool,
    pub platform_window_id: u64,
    pub tabs: Vec<KittyTab>,
    pub wm_class: String,
    pub wm_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KittyTab {
    pub active_window_history: Vec<u64>,
    pub enabled_layouts: Vec<String>,
    pub groups: Vec<KittyGroup>,
    pub id: u64,
    pub is_active: bool,
    pub is_focused: bool,
    pub layout: String,
    pub layout_opts: HashMap<String, serde_json::Value>,
    pub layout_state: serde_json::Value,
    pub title: String,
    pub windows: Vec<KittyInnerWindow>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KittyGroup {
    pub id: u64,
    pub windows: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KittyInnerWindow {
    pub at_prompt: bool,
    pub cmdline: Vec<String>,
    pub columns: u64,
    pub created_at: i64,
    pub cwd: String,
    // pub env: HashMap<String, String>,
    pub foreground_processes: Vec<KittyProcess>,
    pub id: u64,
    pub is_active: bool,
    pub is_focused: bool,
    pub is_self: bool,
    pub last_cmd_exit_status: i32,
    pub last_reported_cmdline: String,
    pub lines: u64,
    pub pid: i64,
    pub title: String,
    pub user_vars: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KittyProcess {
    pub cmdline: Vec<String>,
    pub cwd: String,
    pub pid: i64,
}
