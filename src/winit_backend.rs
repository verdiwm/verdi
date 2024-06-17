use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow, window::Window,
};

pub struct WinitBackend {
    state: Option<(Arc<Window>, Surface<Arc<Window>, Arc<Window>>)>,
}

impl WinitBackend {
    pub fn new() -> Self {
        WinitBackend { state: None }
    }
}

impl ApplicationHandler for WinitBackend {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Basic softbuffer"))
                .unwrap(),
        );

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.state = Some((window, surface));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let (window, surface) = self.state.as_mut().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            WindowEvent::RedrawRequested => {
                if window_id == window.id() {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();
                        for y in 0..height.get() {
                            for x in 0..width.get() {
                                let red = x % 255;
                                let green = y % 255;
                                let blue = (x * y) % 255;
                                let index = y as usize * width.get() as usize + x as usize;
                                buffer[index] = blue | (green << 8) | (red << 16);
                            }
                        }

                        buffer.present().unwrap();
                    }
                }
            }
            WindowEvent::CloseRequested => {
                if window_id == window.id() {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}
