use std::str::FromStr;

impl Rgb {
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn to_u8(self) -> image::Rgb<u8> {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        image::Rgb([r, g, b])
    }

    pub fn distance_to(self, b: Rgb) -> f32 {
        let Rgb {
            r: r1,
            g: g1,
            b: b1,
        } = self;

        let Rgb {
            r: r2,
            g: g2,
            b: b2,
        } = b;

        ((r2 - r1).powi(2) + (g2 - g1).powi(2) + (b2 - b1).powi(2)).sqrt()
    }

    pub fn from_hex(s: &str) -> Option<Self> {
        // 01234567
        // --.
        //   --.
        //     --.

        let rgb = u32::from_str_radix(s, 16).ok()?;
        let [_, r, g, b] = rgb.to_be_bytes();

        Some(Self::from_u8(r, g, b))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
