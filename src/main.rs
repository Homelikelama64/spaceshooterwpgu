#![deny(rust_2018_idioms)]

use game::Game;
use renderer::{FrameRendering, Renderer};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod always_some;
mod game;
pub mod renderer;

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
    let mut game = Game::new(&mut renderer);

    let mut last_frame = None;

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
                let time = Instant::now();
                let dt = last_frame
                    .map(|last_frame| time - last_frame)
                    .unwrap_or(Duration::ZERO)
                    .as_secs_f32();
                last_frame = Some(time);

                game.update(dt);
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let clear_color = wgpu::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                };

                let Some(mut frame) = FrameRendering::new(&mut renderer, clear_color) else {
                    return;
                };

                game.render(&mut frame);
            }

            _ => (),
        })
        .unwrap();
}
