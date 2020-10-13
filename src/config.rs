use sdl2::pixels::Color;

use serde::Deserialize;

const BRIGHT_GRAY: Color = Color::RGB(44, 50, 60);
const WHITE: Color = Color::RGB(255, 255, 255);
const BLUE: Color = Color::RGB(30, 144, 255);
const BROWN: Color = Color::RGB(205, 133, 63);
const DIM_GRAY: Color = Color::RGB(105, 105, 105);
const SLATE: Color = Color::RGB(47, 79, 79);

#[derive(Deserialize)]
struct ColorSchemeJson {
    pub buffer_fg: String,
    pub buffer_bg: String,
    pub cursor: String,
    pub statusline_fg: String,
    pub statusline_bg: String,
    pub filepicker_fg: String,
    pub filepicker_bg: String,
    pub filepicker_border: String,
    pub filepicker_selection: String,
    pub itempicker_fg: String,
    pub itempicker_bg: String,
    pub itempicker_border: String,
    pub itempicker_selection: String,
}

pub struct ColorScheme {
    pub buffer_fg: Color,
    pub buffer_bg: Color,
    pub cursor: Color,
    pub statusline_fg: Color,
    pub statusline_bg: Color,
    pub filepicker_fg: Color,
    pub filepicker_bg: Color,
    pub filepicker_border: Color,
    pub filepicker_selection: Color,
    pub itempicker_fg: Color,
    pub itempicker_bg: Color,
    pub itempicker_border: Color,
    pub itempicker_selection: Color,
}

pub const DEFAULT_CS: ColorScheme = ColorScheme {
    buffer_fg: WHITE,
    buffer_bg: BRIGHT_GRAY,
    cursor: BLUE,
    statusline_fg: WHITE,
    statusline_bg: BROWN,
    filepicker_fg: WHITE,
    filepicker_bg: DIM_GRAY,
    filepicker_border: SLATE,
    filepicker_selection: SLATE,
    itempicker_fg: WHITE,
    itempicker_bg: DIM_GRAY,
    itempicker_border: SLATE,
    itempicker_selection: SLATE,
};

impl ColorScheme {
    pub fn read_from_file(file_path: &String) -> Result<ColorScheme, String> {
        let file = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        let cs_json = serde_json::from_str::<ColorSchemeJson>(&file).map_err(|e| e.to_string())?;
        ColorScheme::json_to_cs(&cs_json)
    }

    fn json_to_cs(csj: &ColorSchemeJson) -> Result<ColorScheme, String> {
        Ok(ColorScheme {
            buffer_fg: string_to_hex_color(&csj.buffer_fg)?,
            buffer_bg: string_to_hex_color(&csj.buffer_bg)?,
            cursor: string_to_hex_color(&csj.cursor)?,
            statusline_bg: string_to_hex_color(&csj.statusline_bg)?,
            statusline_fg: string_to_hex_color(&csj.statusline_fg)?,
            filepicker_fg: string_to_hex_color(&csj.filepicker_fg)?,
            filepicker_bg: string_to_hex_color(&csj.filepicker_bg)?,
            filepicker_border: string_to_hex_color(&csj.filepicker_border)?,
            filepicker_selection: string_to_hex_color(&csj.filepicker_selection)?,
            itempicker_fg: string_to_hex_color(&csj.itempicker_fg)?,
            itempicker_bg: string_to_hex_color(&csj.itempicker_bg)?,
            itempicker_border: string_to_hex_color(&csj.itempicker_border)?,
            itempicker_selection: string_to_hex_color(&csj.itempicker_selection)?,
        })
    }
}

fn strip_hash(color: &str) -> &str {
    if color.starts_with('#') {
        &color[1..]
    } else {
        color
    }
}

fn string_to_hex_color(color: &str) -> Result<Color, String> {
    let nohash: &str = strip_hash(color);
    if nohash.len() == 6 {
        usize::from_str_radix(nohash, 16)
            .map(|num| {
                let r = (num >> 16) & 0xFF;
                let g = (num >> 8) & 0xFF;
                let b = num & 0xFF;
                Color::RGB(r as u8, g as u8, b as u8)
            })
            .map_err(|e| format!("Unable to decode {}, error {}", color, e))
    } else {
        Err(format!(
            "Expected 6 chars but got {} in {}",
            nohash.len(),
            nohash
        ))
    }
}

fn hex_to_string_color(color: &Color) -> String {
    format!("{:X}", (color.r << 16) + (color.g << 8) + color.b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_hex(s: &str) -> super::Color {
        string_to_hex_color(s).unwrap()
    }

    #[test]
    fn string_to_hex_ok_test() {
        assert_eq!(super::Color::RGB(90, 97, 122), str_hex("5a617a"));
        assert_eq!(super::Color::RGB(90, 97, 122), str_hex("#5a617a"));
        assert_eq!(super::Color::RGB(13, 28, 40), str_hex("0d1c28"));
        assert_eq!(super::Color::RGB(13, 28, 40), str_hex("0D1C28"));
        assert_eq!(super::Color::RGB(0, 0, 0), str_hex("000000"));
        assert_eq!(super::Color::RGB(255, 255, 255), str_hex("FFFFFF"));
    }

    #[test]
    #[should_panic(expected = "Unable to decode 5G617a, error invalid digit found")]
    fn string_to_hex_panic1_test() {
        assert_eq!(super::Color::RGB(90, 97, 122), str_hex("5G617a"));
    }

    #[test]
    #[should_panic(expected = "Expected 6 chars but got 9 in 123456789")]
    fn string_to_hex_panic2_test() {
        assert_eq!(super::Color::RGB(69, 103, 137), str_hex("123456789"));
    }

    #[test]
    fn hex_to_string_ok_test() {}
}
