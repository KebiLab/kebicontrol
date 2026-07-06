//! Entry point. Made by KebiLab
#![windows_subsystem = "windows"]

mod voice;

use anyhow::Result;
use clap::Parser;
use kebi_core::{AppPaths, Config, Profile};
use kebi_ui::{MainApp, VoiceController};
use single_instance::SingleInstance;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "KebiControl", author = "KebiLab", version, about = "Voice control for Windows. Made by KebiLab.")]
struct Args {
    #[arg(long)]
    config: Option<String>,
}

fn main() -> Result<()> {
    let _instance = SingleInstance::new("KebiControl-KebiLab")
        .unwrap_or_else(|_| std::process::exit(0));

    let args = Args::parse();
    let paths = AppPaths::new();
    init_tracing(&paths);
    info!("KebiControl starting — Made by KebiLab");

    let mut config = Config::load(&paths).unwrap_or_default();
    if let Some(p) = args.config.clone() {
        if let Ok(c) = kebi_core::config::read_from(std::path::Path::new(&p)) {
            config = c;
        }
    }

    let profile_path = paths.profiles_dir.join(format!("{}.toml", config.general.active_profile));
    let profile = if profile_path.exists() {
        Profile::load_from(&profile_path).unwrap_or_default()
    } else {
        let p = Profile::default();
        let _ = p.save_to(&profile_path);
        p
    };

    // Voice pipeline.
    let controller = Arc::new(VoiceController::new());
    let (tx, rx) = mpsc::unbounded_channel();
    voice::spawn_pipeline(config.clone(), profile, controller.clone(), tx);

    let mut native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([760.0, 600.0])
            .with_min_inner_size([680.0, 540.0])
            .with_transparent(false)
            .with_decorations(true)
            .with_title("KebiControl — Made by KebiLab"),
        ..Default::default()
    };
    if let Some(icon) = kebi_ui::load_icon() {
        native_options.viewport = native_options.viewport.with_icon(icon);
    }
    let cfg_for_app = config.clone();
    eframe::run_native(
        "KebiControl",
        native_options,
        Box::new(move |_cc| {
            Box::new(MainApp::new(cfg_for_app.clone(), controller, rx))
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe: {e}"))?;

    Ok(())
}

fn init_tracing(paths: &AppPaths) {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let file_appender = tracing_appender::rolling::daily(&paths.logs_dir, "kebicontrol.log");
    let (nb, _g) = tracing_appender::non_blocking(file_appender);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_writer(nb).with_ansi(false))
        .try_init();
    std::panic::set_hook(Box::new(|_| {}));
}
