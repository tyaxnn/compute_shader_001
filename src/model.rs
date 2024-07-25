pub mod define_model{

    use wgpu::core::pipeline;
    use wgpu::util::DeviceExt;
    use wgpu::BufferUsages;
    
    use winit::{
        event::*,
        window::Window,
    };

    use crate::compute::compute_model::{input_tx_views_factory, output_tx_view_factory, ComputeModel};
    use crate::render::render_model::RenderModel;
    use crate::status::about_status::{Status,PinPongStatus};

    pub struct Model<'a>{
        pub window: &'a Window,
        pub size: winit::dpi::PhysicalSize<u32>,
        pub surface: wgpu::Surface<'a>,
        pub device: wgpu::Device,
        pub queue: wgpu::Queue,
        pub config: wgpu::SurfaceConfiguration,

        pub compute_model : ComputeModel,
        pub render_model : RenderModel,
        pub status : Status,
    }

    impl<'a> Model<'a> {
        pub async fn new(window: &'a Window) -> Model<'a> {
    
            /*------------------------------------
                    surface device etc ...
            ------------------------------------*/
            let size = window.inner_size();
    
            let instance = wgpu::Instance::new(
                wgpu::InstanceDescriptor{
                    backends: wgpu::Backends::PRIMARY,
                    ..Default::default()
                }
            );
    
            let surface = instance.create_surface(window).unwrap();
    
            let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("error finding adapter");
    
            let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::TEXTURE_BINDING_ARRAY,
                    required_limits: {
                        let mut default_lim = wgpu::Limits::default();
                        
                        default_lim.max_sampled_textures_per_shader_stage = 1024;
    
                        default_lim
                    },
                },
                //Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
    
            let surface_caps = surface.get_capabilities(&adapter);
    
            let surface_format = surface_caps
                .formats
                .iter()
                .copied()
                .find(|f| f.is_srgb())
                .unwrap_or(surface_caps.formats[0]);
    
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                desired_maximum_frame_latency: 2,
                view_formats: vec![],
            };
    
            surface.configure(&device, &config);

            
            let status = Status::new();
    
            /*------------------------------------
                    in/output textures
            ------------------------------------*/
            

            //create input_texture_views
            //  load images here
            let input_tx_views = input_tx_views_factory(&device, &queue,status);
            
            //create output_texture_view
            let output_tx_view = output_tx_view_factory(&device, &size);
    
            let mut input_tx_views_b = Vec::new();
    
            for i in 0..input_tx_views.len(){
                input_tx_views_b.push(&input_tx_views[i])
            }

    
            /*------------------------------------
                    compute model
            ------------------------------------*/
    
            let compute_model = ComputeModel::new(&device, size,&input_tx_views_b,&output_tx_view,status);
    
            /*------------------------------------
                    render model
            ------------------------------------*/
    
            let render_model = RenderModel ::new(&device, surface_format, output_tx_view);
    
    
            /*------------------------------------
                    Return Model
            ------------------------------------*/
            Self {
                window,
                size,
                surface,
                device,
                queue,
                config,
                compute_model,
                render_model,
                status,
            }
        }
    
        pub fn window(&self) -> &Window {
            &self.window
        }
    
        pub fn input(&mut self, _event: &WindowEvent) -> bool {
            false
        }
        
        //update status
        pub fn update_pre(&mut self) {
            

            let elapsed_time: f32 = 0.5 + self.status.start_time.elapsed().as_micros() as f32 * 1e-6;

            self.status.next_frame_index = (elapsed_time * self.status.frame_rate as f32) as u32 % self.status.frame_len;
        }

        pub fn update_post(&mut self) {
            self.status.elapsed_frame += 1;

            self.status.ping_pong = PinPongStatus::FtT2;
        }
    
        pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            if new_size.width > 0 && new_size.height > 0 {
                self.size = new_size;
                self.config.width = new_size.width;
                self.config.height = new_size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }
    
        pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let status_buffer_data = [self.size.width, self.size.height, self.status.next_frame_index,0,(0f32).to_bits()];
            
    
            let status_buffer_host = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&status_buffer_data),
                usage: BufferUsages::COPY_SRC,
            });
            let mut encoder = self.device.create_command_encoder(&Default::default());
            encoder.copy_buffer_to_buffer(&status_buffer_host, 0, &self.compute_model.status_buffer, 0, self.status.buffer_size);

            //copy texture date to buffer
            {
                let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                compute_pass.set_pipeline(&self.compute_model.pipeline_init);

                compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);

                self.status.ping_pong = PinPongStatus::F1T2;
                
                compute_pass.dispatch_workgroups(self.size.width / 16, self.size.height / 16, 1);
            }
            
            //Filter the image here
            for pipeline in &self.compute_model.pipelines
            {
                let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                compute_pass.set_pipeline(pipeline);

                match self.status.ping_pong{
                    PinPongStatus::F2T1 => {
                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);

                        self.status.ping_pong = PinPongStatus::F1T2
                    }
                    PinPongStatus::F1T2 => {
                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);

                        self.status.ping_pong = PinPongStatus::F2T1
                    }
                    _ => {panic!("Wrong Ping-Pong Status")}
                }
                
                compute_pass.dispatch_workgroups(self.size.width / 16, self.size.height / 16, 1);
            }

            {
                
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
    
                render_pass.set_pipeline(&self.render_model.pipeline);
                render_pass.set_bind_group(0, &self.render_model.bindgroup, &[]);
                render_pass.draw(0..3, 0..2);
    
            }
            
    
            self.queue.submit(Some(encoder.finish()));
            output.present();
    
            Ok(())
        }
    }


}