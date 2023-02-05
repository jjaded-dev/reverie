use super::{window::Window, renderer::Renderer, ui::UI};

pub struct Context {
    pub surface: wgpu::Surface,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub scale_factor: f64,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub egui_context: egui::Context,
    pub egui_renderer: egui_wgpu::Renderer,
    pub plot_id: egui::TextureId,
    pub ui: UI,
}

impl Context {
    pub async fn new(window: &winit::window::Window) -> Self {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();

        // let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        //     backends: wgpu::Backends::all(),
        //     dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        // });
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features:wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None
            },
            None,
        ).await.unwrap();
        
        // let surface_caps = surface.get_capabilities(&adapter);
        // let surface_format = surface_caps.formats.iter()
        //     .copied()
        //     .filter(|f| f.describe().srgb)
        //     .next()
        //     .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            //view_formats: vec![],
        };
        surface.configure(&device, &config);

        let egui_context = egui::Context::default();

        let mut egui_renderer = egui_wgpu::Renderer::new(&device, surface.get_supported_formats(&adapter)[0], None, 1);
        let mut renderer = Renderer::new(&device, &queue, &config);

        let plot_id = egui_renderer.register_native_texture(&device, &renderer.texture_view, wgpu::FilterMode::Linear);

        let mut ui = UI::new(plot_id.clone());

        Self {
            surface,
            window_size,
            scale_factor,
            device,
            queue,
            config,
            egui_context,
            egui_renderer,
            plot_id,
            ui,
        }
    }
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState  {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None
    })
}