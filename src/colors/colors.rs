use std::collections::HashMap;

use image::{DynamicImage, EncodableLayout};

use super::structs::Color;

pub fn get_palette(image: DynamicImage) -> Vec<Color> {
    let mut colors: HashMap<Color, u32> = HashMap::new();
    let resized_img = image.resize(30, 30, image::imageops::FilterType::Nearest);
    let bytes = resized_img.into_rgb8();
    let bytes = bytes.as_bytes();

    bytes.chunks(3).for_each(|slice| {
        let color: Color = slice.into();
        if !colors.contains_key(&color) && colors.iter().all(|(c, _)| !c.is_similar(color, 30.0)) {
            colors.insert(color, 1);
        } else if colors.contains_key(&color) {
            if let Some(count) = colors.get(&color) {
                colors.insert(color, count + 1);
            };
        }
    });

    let mut colors: Vec<(Color, u32)> = colors.into_iter().collect();
    colors.sort_by(|(_, a), (_, b)| b.cmp(a));

    return colors.into_iter().map(|(c, _)| c).collect();
}
