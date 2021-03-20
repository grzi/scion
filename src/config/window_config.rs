use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use winit::dpi::Size;
use winit::window::{WindowAttributes, WindowBuilder};

/// Main configuration for the game window
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    /// Window title
    pub(crate) title: String,
    /// Enables fullscreen mode
    pub(crate) fullscreen: bool,
    /// Default window width and height in pixels.
    pub(crate) dimensions: Option<(u32, u32)>,
    /// Minimum window width and height in pixels.
    pub(crate) min_dimensions: Option<(u32, u32)>,
    /// Maximum window width and height in pixels.
    pub(crate) max_dimensions: Option<(u32, u32)>,
    /// Whether to display the window, Use full for loading
    pub(crate) visibility: bool,
    /// The path relative to the game executable of the window icon.
    pub(crate) icon: Option<PathBuf>,
    /// Whether the window should always be on top of other windows.
    pub(crate) always_on_top: bool,
    /// Whether the window should have borders and bars.
    pub(crate) decorations: bool,
    /// Whether the window should be maximized upon creation.
    pub(crate) maximized: bool,
    /// If the user can resize the window
    pub(crate) resizable: bool,
    /// If the window should be able to be transparent.
    pub(crate) transparent: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Default Scion game".to_string(),
            fullscreen: false,
            dimensions: Some((1024, 768)),
            min_dimensions: Some((640, 480)),
            max_dimensions: None,
            visibility: true,
            icon: None,
            always_on_top: false,
            decorations: true,
            maximized: false,
            resizable: true,
            transparent: false,
        }
    }
}

pub struct WindowConfigBuilder {
    config: WindowConfig,
}

impl WindowConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub fn with_dimensions(mut self, dimensions: (u32, u32)) -> Self {
        self.config.dimensions = Some(dimensions);
        self
    }

    pub fn get(self) -> WindowConfig {
        self.config
    }
}

impl Into<WindowBuilder> for WindowConfig {
    fn into(self) -> WindowBuilder {
        let mut builder = WindowBuilder::new();

        builder.window = WindowAttributes {
            title: self.title.clone(),
            fullscreen: None,
            inner_size: self.dimensions.map(|d| d.into()).map(Size::Logical),
            min_inner_size: self.min_dimensions.map(|d| d.into()).map(Size::Logical),
            max_inner_size: self.max_dimensions.map(|d| d.into()).map(Size::Logical),
            visible: self.visibility,
            window_icon: None,
            always_on_top: self.always_on_top,
            decorations: self.decorations,
            maximized: self.maximized,
            resizable: self.resizable,
            transparent: self.transparent,
        };
        builder
    }
}
