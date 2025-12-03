use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

mod gui;
mod rad;
mod unit;

#[derive(clap::Args, Clone, Debug)]
pub struct Config {
    #[arg(short, long, default_value_t = 0.9995)]
    pub source_decay_rate: f64,
    #[arg(short, long, default_value_t = 0.9995)]
    pub target_decay_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source_decay_rate: 0.9995,
            target_decay_rate: 0.9995,
        }
    }
}

// TODO: args
#[derive(clap::Parser, Clone, Debug, Default)]
pub struct Args {
    #[command(flatten)]
    pub cfg: Config,
}

fn main() -> eframe::Result {
    LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().without_time())
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .with(tracing_tracy::TracyLayer::default()),
    )
    .expect("setup tracy layer");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mekanism Radiation Previewer",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::RadiationApp::new(cc)))),
    )
}
