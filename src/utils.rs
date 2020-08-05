use sdl2::pixels::Color;

pub fn blend_alpha(fg: Color, bg: Color) -> Color {
    let a1 = fg.a as f32 / 255.0;
    let a2 = bg.a as f32 / 255.0;
    let a = a1 + a2 * (1.0 - a1);
    let r = fg.r as f32 * a + bg.r as f32 * (1.0 - a);
    let g = fg.g as f32 * a + bg.g as f32 * (1.0 - a);
    let b = fg.b as f32 * a + bg.b as f32 * (1.0 - a);
    // println!("blend {} {} {} {}", r, g, b, a);
    Color::RGBA(r as u8, g as u8, b as u8, (a * 255.0) as u8)
}
