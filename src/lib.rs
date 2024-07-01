use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

struct Model<'a>{
	surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    window: &'a Window,
}

impl<'a> Model<'a> {
    async fn new(window: &'a Window) -> Model<'a> {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(
			wgpu::InstanceDescriptor{
				backends: wgpu::Backends::PRIMARY,
				..Default::default()
			}
		);

		let surface = instance.create_surface(&window).unwrap();

		let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("error finding adapter");

		neko
	}
}