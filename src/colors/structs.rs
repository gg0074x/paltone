use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn get_hue(&self) -> f32 {
        let (r, g, b) = (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        );

        let get_max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let get_min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let (max, min) = (get_max((r, g, b)), get_min((r, g, b)));

        let hue = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else if max == b {
            4.0 + (r - g) / (max - min)
        } else {
            0.0
        };

        let mut hue = if hue.is_nan() { 0.0 } else { hue * 60.0 };

        if hue < 0.0 {
            hue += 360.0;
        }
        return hue;
    }

    pub fn get_lum(&self) -> f32 {
        let (r, g, b) = (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        );

        let get_max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let get_min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let (max, min) = (get_max((r, g, b)), get_min((r, g, b)));

        ((max - min) / 2.0) * 100.0
    }

    pub fn get_sat(&self) -> f32 {
        let (r, g, b) = (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        );

        let get_max = |color: (f32, f32, f32)| -> f32 {
            if color.0 >= color.1 && color.0 >= color.2 {
                return color.0;
            } else if color.1 >= color.0 && color.1 >= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let get_min = |color: (f32, f32, f32)| -> f32 {
            if color.0 <= color.1 && color.0 <= color.2 {
                return color.0;
            } else if color.1 <= color.0 && color.1 <= color.2 {
                return color.1;
            } else {
                return color.2;
            }
        };

        let delta = get_max((r, g, b)) - get_min((r, g, b));
        if delta == 0.0 {
            return 0.0;
        } else {
            return (delta / (1.0 - (2.0 * (self.get_lum() / 100.0) - 1.0).abs())) * 100.0;
        }
    }

    pub fn get_rel_lum(&self) -> f32 {
        let (r, g, b) = (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        );
        return 0.2126 * r + 0.7152 * g + 0.0722 * b;
    }

    pub fn is_similar(&self, other: Color, tolerance: f32) -> bool {
        let hue = self.get_hue();
        let lum = self.get_lum();
        let sat = self.get_sat();
        let other_hue = other.get_hue();
        let other_lum = other.get_lum();
        let other_sat = other.get_sat();

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
