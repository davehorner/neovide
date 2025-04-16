// src/bin/a_neovide_tui.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, ExitCode},
};
use tempfile::tempdir;
use neovide_core::run;

static NV_INIT_LUA: &str = include_str!("../config/a_neovide_tui_init.lua");

fn main() -> ExitCode {
    // Override the window title
    env::set_var("NEOVIDE_WINDOW_TITLE", "🔮 Blink.CMP Demo 🔮");

    // Create a single temp dir for *both* config & data
    let tmp = tempdir().expect("failed to create tempdir");

    // ─── 1) XDG_CONFIG_HOME → tmp/nvim ─────────────────────────────
    let cfg = tmp.path().join("nvim");
    fs::create_dir_all(&cfg).expect("mkdir nvim config dir");
    fs::write(cfg.join("init.lua"), NV_INIT_LUA).expect("write init.lua");
    env::set_var("XDG_CONFIG_HOME", tmp.path());

    // ─── 2) XDG_DATA_HOME → tmp.path() ─────────────────────────────
    // so that stdpath('data') == tmp.path()/nvim-data on Windows
    let data_home = tmp.path();
    env::set_var("XDG_DATA_HOME", &data_home);

    // Make sure Neovim’s data dir (stdpath('data')) exists:
    let std_data = data_home.join("nvim-data");
    fs::create_dir_all(&std_data).expect("mkdir std data dir");

    // ─── 3) Clone blink.cmp into stdpath('data')/blink.cmp ─────────
    let blink_dir = std_data.join("blink.cmp");
    if !blink_dir.exists() {
        Command::new("git")
            .args(&[
                "clone",
                "--depth", "1",
                "https://github.com/Saghen/blink.cmp.git",
                blink_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to clone blink.cmp");
    }

    // ─── 4) Make sure the Neovide config dir exists ───────────────
    let neovide_cfg_dir: PathBuf = tmp.path().join("neovide");
    fs::create_dir_all(&neovide_cfg_dir).expect("failed to create neovide config dir");

    // ─── 5) Create an (empty) settings.toml so the watcher has something to watch
    let settings_file = neovide_cfg_dir.join("settings.toml");
    if !settings_file.exists() {
        fs::File::create(&settings_file).expect("failed to create neovide settings file");
    }

    // Launch Neovide core
    run()
}
