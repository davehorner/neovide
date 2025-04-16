// examples/a_neovide_blink_cmp.rs
// -- dave horner 4/25 MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use neovide_core::run;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, ExitCode},
};
use tempfile::tempdir;

static NV_INIT_LUA: &str = include_str!("a_neovide_blink_cmp.lua");

fn main() -> ExitCode {
    // Determine NVIM_APPNAME from the executable name
    let exe_path = env::args().next().unwrap_or_else(|| "nvim".into());
    let binding = PathBuf::from(&exe_path);
    let app_name = binding
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("nvim");
    env::set_var("NVIM_APPNAME", app_name);

    // Override the window title
    env::set_var("NEOVIDE_WINDOW_TITLE", "🔮 Blink.CMP Demo 🔮");

    // Create a single temp dir for *both* config & data
    let tmp = tempdir().expect("failed to create tempdir");

    // 1) XDG_CONFIG_HOME → tmp/a_neovide_tui
    let cfg = tmp.path().join(app_name);
    fs::create_dir_all(&cfg).expect("mkdir config dir");
    fs::write(cfg.join("init.lua"), NV_INIT_LUA).expect("write init.lua");
    env::set_var("XDG_CONFIG_HOME", tmp.path());

    // 2) XDG_DATA_HOME → tmp/<app_name>-data
    let data_home = tmp.path().join(format!("{}-data", app_name));
    fs::create_dir_all(&data_home).expect("mkdir data dir");
    #[cfg(windows)]
    env::set_var("XDG_DATA_HOME", &tmp.path());
    #[cfg(not(windows))]
    env::set_var("XDG_DATA_HOME", &data_home);

    // 3) Neovim’s stdpath('data') will now be either:
    //    - on Windows: XDG_DATA_HOME/<app_name>-data  (so stdpath('data') == data_home)
    //    - on non-Windows: XDG_DATA_HOME/<app_name>-data/<app_name> (so stdpath('data') == data_home/app_name)
    #[cfg(windows)]
    let std_data = data_home.clone();

    #[cfg(not(windows))]
    let std_data = data_home.join(app_name);

    fs::create_dir_all(&std_data).expect("mkdir stdpath data dir");

    let blink_dir = std_data.join("blink.cmp");
    if !blink_dir.exists() {
        Command::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                "https://github.com/Saghen/blink.cmp.git",
                blink_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to clone blink.cmp");
    }

    // 3) Make sure the Neovide config dir exists
    let neovide_cfg_dir: PathBuf = tmp.path().join("neovide");
    fs::create_dir_all(&neovide_cfg_dir).expect("failed to create neovide config dir");

    // 4) Create an (empty) settings.toml so the watcher has something to watch
    let settings_file = neovide_cfg_dir.join("settings.toml");
    if !settings_file.exists() {
        fs::File::create(&settings_file).expect("failed to create neovide settings file");
    }

    // Launch Neovide core
    let ret = run();
        // now read the selection file
    let sel_path = std_data.join("selection.txt");
    if let Ok(contents) = fs::read_to_string(&sel_path) {
        let choice = contents.trim();
        if !choice.is_empty() {
            println!("You selected: {}", choice);
        }
    }
    ret
}
