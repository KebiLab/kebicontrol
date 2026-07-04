//! Entry point. Made by KebiLab

mod state;

use crate::state::AppState;
use anyhow::Result;
use clap::Parser;
use kebi_core::{AppPaths, Config, Profile};
use kebi_llm::LlmClient;
use single_instance::SingleInstance;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "KebiControl", author = "KebiLab", version, about = "Voice control for Windows. Made by KebiLab.")]
struct Args {
    #[arg(long)]
    hidden: bool,
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
    if let Some(p) = args.config {
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

    let api_key = config.get_api_key().unwrap_or_default();
    let llm = Arc::new(Mutex::new(LlmClient::new(
        config.llm.base_url.clone(),
        api_key,
        config.llm.model.clone(),
        config.llm.timeout_secs,
    )));

    let state = Arc::new(AppState::new(config, profile, paths, llm));
    let _ = args.hidden;

    eframe::run_native(
        "KebiControl",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([640.0, 480.0])
                .with_transparent(true)
                .with_decorations(true),
            ..Default::default()
        },
        Box::new(move |cc| {
            theme_install(cc.egui_ctx.clone());
            let overlay = kebi_ui::OverlayApp::new(state.config().general.wake_word.clone());
            Box::new(overlay)
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe: {e}"))?;

    Ok(())
}

fn theme_install(ctx: eframe::egui::Context) {
    kebi_ui::theme::install(&ctx);
}

fn init_tracing(paths: &AppPaths) {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let file_appender = tracing_appender::rolling::daily(&paths.logs_dir, "kebicontrol.log");
    let (nb, _g) = tracing_appender::non_blocking(file_appender);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_writer(nb))
        .with(fmt::layer().with_target(false))
        .try_init();
}
