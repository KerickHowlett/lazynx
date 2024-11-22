use color_eyre::Result;
use derive_deref::{Deref, DerefMut};
use lazy_static::lazy_static;
use ratatui::style::{Color, Modifier, Style};
use serde::{de::Deserializer, Deserialize};
use std::{collections::HashMap, default::Default, hash::Hash};

lazy_static! {
    pub static ref COLOR_PRESET: HashMap<String, Color> = HashMap::from([
        (String::from("bold black"), Color::Indexed(8)),
        (String::from("bold red"), Color::Indexed(9)),
        (String::from("bold green"), Color::Indexed(10)),
        (String::from("bold yellow"), Color::Indexed(11)),
        (String::from("bold blue"), Color::Indexed(12)),
        (String::from("bold magenta"), Color::Indexed(13)),
        (String::from("bold cyan"), Color::Indexed(14)),
        (String::from("bold white"), Color::Indexed(15)),
        (String::from("black"), Color::Indexed(0)),
        (String::from("red"), Color::Indexed(1)),
        (String::from("green"), Color::Indexed(2)),
        (String::from("yellow"), Color::Indexed(3)),
        (String::from("blue"), Color::Indexed(4)),
        (String::from("magenta"), Color::Indexed(5)),
        (String::from("cyan"), Color::Indexed(6)),
        (String::from("white"), Color::Indexed(7)),
    ]);
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Styles<TMode>(pub HashMap<TMode, HashMap<String, Style>>);

impl<'de, TMode: Deserialize<'de> + Eq + Hash> Deserialize<'de> for Styles<TMode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed_map = HashMap::<TMode, HashMap<String, String>>::deserialize(deserializer)?;

        let styles = parsed_map
            .into_iter()
            .map(|(mode, inner_map)| {
                let converted_inner_map = inner_map
                    .into_iter()
                    .map(|(str, style)| (str, parse_style(&style)))
                    .collect();
                (mode, converted_inner_map)
            })
            .collect();

        Ok(Styles(styles))
    }
}

pub fn parse_style(line: &str) -> Style {
    let (foreground, background) =
        line.split_at(line.to_lowercase().find("on ").unwrap_or(line.len()));
    let foreground = process_color_string(foreground);
    let background = process_color_string(&background.replace("on ", ""));

    let mut style = Style::default();
    if let Some(fg) = parse_color(&foreground.0) {
        style = style.fg(fg);
    }
    if let Some(bg) = parse_color(&background.0) {
        style = style.bg(bg);
    }
    style = style.add_modifier(foreground.1 | background.1);
    style
}

fn process_color_string(color_str: &str) -> (String, Modifier) {
    let color = color_str
        .replace("grey", "gray")
        .replace("bright ", "")
        .replace("bold ", "")
        .replace("underline ", "")
        .replace("inverse ", "");

    let mut modifiers = Modifier::empty();
    if color_str.contains("underline") {
        modifiers |= Modifier::UNDERLINED;
    }
    if color_str.contains("bold") {
        modifiers |= Modifier::BOLD;
    }
    if color_str.contains("inverse") {
        modifiers |= Modifier::REVERSED;
    }

    (color, modifiers)
}

fn parse_color(color_option: &str) -> Option<Color> {
    let color = color_option.trim_start().trim_end();

    if color.contains("bright color") {
        let s = color.trim_start_matches("bright ");
        let c = s
            .trim_start_matches("color")
            .parse::<u8>()
            .unwrap_or_default();

        return Some(Color::Indexed(c.wrapping_shl(8)));
    }

    if color.contains("color") {
        let c = color
            .trim_start_matches("color")
            .parse::<u8>()
            .unwrap_or_default();

        return Some(Color::Indexed(c));
    }

    if color.contains("gray") {
        let c = 232
            + color
                .trim_start_matches("gray")
                .parse::<u8>()
                .unwrap_or_default();

        return Some(Color::Indexed(c));
    }

    if color.contains("rgb") {
        let red = (color.as_bytes()[3] as char)
            .to_digit(10)
            .unwrap_or_default() as u8;
        let green = (color.as_bytes()[4] as char)
            .to_digit(10)
            .unwrap_or_default() as u8;
        let blue = (color.as_bytes()[5] as char)
            .to_digit(10)
            .unwrap_or_default() as u8;
        let c = 16 + red * 36 + green * 6 + blue;

        return Some(Color::Indexed(c));
    }

    if COLOR_PRESET.contains_key(color) {
        return Some(COLOR_PRESET[color]);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_style_default() {
        let style = parse_style("");
        assert_eq!(style, Style::default());
    }

    #[test]
    fn test_parse_style_foreground() {
        let style = parse_style("red");
        assert_eq!(style.fg, Some(Color::Indexed(1)));
    }

    #[test]
    fn test_parse_style_background() {
        let style = parse_style("on blue");
        assert_eq!(style.bg, Some(Color::Indexed(4)));
    }

    #[test]
    fn test_parse_style_modifiers() {
        let style = parse_style("underline red on blue");
        assert_eq!(style.fg, Some(Color::Indexed(1)));
        assert_eq!(style.bg, Some(Color::Indexed(4)));
    }

    #[test]
    fn test_process_color_string() {
        let (color, modifiers) = process_color_string("underline bold inverse gray");
        assert_eq!(color, "gray");
        assert!(modifiers.contains(Modifier::UNDERLINED));
        assert!(modifiers.contains(Modifier::BOLD));
        assert!(modifiers.contains(Modifier::REVERSED));
    }

    #[test]
    fn test_parse_color_rgb() {
        let color = parse_color("rgb123");
        let expected = 16 + 36 + 2 * 6 + 3;
        assert_eq!(color, Some(Color::Indexed(expected)));
    }

    #[test]
    fn test_parse_color_unknown() {
        let color = parse_color("unknown");
        assert_eq!(color, None);
    }
}
