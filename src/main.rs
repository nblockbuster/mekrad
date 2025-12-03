
mod gui;
mod rad;
mod unit;

#[derive(Clone, Debug)]
pub struct Config {
    pub source_decay_rate: f64,
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

// TODO: command line args?

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use tracing::level_filters::LevelFilter;
    use tracing_log::LogTracer;
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
    
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

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(gui::RadiationApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
