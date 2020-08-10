use sdl2::pixels::Color as SdlColor;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn shading(&self, intensity: f32) -> Color {
        let Self { r, g, b, a } = *self;
        let r = r as f32 * intensity;
        let g = g as f32 * intensity;
        let b = b as f32 * intensity;
        let a = a as f32 * intensity;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: a as u8,
        }
    }

    pub fn blend_alpha(&self, bg: &Color) -> Color {
        let a1 = self.a as f32 / 255.0;
        let a2 = bg.a as f32 / 255.0;
        let a = a1 + a2 * (1.0 - a1);
        let r = self.r as f32 * a + bg.r as f32 * (1.0 - a);
        let g = self.g as f32 * a + bg.g as f32 * (1.0 - a);
        let b = self.b as f32 * a + bg.b as f32 * (1.0 - a);
        // println!("blend {} {} {} {}", r, g, b, a);
        Color::new(r as u8, g as u8, b as u8, (a * 255.0) as u8)
    }
}

impl Into<SdlColor> for Color {
    fn into(self) -> SdlColor {
        let Color { r, g, b, a } = self;
        SdlColor::RGBA(r, g, b, a)
    }
}
