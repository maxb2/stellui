use image::DynamicImage;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

use crate::app::App;
use stellui::astro::CartesianCoordinates;

const SIZE: u32 = 800;
const CENTER: f32 = SIZE as f32 / 2.0;
const SCALE: f32 = SIZE as f32 * 0.47 / 2.0;

fn canvas_to_px(x: f64, y: f64) -> (f32, f32) {
    let px = CENTER + x as f32 * SCALE;
    let py = CENTER - y as f32 * SCALE; // y flipped
    (px, py)
}

fn planet_color_rgb(name: &str) -> (u8, u8, u8) {
    match name {
        "Mercury" => (128, 128, 128),
        "Venus" => (255, 220, 100),
        "Mars" => (200, 50, 50),
        "Jupiter" => (220, 200, 180),
        "Saturn" => (200, 180, 100),
        "Uranus" => (100, 220, 220),
        "Neptune" => (100, 100, 220),
        _ => (255, 255, 255),
    }
}

pub fn generate_sky_image(app: &App) -> DynamicImage {
    let mut pixmap = Pixmap::new(SIZE, SIZE).expect("failed to create pixmap");

    // Black background
    pixmap.fill(Color::BLACK);

    // Horizon circle: dark-gray stroke at canvas radius=2.0
    let horizon_r = 2.0f32 * SCALE;
    let mut pb = PathBuilder::new();
    pb.push_circle(CENTER, CENTER, horizon_r);
    if let Some(path) = pb.finish() {
        let mut paint = Paint::default();
        paint.set_color_rgba8(64, 64, 64, 255);
        paint.anti_alias = true;
        let stroke = Stroke { width: 1.5, ..Default::default() };
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    // Stars
    for star in &app.stars {
        let (px, py) = canvas_to_px(star.x, star.y);
        let (radius, r, g, b) = if star.mag <= 2.0 {
            (3.0f32, 255u8, 255u8, 255u8)
        } else if star.mag <= 4.0 {
            (2.0, 180, 180, 180)
        } else {
            (1.0, 100, 100, 100)
        };

        let mut pb = PathBuilder::new();
        pb.push_circle(px, py, radius);
        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color_rgba8(r, g, b, 255);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    // Sun
    if let Some(polar) = &app.sun_moon.sun_stereo {
        let c = CartesianCoordinates::from(polar);
        let (px, py) = canvas_to_px(c.x, c.y);
        let mut pb = PathBuilder::new();
        pb.push_circle(px, py, 6.0);
        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color_rgba8(255, 200, 0, 255);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    // Moon
    if let Some(polar) = &app.sun_moon.moon_stereo {
        let c = CartesianCoordinates::from(polar);
        let (px, py) = canvas_to_px(c.x, c.y);
        let mut pb = PathBuilder::new();
        pb.push_circle(px, py, 5.0);
        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color_rgba8(220, 220, 220, 255);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    // Planets
    for planet in &app.planets {
        let (px, py) = canvas_to_px(planet.x, planet.y);
        let (r, g, b) = planet_color_rgb(planet.name);
        let mut pb = PathBuilder::new();
        pb.push_circle(px, py, 3.0);
        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color_rgba8(r, g, b, 255);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    // Convert Pixmap (premultiplied RGBA) -> DynamicImage
    // All pixels are opaque so premul == straight alpha
    let data = pixmap.data().to_vec();
    let rgba_image =
        image::RgbaImage::from_raw(SIZE, SIZE, data).expect("image dimensions mismatch");
    DynamicImage::ImageRgba8(rgba_image)
}
