// src/bin/a_neovide_tui.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io::Write, process::ExitCode};
use tempfile::tempdir;
use neovide_core::run;

// -----------------------------------------------------------------------------
// Embed a complete, standalone `init.lua`
// -----------------------------------------------------------------------------
static NV_INIT_LUA: &str = include_str!("../config/a_neovide_tui_init.lua");

fn main() -> ExitCode {
    std::env::set_var("NEOVIDE_WINDOW_TITLE", "My Custom TUI ");//✨");
    // 1) Create a temporary XDG_CONFIG_HOME
    let tmp = tempdir().expect("failed to create tempdir");
    let cfg_dir = tmp.path().join("nvim");
    fs::create_dir_all(&cfg_dir).expect("failed to mkdir nvim config dir");

    // 2) Write our embedded init.lua
    let init_path = cfg_dir.join("init.lua");
    let mut f = fs::File::create(&init_path).expect("failed to create init.lua");
    f.write_all(NV_INIT_LUA.as_bytes())
        .expect("failed to write init.lua");

    // 3) Point Neovim at it (isolated from your real config)
    std::env::set_var("XDG_CONFIG_HOME", tmp.path());

    // 4) Launch the Neovide UI
    run()
}
