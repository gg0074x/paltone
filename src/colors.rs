use std::collections::HashMap;

use image::{DynamicImage, EncodableLayout};

pub fn get_palette(image: &DynamicImage, tolerance: f32) -> Vec<Color> {
    let mut colors: HashMap<Color, u32> = HashMap::new();
    let resized_img = image.resize(30, 30, image::imageops::FilterType::Nearest);
    let bytes = resized_img.into_rgb8();
    let bytes = bytes.as_bytes();

    bytes.chunks(3).for_each(|slice| {
        let color: Color = slice.into();
        if colors.contains_key(&color) {
            if let Some(count) = colors.get(&color) {
                colors.insert(color, count + 1);
            }
        } else {
            colors.insert(color, 1);
        }
    });

    let mut colors: Vec<(Color, u32)> = colors.into_iter().collect();
    colors.sort_by(|(_, a), (_, b)| b.cmp(a));

    let mut filtered = Vec::new();

    for (color, count) in &colors {
        if !filtered
            .iter()
            .any(|(c, _): &(Color, u32)| c.is_similar(*color, tolerance))
        {
            filtered.push((*color, *count));
        }
    }

    colors = filtered;

    colors.into_iter().map(|(c, _)| c).collect()
}

use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn hue(self) -> f32 {
        let (r, g, b) = (
            f32::from(self.0) / 255.0,
            f32::from(self.1) / 255.0,
            f32::from(self.2) / 255.0,
        );

        let max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            }
            color.2
        };

        let min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            }
            color.2
        };

        let (max, min) = (max((r, g, b)), min((r, g, b)));

        let hue = if max.to_bits() == r.to_bits() {
            (g - b) / (max - min)
        } else if max.to_bits() == g.to_bits() {
            2.0 + (b - r) / (max - min)
        } else if max.to_bits() == b.to_bits() {
            4.0 + (r - g) / (max - min)
        } else {
            0.0
        };

        let mut hue = if hue.is_nan() { 0.0 } else { hue * 60.0 };

        if hue < 0.0 {
            hue += 360.0;
        }
        hue
    }

    pub fn luminance(self) -> f32 {
        let (r, g, b) = (
            f32::from(self.0) / 255.0,
            f32::from(self.1) / 255.0,
            f32::from(self.2) / 255.0,
        );

        let max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            }
            color.2
        };

        let min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            }
            color.2
        };

        let (max, min) = (max((r, g, b)), min((r, g, b)));

        ((max - min) / 2.0) * 100.0
    }

    pub fn saturation(self) -> f32 {
        let (r, g, b) = (
            f32::from(self.0) / 255.0,
            f32::from(self.1) / 255.0,
            f32::from(self.2) / 255.0,
        );

        let max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            }
            color.2
        };

        let min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            }
            color.2
        };

        let delta = max((r, g, b)) - min((r, g, b));
        if delta == 0.0 {
            return 0.0;
        }
        (delta / (1.0 - (2.0 * (self.luminance() / 100.0) - 1.0).abs())) * 100.0
    }

    pub fn relative_luminance(self) -> f32 {
        let (r, g, b) = (
            f32::from(self.0) / 255.0,
            f32::from(self.1) / 255.0,
            f32::from(self.2) / 255.0,
        );
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn is_similar(self, other: Color, tolerance: f32) -> bool {
        let hue = self.hue();
        let lum = self.luminance();
        let sat = self.saturation();
        let other_hue = other.hue();
        let other_lum = other.luminance();
        let other_sat = other.saturation();

        let hue_diff = (hue - other_hue).abs();
        let hue_diff = hue_diff.min(360.0 - hue_diff) / 180.0;

        let lum_diff = (lum - other_lum).abs() / 50.0;

        let sat_diff = (sat - other_sat).abs() / 100.0;

        let distance = hue_diff * 0.6 + lum_diff * 0.3 + sat_diff * 0.1;

        (distance * 50.0) <= tolerance
    }
}

impl From<&[u8]> for Color {
    fn from(value: &[u8]) -> Self {
        Color(value[0], value[1], value[2])
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Color", 4)?;
        s.serialize_field("r", &self.0)?;
        s.serialize_field("g", &self.1)?;
        s.serialize_field("b", &self.2)?;
        s.serialize_field(
            "hex",
            &format!("{:02X}{:02X}{:02X}", &self.0, &self.1, &self.2),
        )?;
        s.end()
    }
}
