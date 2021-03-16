use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerConfig {
    pub(crate) level_filter: LevelFilter,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level_filter: LevelFilter::Info,
        }
    }
}
