pub use egui;

use aerin_renderer::Renderer;
use aerin_window::Window;
use egui::{ClippedPrimitive, Context, TextureId};
use egui_wgpu::{Renderer as WgpuRenderer, RendererOptions, ScreenDescriptor};
use egui_winit::State as WinitState;
use wgpu::{CommandEncoderDescriptor, Device, Queue};
use winit::event::WindowEvent as WinitWindowEvent;

pub struct EguiIntegration {
    pub context: Context,
    pub winit_state: WinitState,
    pub wgpu_renderer: WgpuRenderer,

    device: Device,
    queue: Queue,

    paint_jobs: Vec<ClippedPrimitive>,
    screen_descriptor: ScreenDescriptor,
    textures_to_free: Vec<TextureId>,
}

impl EguiIntegration {
    pub fn new(window: &Window, renderer: &Renderer) -> Self {
        let context = Context::default();

        let winit_state = WinitState::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.winit_window().scale_factor() as f32),
            None,
            Some(renderer.device().limits().max_texture_dimension_2d as usize),
        );

        let wgpu_renderer = WgpuRenderer::new(
            renderer.device(),
            renderer.format(),
            RendererOptions::default(),
        );

        let (width, height) = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: window.winit_window().scale_factor() as f32,
        };

        Self {
            context,
            winit_state,
            wgpu_renderer,
            device: renderer.device().clone(),
            queue: renderer.queue().clone(),
            paint_jobs: Vec::new(),
            screen_descriptor,
            textures_to_free: Vec::new(),
        }
    }

    pub fn on_event(&mut self, window: &Window, event: &WinitWindowEvent) -> bool {
        self.winit_state
            .on_window_event(window.winit_window(), event)
            .consumed
    }

    pub fn run(&mut self, window: &Window, mut run_ui: impl FnMut(&Context)) {
        for id in self.textures_to_free.drain(..) {
            self.wgpu_renderer.free_texture(&id);
        }
        let raw_input = self.winit_state.take_egui_input(window.winit_window());
        let mut full_output = self.context.run_ui(raw_input, |_| {
            run_ui(&self.context);
        });
        self.winit_state
            .handle_platform_output(window.winit_window(), full_output.platform_output);

        let (width, height) = window.inner_size();
        self.screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: full_output.pixels_per_point, // <── Use full_output's scale!
        };

        self.paint_jobs = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, deltas) in &full_output.textures_delta.set {
            for delta in deltas {
                self.wgpu_renderer
                    .update_texture(&self.device, &self.queue, *id, delta);
            }
        }
        self.textures_to_free
            .extend(full_output.textures_delta.free.iter().copied());
        full_output.textures_delta.clear();

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("egui_vertex_upload"),
            });
        let user_cmd_bufs = self.wgpu_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );
        let mut to_submit = vec![encoder.finish()];
        to_submit.extend(user_cmd_bufs);
        self.queue.submit(to_submit);
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'static>) {
        self.wgpu_renderer
            .render(render_pass, &self.paint_jobs, &self.screen_descriptor);
    }
}
