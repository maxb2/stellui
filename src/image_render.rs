use image::DynamicImage;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

use crate::sky::{RenderedPlanet, RenderedStar, SunMoonInfo};
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

/// Snapshot of the sky data needed to render an image, cloneable and Send.
#[derive(Clone)]
pub struct SkySnapshot {
    pub stars: Vec<RenderedStar>,
    pub sun_moon: SunMoonInfo,
    pub planets: Vec<RenderedPlanet>,
    pub southern: bool,
}

/// Draw a single character as strokes. (ox, oy) is top-left; y increases downward.
fn draw_char(pixmap: &mut Pixmap, ch: char, ox: f32, oy: f32, h: f32, paint: &Paint, stroke: &Stroke) {
    let w = h * 0.65;
    let m = h * 0.5;

    let lines: &[(f32, f32, f32, f32)] = match ch {
        'N' => &[
            (0.0, 0.0, 0.0, h),
            (w,   0.0, w,   h),
            (0.0, 0.0, w,   h),
        ],
        'S' => &[
            (w,   0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, m),
            (0.0, m,   w,   m),
            (w,   m,   w,   h),
            (w,   h,   0.0, h),
        ],
        'E' => &[
            (0.0, 0.0, 0.0, h),
            (0.0, 0.0, w,   0.0),
            (0.0, m,   w * 0.75, m),
            (0.0, h,   w,   h),
        ],
        'W' => &[
            (0.0,      0.0, w * 0.25, h),
            (w * 0.25, h,   w * 0.5,  m),
            (w * 0.5,  m,   w * 0.75, h),
            (w * 0.75, h,   w,        0.0),
        ],
        _ => &[],
    };

    for &(x1, y1, x2, y2) in lines {
        let mut pb = PathBuilder::new();
        pb.move_to(ox + x1, oy + y1);
        pb.line_to(ox + x2, oy + y2);
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, paint, stroke, Transform::identity(), None);
        }
    }
}

/// Draw a cardinal label centered at (cx, cy) in image pixel space.
fn draw_cardinal(pixmap: &mut Pixmap, label: &str, cx: f32, cy: f32) {
    let h = 18.0_f32;
    let w = h * 0.65;
    let mut paint = Paint::default();
    paint.set_color_rgba8(160, 160, 160, 255);
    paint.anti_alias = true;
    let stroke = Stroke { width: 2.0, ..Default::default() };
    // Center the label horizontally and vertically
    let total_w = label.len() as f32 * (w + 2.0) - 2.0;
    let ox = cx - total_w * 0.5;
    let oy = cy - h * 0.5;
    for (i, ch) in label.chars().enumerate() {
        draw_char(pixmap, ch, ox + i as f32 * (w + 2.0), oy, h, &paint, &stroke);
    }
}

pub fn generate_sky_image(snap: &SkySnapshot) -> DynamicImage {
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
    for star in &snap.stars {
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
    if let Some(polar) = &snap.sun_moon.sun_stereo {
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
    if let Some(polar) = &snap.sun_moon.moon_stereo {
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
    for planet in &snap.planets {
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

    // Cardinal labels — placed just inside the image edge (horizon circle is 2.0*SCALE ≈ 376px
    // from center; image half-width is 400px, so 2.15*SCALE would be out of bounds)
    let label_r = 2.0f32 * SCALE + 8.0;
    // Image y increases downward, canvas y increases upward (y flipped in canvas_to_px).
    // Northern hemisphere: N is at canvas y=-2.15 → image y = CENTER + r (bottom of image).
    // n_y is the image-y offset from CENTER (positive = down).
    let (n_y, s_y) = if snap.southern { (-label_r, label_r) } else { (label_r, -label_r) };
    let (e_x, w_x) = if snap.southern { (-label_r, label_r) } else { (label_r, -label_r) };
    draw_cardinal(&mut pixmap, "N", CENTER, CENTER + n_y);
    draw_cardinal(&mut pixmap, "S", CENTER, CENTER + s_y);
    draw_cardinal(&mut pixmap, "E", CENTER + e_x, CENTER);
    draw_cardinal(&mut pixmap, "W", CENTER + w_x, CENTER);

    // Convert Pixmap (premultiplied RGBA) -> DynamicImage
    // All pixels are opaque so premul == straight alpha
    let data = pixmap.data().to_vec();
    let rgba_image =
        image::RgbaImage::from_raw(SIZE, SIZE, data).expect("image dimensions mismatch");
    DynamicImage::ImageRgba8(rgba_image)
}
