// The primary code was transformed into a library.
// This is now only a testing file.
// It used to be main.rs, but now is only a demo/example.

use std::arch::x86_64::_mm_aesdecwide128kl_u8;
use std::mem::offset_of;
use std::num::{NonZeroU32};
use std::process::exit;
use std::rc::Rc;
use softbuffer::{Context, Surface};
use winit::{
    self,
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId}
};
use winit::window::CursorIcon::Default;
use rawrendering::*;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    dom: Vec<Shape2D>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Softbuffer Rendering Test");

        let raw_window = match event_loop.create_window(window_attributes) {
            Ok(raw_window) => raw_window,
            Err(e) => {
                println!("Error creating window: {:?}", e);
                return;
            }
        };

        let window = Rc::new(raw_window);

        let context = match Context::new(window.clone()) {
            Ok(context) => context,
            Err(e) => {
                println!("Error creating context: {:?}", e);
                return;
            }
        };

        let surface = match Surface::new(&context, window.clone()) {
            Ok(surface) => surface,
            Err(e) => {
                println!("Error creating surface: {:?}", e);
                return;
            }
        };

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let (Some(surface), Some(window)) = (self.surface.as_mut(), self.window.as_ref()) {
                    let size = window.inner_size();
                    let (width, height) = (size.width, size.height);

                    if let Err(e) = surface.resize(
                        match NonZeroU32::new(width) {
                            Some(width) => width,
                            None => return,
                        },
                        match NonZeroU32::new(height) {
                            Some(height) => height,
                            None => return,
                        },
                    ) {
                        println!("Error resizing surface: {:?}", e);
                        return;
                    }

                    let mut buffer = match surface.buffer_mut() {
                        Ok(buffer) => buffer,
                        Err(e) => {
                            println!("Error getting buffer: {:#?}", e);
                            return;
                        },
                    };

                    let mut canvas = Canvas {
                        pixels: &mut buffer,
                        width,
                        height,
                    };

                    canvas.clear(0x00000000);

                    let mut graphics = Graphics2D {
                        canvas: &mut canvas,
                    };

                    for shape in &self.dom {
                        graphics.draw_shape(shape);
                    }

                    // Render buffer to screen
                    if let Err(e) = buffer.present() {
                        println!("Error presenting buffer: {:?}", e);
                    }
                }
            },
            _ => ()
        }
    }
}

fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            println!("Exit 1: Event loop creation error: {:#?}", e);
            exit(1)
        },
    };

    event_loop.set_control_flow(ControlFlow::Wait); // Run event loop only on change
    // ControlFlow::Poll runs on every available cycle, ideal for games

    let center_transform_32radius = Object2D {
        relative_offset: Vec2::new(0.5, 0.5),
        anchor: Vec2::new(32, 32),
        ..Object2D::default()
    };

    let mut app = App {
        dom: vec![
            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(8, 8),
                    ..Object2D::default()
                },
                width: 220,
                height: 42,
                outline_color: 0xCCFFFFFF.into(),
                fill_color: 0xDDFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(236, 8),
                    ..Object2D::default()
                },
                width: 220,
                height: 42,
                outline_color: 0xDDFFFFFF.into(),
                fill_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Circle(Circle {
                base: Object2D {
                    offset: Vec2::new(16, 16),
                    ..Object2D::default()
                },
                radius: (42 - (16 - 8) * 2) / 2,
                fill_color: 0x00738ADB.into(),
                ..Circle::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(244, 8 + 42 / 2 - 22 / 2),
                    ..Object2D::default()
                },
                fill_color: 0x00FF0000.into(),
                width: 32,
                height: 22,
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(0, 42 + 8 + 8),
                    ..Object2D::default()
                },
                width: 800,
                height: 600 - (42 + 8 + 8),
                fill_color: 0x00080808.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(128, 42 + 8 + 8 + 4),
                    ..Object2D::default()
                },
                width: 800 - 256,
                height: 32,
                fill_color: 0x88000000.into(),
                outline_color: 0xAA444444.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect{
                base: Object2D {
                    offset: Vec2::new(42, 42 + 8 + 8 + 32 + 8),
                    ..Object2D::default()
                },
                width: 196-42,
                height: 600 - (42 + 8 + 8 + 8 + 32 + 52),
                outline_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    relative_offset: Vec2::new(0f32, 1f32),
                    anchor: Vec2::new(0, 52),
                    ..Object2D::default()
                },
                height: 52,
                width: 196,
                fill_color: 0x00181818.into(),
                outline_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Circle(Circle {
                base: Object2D {
                    relative_offset: Vec2::new(0f32, 1f32),
                    anchor: Vec2::new(0, (52 - 8 * 2)),
                    offset: Vec2::new(8, -8),
                    ..Object2D::default()
                },
                fill_color: 0x00AA55AA.into(),
                radius: (52 - 8 * 2) / 2,
                ..Circle::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    relative_offset: Vec2::new(0f32, 1f32),
                    offset: Vec2::new(196, 0),
                    anchor: Vec2::new(0, 52),
                    ..Object2D::default()
                },
                width: 800 - 196 - 256,
                height: 52,
                fill_color: 0x00202020.into(),
                outline_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    relative_offset: Vec2::new(1f32, 1f32),
                    anchor: Vec2::new(256, 600 - (42 + 8 + 8 + 8 + 32)),
                    ..Object2D::default()
                },
                width: 256,
                height: 600 - (42 + 8 + 8 + 32),
                outline_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Circle(Circle {
                base: Object2D {
                    offset: Vec2::new(-256 + 8, 42 + 8 + 8 + 8 + 32 + 12 + 32),
                    relative_offset: Vec2::new(1f32, 0f32),
                    ..Object2D::default()
                },
                fill_color: 0x00AA55AA.into(),
                radius: 16,
                ..Circle::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(42, 42 + 8 + 8 + 8 + 32),
                    ..Object2D::default()
                },
                width: 800 - 42 - 256,
                height: 32,
                outline_color: 0xEEFFFFFF.into(),
                ..Rect::default()
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(42 + 4, 42 + 8 + 8 + 8 + 32 + 32 + 4 + 12),
                    ..Object2D::default()
                },
                height: 28,
                width: 196 - 42 - 8,
                fill_color: 0xDDFFFFFF.into(),
                ..Rect::default()
            }),
        ],
        ..App::default()
    };
    match event_loop.run_app(&mut app) {
        Ok(_) => {
            println!("Bye bye :3");
        },
        Err(e) => {
            println!("The application has encountered an error:\n{:#?}", e);
        }
    };
}