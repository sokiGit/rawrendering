// The primary code was transformed into a library.
// This is now only a testing file.
// It used to be main.rs, but now is only a demo/example.

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

                    /*canvas.draw_polygon_outline(
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
                    );*/

                    /*
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
                    );*/

                    /*
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
                    );*/


                    /*
                    canvas.draw_rect_outline(200, 200, 400, 400, 0x00888888);
                    canvas.draw_line(200, 200, 300,  200, 0x00FF8888);
                    canvas.draw_line(300, 400, 400, 400, 0x008888FF);*/

                    /*
                    // Bezier curves
                    canvas.draw_cubic_bezier_curve_outline(
                        200, 200,
                        300, 200,
                        400, 400,
                        300, 400,
                        0x00FF44AA
                    );

                    canvas.draw_cubic_bezier_curve_outline(
                        200, 200,
                        200, 100,
                        400, 400,
                        400, 500,
                        0x00AA44FF
                    );

                    canvas.draw_cubic_bezier_curve_outline(
                        200, 200,
                        700, 200,
                        400, 400,
                        400, -100,
                        0x00AAFF44
                    );*/

                    /*
                    canvas.draw_polygon_outline(
                        Polygon {
                            path: &[
                                (530, 450),
                                (570, 450),
                                (590, 470),
                                (570, 490),
                                (530, 490)
                            ],
                            color: 0x0055AAAA,
                            closed: true,
                        }
                    );*/

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
                    offset: Vec2::new(20, 20),
                    ..Object2D::default()
                },
                width: 32,
                height: 32,
                fill_color: 0x00FF00FF.into(),
                outline_color: 0xFF000000.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(128, 64),
                    ..Object2D::default()
                },
                width: 64,
                height: 32,
                fill_color: 0x00664499.into(),
                outline_color: 0xFF000000.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(128, 64),
                    ..Object2D::default()
                },
                width: 64,
                height: 64,
                fill_color: 0xFF000000.into(),
                outline_color: 0x00AA55FF.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(200, 200),
                    ..Object2D::default()
                },
                width: 200,
                height: 0,
                fill_color: 0xFF000000.into(),
                outline_color: 0x00FFAAAA.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(200, 210),
                    ..Object2D::default()
                },
                width: 200,
                height: 1,
                fill_color: 0xFF000000.into(),
                outline_color: 0x00AAFFAA.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(200, 220),
                    ..Object2D::default()
                },
                width: 200,
                height: 2,
                fill_color: 0xFF000000.into(),
                outline_color: 0x00AAAAFF.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(300, 64),
                    ..Object2D::default()
                },
                width: 64,
                height: 64,
                fill_color: 0x00FF0000.into(),
                outline_color: 0xFF000000.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(332, 96),
                    ..Object2D::default()
                },
                width: 64,
                height: 64,
                fill_color: 0x8800FF00.into(),
                outline_color: 0xFF000000.into(),
            }),

            Shape2D::VectorPath2D(VectorPath2D {
                base: Object2D {
                    offset: Vec2::new(30, 30),
                    ..Object2D::default()
                },
                path: vec![
                    PathTrace2D::Line2D(Line2D {
                        from: Vec2::new(0, 0),
                        to: Vec2::new(30, 60),
                        color: 0x00FFFFFF.into(),
                    }),
                ],
            }),

            Shape2D::VectorPath2D(VectorPath2D {
                base: Object2D::default(),
                path: vec![
                    PathTrace2D::Line2D(Line2D {
                        from: Vec2::new(0, 400),
                        to: Vec2::new(600, 0),
                        color: 0x00AAAAAAu32.into(),
                    }),
                ],
            }),

            // [...]

            Shape2D::Circle(Circle {
                base: Object2D {
                    offset: Vec2::new(200, 200),
                    anchor: Vec2::new(32, 32),
                    ..Object2D::default()
                },
                radius: 32,
                fill_color: 0x00AA5533.into(),
                outline_color: 0xFF000000.into(),
                outline_thickness: 1,
            }),

            Shape2D::Circle(Circle {
                base: Object2D {
                    offset: Vec2::new(200, 200),
                    anchor: Vec2::new(32 * 16, 32 * 16),
                    ..Object2D::default()
                },
                radius: 32 * 16,
                fill_color: 0xFF000000.into(),
                outline_color: 0x0044FFFF.into(),
                outline_thickness: 12,
            }),

            // [...]

            Shape2D::Rect(Rect {
                    base: Object2D {
                        offset: Vec2::new(12, 24),
                        ..Object2D::default()
                    },
                    height: 64,
                    width: 64,
                    fill_color: 0x0000FF00.into(),
                    outline_color: 0x00FF00FF.into(),
                    ..Rect::default()
                }
            ),

            Shape2D::Rect(Rect {
                    base: Object2D {
                        relative_offset: Vec2::new(0.5, 0.5),
                        anchor: Vec2::new(16, 16),
                        ..Object2D::default()
                    },
                    width: 32,
                    height: 32,
                    fill_color: 0x00AA2266.into(),
                    outline_color: 0xFF000000.into(),
                }
            ),

            Shape2D::Circle(Circle {
                    base: Object2D {
                        offset: Vec2::new(75, 50),
                        ..Object2D::default()
                    },
                    radius: 20,
                    fill_color: 0x00FF0000.into(),
                    outline_color: 0x00FFFFFF.into(),
                    outline_thickness: 1,
                }
            ),

            Shape2D::VectorPath2D(VectorPath2D {
                    base: Object2D::default(),
                    path: vec![
                        PathTrace2D::Line2D(Line2D {
                            from: Vec2::new(0, 0),
                            to: Vec2::new(50, 0),
                            color: 0x0000FFFF.into(),
                        }),
                        PathTrace2D::CubicBezierCurve2D(CubicBezierCurve2D {
                            from: Vec2::new(50, 0),
                            to: Vec2::new(100, 50),
                            from_control_point: Vec2::new(100, 0),
                            to_control_point: Vec2::new(100, 0),
                            color: 0x0000FFFF.into(),
                        }),
                    ],
                }
            ),

            Shape2D::VectorPath2D(VectorPath2D {
                    base: Object2D {
                        offset: Vec2::new(750, 600),
                        ..Object2D::default()
                    },
                    path: vec![
                        PathTrace2D::CubicBezierCurve2D(CubicBezierCurve2D {
                            from: Vec2::new(100, 50),
                            to: Vec2::new(200, 50),
                            from_control_point: Vec2::new(110, -15),
                            to_control_point: Vec2::new(200, -15),
                            color: 0x00FF0000.into(),
                        }),
                        PathTrace2D::CubicBezierCurve2D(CubicBezierCurve2D {
                            from: Vec2::new(200, 50),
                            to: Vec2::new(100, 175),
                            from_control_point: Vec2::new(200, 100),
                            to_control_point: Vec2::new(150, 150),
                            color: 0x00FF0000.into(),
                        }),
                        PathTrace2D::CubicBezierCurve2D(CubicBezierCurve2D {
                            from: Vec2::new(100, 175),
                            to: Vec2::new(0, 50),
                            from_control_point: Vec2::new(50, 150),
                            to_control_point: Vec2::new(0, 100),
                            color: 0x00FF0000.into(),
                        }),
                        PathTrace2D::CubicBezierCurve2D(CubicBezierCurve2D {
                            from: Vec2::new(0, 50),
                            to: Vec2::new(100, 50),
                            from_control_point: Vec2::new(0, -15),
                            to_control_point: Vec2::new(90, -15),
                            color: 0x00FF0000.into(),
                        })
                    ]
                }
            ),

        Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(8, -32),
                    relative_offset: Vec2::new(0f32, 1f32),
                    ..Object2D::default()
                },
                width: 600 - 16,
                height: 24,
                outline_color: 0x0099DDFF.into(),
                fill_color: 0x7799DDFF.into(),
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(64, 64),
                    relative_offset: Vec2::new(0.5, 0.5),
                    anchor: Vec2::new(16, 16),
                    ..Object2D::default()
                },
                width: 32,
                height: 32,
                fill_color: 0x000000FF.into(),
                outline_color: 0x0000FF00.into(),
            }),

            Shape2D::Circle(Circle {
                base: Object2D {
                    anchor: Vec2::new(16, 16),
                    relative_offset: Vec2::new(0.5, 0.5),
                    ..Object2D::default()
                },
                radius: 16,
                fill_color: 0xCC0000FF.into(),
                outline_color: 0xCCFFFFFF.into(),
                outline_thickness: 4,
            }),

            Shape2D::Circle(Circle {
                base: Object2D::default(),
                radius: 0xFF,
                fill_color: 0xFF000000.into(),
                outline_color: 0xCCFF00FF.into(),
                outline_thickness: 1,
            }),

            Shape2D::Circle(Circle {
                base: Object2D::default(),
                radius: 0xFA,
                fill_color: 0xFF000000.into(),
                outline_color: 0x00FF00FF.into(),
                outline_thickness: 1,
            }),

            Shape2D::Rect(Rect {
                base: Object2D {
                    offset: Vec2::new(512, 256),
                    ..Object2D::default()
                },
                height: 32,
                width: 16,
                fill_color: 0xCCFF00FF.into(),
                outline_color: 0xFFFF00FF.into(),
            }),
        ],
        ..App::default()
    };

    println!("Shape2D:\t\t{:} B", std::mem::size_of::<Shape2D>());
    println!("Line2D:\t\t\t{:} B", std::mem::size_of::<Line2D>());
    println!("CubicBezierCurve2D:\t{:} B", std::mem::size_of::<CubicBezierCurve2D>());
    println!("Rect:\t\t\t{:} B", std::mem::size_of::<Rect>());
    println!("Circle:\t\t\t{:} B", std::mem::size_of::<Circle>());
    println!("VectorPath2D:\t\t{:} B", std::mem::size_of::<VectorPath2D>());
    println!("Object2D:\t\t{:} B", std::mem::size_of::<Object2D>());
    println!("Canvas:\t\t\t{:} B", std::mem::size_of::<Canvas>());
    println!("Vec2<u32>:\t\t{:} B", std::mem::size_of::<Vec2<u32>>());
    println!("(u32, u32):\t\t{:} B", std::mem::size_of::<(u32, u32)>());
    println!("Vec2<f32>:\t\t{:} B", std::mem::size_of::<Vec2<f32>>());
    println!("(f32, f32):\t\t{:} B", std::mem::size_of::<(f32, f32)>());
    println!("Color:\t\t\t{:} B", std::mem::size_of::<Color>());
    println!("u32:\t\t\t{:} B", std::mem::size_of::<u32>());
    println!("Vec<PathTrace2D>:\t{:} B", std::mem::size_of::<Vec<PathTrace2D>>());
    println!("PathTrace2D:\t\t{:} B", std::mem::size_of::<PathTrace2D>());
    println!("&PathTrace2D:\t\t{:} B", std::mem::size_of::<&PathTrace2D>());

    match event_loop.run_app(&mut app) {
        Ok(_) => {
            println!("Bye bye :3");
        },
        Err(e) => {
            println!("The application has encountered an error:\n{:#?}", e);
        }
    };
}