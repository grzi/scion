
use log::LevelFilter;

use scion::config::logger_config::LoggerConfig;
use scion::config::scion_config::{ScionConfig, ScionConfigBuilder};
use scion::config::window_config::WindowConfigBuilder;
use scion::graphics::components::color::Color;
use scion::Scion;
use crate::scene::DemoScene;

mod scene;

fn main() {
    Scion::app_with_config(create_config())
        .with_scene::<DemoScene>()
        .run();
}


fn create_config() -> ScionConfig {
    ScionConfigBuilder::new()
        .with_logger_config(LoggerConfig{
            scion_level_filter: LevelFilter::Debug,
            level_filter: LevelFilter::Debug })
        .with_app_name("Showcase - Scion".to_string())
        .with_window_config(
            WindowConfigBuilder::new()
                .with_resizable(false)
                .with_dimensions((1024, 768))
                .with_default_background_color(Some(Color::new_rgb(0,0,0)))
                .get())
        .get()
}