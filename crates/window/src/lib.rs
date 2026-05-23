use std::num::NonZeroU32;

use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::Display,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

struct WinitApp {
    window: Option<Window>,
    surface: Option<Surface<WindowSurface>>,
    context: Option<PossiblyCurrentContext>,
    gl: Option<glow::Context>,
}

impl WinitApp {
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            context: None,
            gl: None,
        }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("winit resumed");
        let winit_window_attributes = Window::default_attributes()
            .with_title("dupa")
            .with_inner_size(LogicalSize::new(800, 800));
        let window = event_loop.create_window(winit_window_attributes).unwrap();

        let raw_window_handle = window.window_handle().unwrap();
        let raw_display_handle = window.display_handle().unwrap();

        let preference = glutin::display::DisplayApiPreference::EglThenGlx(Box::new(|_| {}));

        let gl_display = unsafe { Display::new(raw_display_handle.as_raw(), preference).unwrap() };

        let template = ConfigTemplateBuilder::new();

        let gl_config = unsafe {
            gl_display
                .find_configs(template.build())
                .unwrap()
                .next()
                .unwrap()
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle.as_raw()));

        let not_current = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle.as_raw(),
            NonZeroU32::new(800).unwrap(),
            NonZeroU32::new(800).unwrap(),
        );

        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context = not_current.make_current(&surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap()) as *const _
            })
        };

        self.window = Some(window);
        self.surface = Some(surface);
        self.context = Some(context);
        self.gl = Some(gl);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                let gl = self.gl.as_ref().unwrap();

                unsafe {
                    gl.clear_color(0.0, 0.0, 0.0, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                self.surface
                    .as_ref()
                    .unwrap()
                    .swap_buffers(self.context.as_ref().unwrap())
                    .unwrap();
            }
            _ => {}
        }
    }
}

pub fn run_winit() {
    println!("winit run");
    let mut winit_app = WinitApp::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let _ = event_loop.run_app(&mut winit_app);
}
