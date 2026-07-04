//! Entry point. Made by KebiLab

use anyhow::Result;
use clap::Parser;
use kebi_core::{AppPaths, Config, Profile};
use kebi_llm::LlmClient;
use kebi_ui::{OverlayApp, SettingsApp};
use single_instance::SingleInstance;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "KebiControl", author = "KebiLab", version, about = "Voice control for Windows. Made by KebiLab.")]
struct Args {
    /// Open Settings window instead of the overlay.
    #[arg(long)]
    settings: bool,
    /// Path to a config file.
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

    let _api_key = config.get_api_key().unwrap_or_default();
    let _llm = Arc::new(tokio::sync::Mutex::new(LlmClient::new(
        config.llm.base_url.clone(),
        String::new(),
        config.llm.model.clone(),
        config.llm.timeout_secs,
    )));
    let _profile = profile;
    let _paths = paths;

    eframe::run_native(
        "KebiControl",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([560.0, 560.0])
                .with_min_inner_size([480.0, 420.0])
                .with_transparent(false)
                .with_decorations(true)
                .with_title("KebiControl — Made by KebiLab"),
            ..Default::default()
        },
        Box::new(move |cc| {
            let open_settings = Arc::new(AtomicBool::new(args.settings));
            if args.settings {
                Box::new(SettingsApp::new(config.clone()))
            } else {
                theme_install(cc.egui_ctx.clone());
                let mut app = OverlayApp::new(open_settings.clone());
                app.status = format!("Готов — скажите «{}»", config.general.wake_word);
                Box::new(OverlayHost::new(app, config, open_settings))
            }
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe: {e}"))?;

    Ok(())
}

/// Hosts OverlayApp and opens Settings when the flag is set.
struct OverlayHost {
    overlay: OverlayApp,
    config: Config,
    open_settings_flag: Arc<AtomicBool>,
    settings_open: bool,
}

impl OverlayHost {
    fn new(overlay: OverlayApp, config: Config, flag: Arc<AtomicBool>) -> Self {
        Self { overlay, config, open_settings_flag: flag, settings_open: false }
    }
}

impl eframe::App for OverlayHost {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.open_settings_flag.swap(false, Ordering::SeqCst) {
            self.settings_open = true;
        }
        if self.settings_open {
            let mut settings = SettingsApp::new(self.config.clone());
            let mut close = false;
            egui::Window::new("Настройки — KebiControl")
                .title_bar(true)
                .resizable(true)
                .default_size(egui::Vec2::new(520.0, 620.0))
                .show(ctx, |ui| {
                    // Use a fake App to render the settings body.
                    // We delegate by simulating update via a dummy Frame.
                    settings.update(ctx, _frame);
                    ui.horizontal(|ui| {
                        if ui.button("Закрыть").clicked() { close = true; }
                        if ui.button("Сохранить").clicked() {
                            self.config = settings.config.clone();
                            let _ = self.config.save(&kebi_core::AppPaths::new());
                        }
                    });
                });
            if close { self.settings_open = false; }
            self.config = settings.config;
        }
        self.overlay.update(ctx, _frame);
    }
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
