#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use std::sync::mpsc;

    use simple_logger::SimpleLogger;
    use winit::{
        dpi::PhysicalSize,
        event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };


    const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(600, 400);

    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_inner_size(WINDOW_SIZE)
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let (tx_self, rx_self) = mpsc::channel();
    rayon::spawn(move || {
        while let Ok(event) = rx_self.recv() {
            match event {
                _ => (),
            }
        }
    });

    let (tx_global, rx_global) = mpsc::channel();
    rayon::spawn(move || {
        while let Ok(event) = rx_global.recv() {
            match event {
                DeviceEvent::Key(key) => {
                    dbg!(key);
                }
                _ => (),
            }
        }
    });

    event_loop.run(move |event, _event_loop, control_flow| {

        match event {
            Event::WindowEvent { event, window_id: _ } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::Destroyed
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    if let Some(event) = event.to_static() {
                        tx_self.send(event).unwrap();
                    }
                }
            },
            Event::DeviceEvent { device_id: _, event } => match event {
                _ => {
                    tx_global.send(event).unwrap();
                }
            }
            _ => {}
        }
    })
}
