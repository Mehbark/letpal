use crate::rgb::Rgb;

#[derive(Debug)]
pub struct Palette {
    colors: Vec<Rgb>,
}

impl Palette {
    pub fn new(colors: Vec<Rgb>) -> Self {
        assert!(!colors.is_empty());

        Self { colors }
    }

    pub fn from_palette_text(p: &str) -> Self {
        let colors = p.lines().filter_map(Rgb::from_hex).collect();

        Self { colors }
    }

    // pub fn closest_color(&self, color: Rgb) -> &Rgb {
    //     self.memo
    //         .entry(color)
    //         .or_insert(self.closest_color_slow(color))
    // }

    pub fn closest_color(&self, color: Rgb) -> &Rgb {
        self.colors
            .iter()
            .min_by_key(|&&c| (color.distance_to(c) * 1000.0) as u32)
            .expect("i mean come on put more than one color")
    }
}
