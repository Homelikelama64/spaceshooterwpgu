#![deny(rust_2018_idioms)]

use renderer::{FrameRendering, Renderer, Rendering2D};
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod always_some;
mod renderer;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Space Shooter")
            .with_visible(false)
            .build(&event_loop)
            .unwrap(),
    );

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    window.set_visible(true);
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                window.set_visible(false);
                elwt.exit();
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.resize(size);
            }

            Event::AboutToWait => {
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let clear_color = wgpu::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                };

                let Some(mut frame) = FrameRendering::new(&mut renderer, clear_color) else {
                    return;
                };

                let mut rendering = Rendering2D::new(&mut frame, cgmath::vec2(0.0, 0.0), 1.0);
                rendering.draw_quad(
                    cgmath::vec2(0.0, 0.0),
                    cgmath::vec2(0.5, 0.2),
                    cgmath::vec4(0.0, 1.0, 0.0, 1.0),
                );
            }

            _ => (),
        })
        .unwrap();
}
