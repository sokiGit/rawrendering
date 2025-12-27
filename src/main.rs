use std::num::{NonZeroU32};
use std::process::exit;
use std::rc::Rc;
use softbuffer::{Context, Surface};
use winit;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::draw::Canvas;
use crate::draw::polygon::Polygon;

mod draw;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
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
                        width: width as usize,
                        height: height as usize,
                    };

                    canvas.clear(0x000000);
                    canvas.draw_rect(20, 20, 32, 32, 0x00FF00FF);
                    canvas.draw_rect(128, 64, 64, 32, 0x00664499);
                    canvas.draw_rect_outline(128, 64, 64, 64, 0x00AA55FF);
                    canvas.draw_rect_outline(200, 200, 200, 0, 0x00FFAAAA);
                    canvas.draw_rect_outline(200, 210, 200, 1, 0x00AAFFAA);
                    canvas.draw_rect_outline(200, 220, 200, 2, 0x00AAAAFF);
                    canvas.draw_rect_with_transparency(8, (height.max(32) - 32) as usize, (width.max(16) - 16) as usize, 24, 0x7799DDFF);
                    canvas.draw_rect_outline(8, (height.max(32) - 32) as usize, (width.max(16) - 16) as usize, 24, 0x0099DDFF);

                    canvas.draw_rect(300, 64, 64, 64, 0xFF0000);
                    canvas.draw_rect_with_transparency(332, 96, 64, 64, 0x8800FF00);

                    canvas.draw_line(30, 30, 60, 90, 0x00FFFFFF);
                    canvas.draw_line(0, height as usize, width as usize, 0, 0x00AAAAAA);

                    canvas.draw_polygon_outline(
                        Polygon {
                            path: &[
                                (100, 100),
                                (200, 100),
                                (220, 90),
                                (300, 300),
                                (450, 320),
                                (200, 350),
                            ],
                            color: 0x00FFAAAA,
                            closed: true,
                        }
                    );

                    canvas.draw_polygon_outline(
                        Polygon {
                            path: &[
                                (350, 350),
                                (375, 450),
                                (375, 500),
                                (400, 500),
                                (500, 400),
                            ],
                            color: 0x0000FFAA,
                            closed: false,
                        }
                    );

                    // <3
                    canvas.draw_polygon_outline(
                        Polygon {
                            path: &[
                                (200, 200),
                                (300, 200),
                                (400, 300),
                                (500, 200),
                                (600, 200),
                                (700, 300),
                                (700, 500),
                                (400, 800),
                                (100, 500),
                                (100, 300),
                            ],
                            color: 0x00FF0000,
                            closed: true,
                        }
                    );

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

    let mut app = App::default();

    match event_loop.run_app(&mut app) {
        Ok(_) => {
            println!("Bye bye :3");
        },
        Err(e) => {
            println!("The application has encountered an error:\n{:#?}", e);
        }
    };
}