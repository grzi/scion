use std::sync::Arc;

use wgpu::{Limits, Surface, SurfaceConfiguration};
use winit::{window::Window};

use crate::core::components::color::Color;
use crate::graphics::rendering::{RenderingInfos, RenderingUpdate};
use crate::graphics::rendering::scion2d::renderer::Scion2D;

pub(crate) struct ScionWindowRenderingManager {
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    scion_renderer: Scion2D,
    default_background_color: Option<Color>,
}

impl ScionWindowRenderingManager {
    pub(crate) async fn new(window: Arc<Window>,
                            default_background : Option<Color>) -> Self {
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let backends = wgpu::util::backend_bits_from_env().unwrap_or_default();
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            dx12_shader_compiler,
            gles_minor_version,
        });

        let surface = instance.create_surface(window).expect("Surface creation failed");
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
            .await
            .expect("No suitable GPU adapters found on the system!");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::TEXTURE_BINDING_ARRAY | wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER,
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: Limits {
                        max_texture_array_layers: 512,
                        ..Limits::default()
                    }
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut config = surface
            .get_default_config(&adapter, width, height)
            .unwrap();

        surface.configure(&device, &config);

        let mut scion_renderer = Scion2D::default();
        scion_renderer.start(&device, &config);

        Self { surface, device, queue, config, scion_renderer, default_background_color: default_background }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, _scale_factor: f64) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(crate) fn update(&mut self, updates: Vec<RenderingUpdate>) {
        self.scion_renderer.update(updates, &self.device, &self.config, &mut self.queue);
    }

    pub(crate) fn render(
        &mut self,
        data: Vec<RenderingInfos>,
    ) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.scion_renderer.render(data, &self.default_background_color, view, &mut encoder);

        self.queue.submit(Some(encoder.finish()));

        frame.present();
        Ok(())
    }
}
