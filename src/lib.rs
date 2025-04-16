#[macro_use]
extern crate neovide_derive;

#[cfg(target_os = "windows")]
#[cfg(test)]
#[macro_use]
extern crate approx;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate derive_new;

mod bridge;
mod channel_utils;
mod clipboard;
mod cmd_line;
mod dimensions;
mod editor;
mod error_handling;
mod frame;
mod profiling;
mod renderer;
mod running_tracker;
mod settings;
mod units;
mod utils;
mod window;

#[cfg(target_os = "windows")]
mod windows_utils;

use std::{
    env::args,
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    panic::set_hook,
    process::ExitCode,
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use log::trace;
use std::env::var;
use std::env;
use std::panic::PanicHookInfo;
use std::path::PathBuf;
use time::macros::format_description;
use time::OffsetDateTime;
use winit::{error::EventLoopError, event_loop::EventLoopProxy};

#[cfg(not(test))]
use flexi_logger::{Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};

use backtrace::Backtrace;
use bridge::NeovimRuntime;
use cmd_line::CmdLineSettings;
use error_handling::handle_startup_errors;
use renderer::{cursor_renderer::CursorSettings, RendererSettings};
use running_tracker::RunningTracker;
use window::{
    create_event_loop, determine_window_size, UpdateLoop, UserEvent, WindowSettings, WindowSize,
};

pub use channel_utils::*;
#[cfg(target_os = "windows")]
pub use windows_utils::*;

use crate::settings::{load_last_window_settings, Config, PersistentWindowSettings, Settings};

pub use profiling::startup_profiler;

const DEFAULT_BACKTRACES_FILE: &str = "neovide_backtraces.log";
const BACKTRACES_FILE_ENV_VAR: &str = "NEOVIDE_BACKTRACES";
const REQUEST_MESSAGE: &str =
    "This is a bug and we would love for it to be reported to https://github.com/neovide/neovide/issues";

/// The one‑and‑only entry point for library consumers.
pub fn run() -> ExitCode {
    set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::new();

        let stderr_msg = generate_stderr_log_message(panic_info, &backtrace);
        eprintln!("{stderr_msg}");

        log_panic_to_file(panic_info, &backtrace, &None);
    }));

    #[cfg(target_os = "windows")]
    {
        windows_fix_dpi();
    }

    #[cfg(target_os = "linux")]
    env::remove_var("ARGV0");

    let event_loop = create_event_loop();
    clipboard::init(&event_loop);

    let running_tracker = RunningTracker::new();
    let settings = Arc::new(Settings::new());

    match setup(
        event_loop.create_proxy(),
        running_tracker.clone(),
        settings.clone(),
    ) {
        Err(err) => handle_startup_errors(err, event_loop, settings.clone()),
        Ok((window_size, initial_config, runtime)) => {
            let mut update_loop = UpdateLoop::new(
                window_size,
                initial_config,
                event_loop.create_proxy(),
                settings.clone(),
            );

            let result = event_loop.run_app(&mut update_loop);

            runtime.runtime.shutdown_timeout(Duration::from_millis(500));

            match result {
                Ok(_) => running_tracker.exit_code(),
                Err(EventLoopError::ExitFailure(code)) => ExitCode::from(code as u8),
                _ => ExitCode::FAILURE,
            }
        }
    }
}

fn setup(
    proxy: EventLoopProxy<UserEvent>,
    running_tracker: RunningTracker,
    settings: Arc<Settings>,
) -> Result<(WindowSize, Config, NeovimRuntime)> {
    settings.register::<WindowSettings>();
    settings.register::<RendererSettings>();
    settings.register::<CursorSettings>();

    let config = Config::init();
    Config::watch_config_file(config.clone(), proxy.clone());

    set_hook(Box::new({
        let path = config.backtraces_path.clone();
        move |panic_info: &PanicHookInfo<'_>| {
            let backtrace = Backtrace::new();

            let stderr_msg = generate_stderr_log_message(panic_info, &backtrace);
            eprintln!("{stderr_msg}");

            log_panic_to_file(panic_info, &backtrace, &path);
        }
    }));

    cmd_line::handle_command_line_arguments(args().collect(), settings.as_ref())?;
    #[cfg(not(target_os = "windows"))]
    maybe_disown(&settings);

    startup_profiler();

    #[cfg(not(test))]
    init_logger(&settings);

    trace!("Neovide version: {}", crate_version!());

    let window_settings = load_last_window_settings().ok();
    let window_size = determine_window_size(window_settings.as_ref(), &settings);
    let grid_size = match window_size {
        WindowSize::Grid(grid_size) => Some(grid_size),
        #[allow(clippy::manual_unwrap_or_default)]
        _ => match window_settings {
            Some(PersistentWindowSettings::Maximized { grid_size, .. }) => grid_size,
            Some(PersistentWindowSettings::Windowed { grid_size, .. }) => grid_size,
            _ => None,
        },
    };

    let mut runtime = NeovimRuntime::new()?;
    runtime.launch(proxy, grid_size, running_tracker, settings)?;
    Ok((window_size, config, runtime))
}

#[cfg(not(test))]
pub fn init_logger(settings: &Settings) {
    let cmdline_settings = settings.get::<CmdLineSettings>();

    let logger = if cmdline_settings.log_to_file {
        Logger::try_with_env_or_str("neovide")
            .expect("Could not init logger")
            .log_to_file(FileSpec::default())
            .rotate(
                Criterion::Size(10_000_000),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(1),
            )
            .duplicate_to_stderr(Duplicate::Error)
    } else {
        Logger::try_with_env_or_str("neovide = error").expect("Could not init logger")
    };

    logger.start().expect("Could not start logger");
}

#[cfg(not(target_os = "windows"))]
fn maybe_disown(settings: &Settings) {
    use std::process;

    let cmdline_settings = settings.get::<CmdLineSettings>();

    if !cmdline_settings.fork || !utils::is_tty() {
        return;
    }

    if let Ok(current_exe) = env::current_exe() {
        assert!(process::Command::new(current_exe)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .args(env::args().skip(1))
            .spawn()
            .is_ok());
        process::exit(0);
    } else {
        eprintln!(
            "error in disowning process, cannot obtain the path for the current executable, continuing without disowning..."
        );
    }
}

fn generate_stderr_log_message(panic_info: &PanicHookInfo, backtrace: &Backtrace) -> String {
    if cfg!(debug_assertions) {
        let print_backtrace = match var("RUST_BACKTRACE") {
            Ok(x) => x == "full" || x == "1",
            Err(_) => false,
        };

        let backtrace_msg = if print_backtrace {
            format!("{backtrace:?}")
        } else {
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
                .to_owned()
        };

        let panic_msg = generate_panic_message(panic_info);

        format!("{panic_msg}\n{REQUEST_MESSAGE}\n{backtrace_msg}")
    } else {
        let panic_msg = generate_panic_message(panic_info);
        format!("{panic_msg}\n{REQUEST_MESSAGE}")
    }
}

fn log_panic_to_file(panic_info: &PanicHookInfo, backtrace: &Backtrace, path: &Option<PathBuf>) {
    let log_msg = generate_panic_log_message(panic_info, backtrace);

    let file_path = match path {
        Some(v) => v,
        None => &match var(BACKTRACES_FILE_ENV_VAR) {
            Ok(v) => PathBuf::from(v),
            Err(_) => settings::neovide_std_datapath().join(DEFAULT_BACKTRACES_FILE),
        },
    };

    if let Some(parent) = file_path.parent() {
        create_dir_all(parent).ok();
    }

    let mut file = match OpenOptions::new()
        .append(true)
        .open(file_path)
        .or_else(|_| File::create(file_path))
    {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not create backtraces file. ({e})");
            return;
        }
    };

    if let Err(e) = file.write_all(log_msg.as_bytes()) {
        eprintln!("Failed writing panic to {:?}: {e}", file_path);
    } else {
        eprintln!("\nBacktrace saved to {:?}!", file_path);
    }
}

fn generate_panic_log_message(panic_info: &PanicHookInfo, backtrace: &Backtrace) -> String {
    let system_time: OffsetDateTime = SystemTime::now().into();
    let timestamp = system_time
        .format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .expect("Failed to parse current time");
    let partial = generate_panic_message(panic_info);
    format!("{timestamp} - {partial}\n{backtrace:?}\n")
}

fn generate_panic_message(panic_info: &PanicHookInfo) -> String {
    let loc = panic_info.location().unwrap();
    let file = loc.file();
    let line = loc.line();
    let col = loc.column();

    let payload = panic_info
        .payload()
        .downcast_ref::<&str>()
        .map(ToOwned::to_owned)
        .or_else(|| {
            panic_info
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str)
        })
        .unwrap_or_else(|| "Could not parse panic payload".into());

    format!(
        "Neovide panicked with the message '{payload}'. (File: {file}; Line: {line}, Column: {col})"
    )
}
