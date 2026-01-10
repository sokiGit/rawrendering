// The primary code was transformed into a library.
// This is now only a testing file.
// It used to be main.rs, but now is only a demo/example.

use rawrendering::*;

fn main() {
    let width = 1920u32;
    let height = 1080u32;

    let dom = vec![
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
            ..Circle::default()
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
    ];

    for _ in 0..1000 {
        let mut canvas = Canvas {
            pixels: &mut vec![0u32; (width * height) as usize],
            width,
            height,
        };

        canvas.clear(0x00000000);

        let mut graphics = Graphics2D {
            canvas: &mut canvas,
        };

        for shape in &dom {
            graphics.draw_shape(shape);
        }
    }
}