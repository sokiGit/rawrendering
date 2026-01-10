use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use rawrendering::{Canvas, Graphics2D, Shape2D, Rect, Color, Circle};

fn bench_rect_fill(c: &mut Criterion) {
    let mut pixels = vec![0u32; 1920 * 1080];
    let shape = Shape2D::Rect(Rect {
        width: 100,
        height: 100,
        fill_color: Color::from(0xFF0000FF),
        ..Default::default()
    });

    let circle_large = Shape2D::Circle(Circle {
        radius: 512,
        outline_color: 0x12345678.into(),
        fill_color: 0x98765432.into(),
        ..Default::default()
    });

    let circle_medium = Shape2D::Circle(Circle {
        radius: 256,
        outline_color: 0x66996699.into(),
        fill_color: 0x77667766.into(),
        ..Default::default()
    });

    let circle_small = Shape2D::Circle(Circle {
        radius: 64,
        outline_color: 0xAAFFBBFF.into(),
        fill_color: 0x22AAAADD.into(),
        ..Default::default()
    });
    /*
    c.bench_function("draw_100x100_rect", |b| {
        b.iter(|| {
            // black_box prevents the compiler from "optimizing away" the code
            let mut canvas = Canvas { pixels: &mut pixels, width: 1920, height: 1080 };
            let mut g = Graphics2D { canvas: &mut canvas };
            g.draw_shape(black_box(&shape));
        })
    });*/

    c.bench_function("draw_multiple_circles", |b| {
        b.iter(|| {
            let mut canvas = Canvas { pixels: &mut pixels, width: 1920, height: 1080 };
            let mut g = Graphics2D { canvas: &mut canvas };
            g.draw_shape(black_box(&circle_large));
            g.draw_shape(black_box(&circle_medium));
            g.draw_shape(black_box(&circle_small));
        });
    });
}

criterion_group!(benches, bench_rect_fill);
criterion_main!(benches);