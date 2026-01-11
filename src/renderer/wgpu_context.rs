

//! WGPU rendering context setup
//!
//! This module handles initialization of WGPU surface, device, and queue
//! for both web (WebGPU) and native platforms.

use wgpu::*;

/// WGPU rendering context containing device, queue, surface configuration
pub struct WgpuContext {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: (u32, u32),
}

/// Configuration for creating a WGPU context
pub struct WgpuContextConfig {
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentMode,
}

impl Default for WgpuContextConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            present_mode: PresentMode::Fifo, // VSync enabled by default
        }
    }
}

impl WgpuContext {
    /// Creates a new WGPU context with the given window surface
    ///
    /// # Arguments
    ///
    /// * `surface` - The window surface to render to
    /// * `config` - Configuration parameters for the context
    ///
    /// # Returns
    ///
    /// A new `WgpuContext` ready for rendering
    pub async fn new(surface: Surface<'static>, config: WgpuContextConfig) -> Self {
        let size = (config.width, config.height);

        // Request an adapter (GPU)
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Request a device and queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Rubik's Cube Renderer Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Get surface capabilities and configure
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: config.width,
            height: config.height,
            present_mode: config.present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        Self {
            surface,
            device,
            queue,
            config: surface_config,
            size,
        }
    }

    /// Resizes the rendering surface
    ///
    /// # Arguments
    ///
    /// * `new_width` - New width in pixels
    /// * `new_height` - New height in pixels
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.size = (new_width, new_height);
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Gets the current surface texture for rendering
    ///
    /// # Returns
    ///
    /// The current surface texture wrapped in a Result
    pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Gets the aspect ratio of the current surface
    ///
    /// # Returns
    ///
    /// Width divided by height as f32
    pub fn aspect_ratio(&self) -> f32 {
        self.size.0 as f32 / self.size.1 as f32
    }

    /// Gets the current size of the rendering surface
    ///
    /// # Returns
    ///
    /// Tuple of (width, height)
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WgpuContextConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.present_mode, PresentMode::Fifo);
    }

    #[test]
    fn test_custom_config() {
        let config = WgpuContextConfig {
            width: 1920,
            height: 1080,
            present_mode: PresentMode::Immediate,
        };
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.present_mode, PresentMode::Immediate);
    }

    #[test]
    fn test_aspect_ratio_calculation() {
        // We can't easily test the full WgpuContext without a window,
        // but we can test the aspect ratio calculation logic
        let width = 1920u32;
        let height = 1080u32;
        let aspect = width as f32 / height as f32;
        assert!((aspect - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn test_config_values() {
        let config = WgpuContextConfig {
            width: 640,
            height: 480,
            present_mode: PresentMode::Mailbox,
        };
        assert!(config.width > 0);
        assert!(config.height > 0);
    }
}
