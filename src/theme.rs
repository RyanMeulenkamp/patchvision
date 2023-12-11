use crate::panel::Panel;
use crate::placeholder::{Color, PlaceHolder};
use crate::slot::Slot;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

trait AppendMultiline {
    fn append_multiline(&self, other: String) -> String;
}

impl AppendMultiline for &str {
    fn append_multiline(&self, other: String) -> String {
        self.lines()
            .zip(other.lines())
            .map(|(a, b)| format!("{}{}", a, b))
            .intersperse("\n".into())
            .fold(String::new(), |string, line| format!("{}{}", string, line))
    }
}

impl AppendMultiline for String {
    fn append_multiline(&self, other: String) -> String {
        self.as_str().append_multiline(other)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum GenericTheme {
    ASCII,
    Rounded,
}

impl GenericTheme {
    pub(crate) fn new(&self) -> Box<dyn Theme> {
        match self {
            GenericTheme::ASCII => Box::new(DefaultTheme::new()),
            GenericTheme::Rounded => Box::new(RoundedTheme::new()),
        }
    }
}

pub(crate) trait Theme {
    fn render(&self, placeholder: PlaceHolder) -> String;

    fn render_slot(&self, slot: &Slot, index: usize) -> String;

    fn render_panel(&self, panel: &Panel) -> String;

    fn style_group(&mut self, group: &str) -> Color;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DefaultTheme {
    groups: HashSet<String>,
}

impl Default for DefaultTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultTheme {
    const LEFT: &'static str = include_str!("../resources/default/left.txt");
    const RIGHT: &'static str = include_str!("../resources/default/right.txt");
    const SEPARATOR: &'static str = include_str!("../resources/default/separator.txt");
    const SLOT: &'static str = include_str!("../resources/default/slot.txt");

    pub fn new() -> Self {
        Self {
            groups: HashSet::new(),
        }
    }
}

impl Theme for DefaultTheme {
    fn render(&self, placeholder: PlaceHolder) -> String {
        match placeholder {
            PlaceHolder::Text(c, color) => {
                return c
                    .to_string()
                    .color(colored::Color::TrueColor {
                        r: color.red,
                        g: color.green,
                        b: color.blue,
                    })
                    .to_string()
            }
            PlaceHolder::None | PlaceHolder::Padding => ' ',
            PlaceHolder::North | PlaceHolder::South => '─',
            PlaceHolder::East | PlaceHolder::West => '│',
            PlaceHolder::NorthEast | PlaceHolder::TransitionLeft => '┐',
            PlaceHolder::NorthWest | PlaceHolder::TransitionRight => '┌',
            PlaceHolder::SouthEast => '┘',
            PlaceHolder::SouthWest => '└',
            PlaceHolder::TransitionLeftEdge | PlaceHolder::TransitionRightEdge => '│',
            PlaceHolder::ArrowLeft | PlaceHolder::ArrowRight => '│',
            PlaceHolder::ArrowOverlayNorthLeft | PlaceHolder::ArrowOverlayNorthRight => '┴',
            PlaceHolder::ArrowOverlaySouthLeft | PlaceHolder::ArrowOverlaySouthRight => '┬',
        }
        .to_string()
    }

    fn render_slot(&self, slot: &Slot, index: usize) -> String {
        let mut base = "\n".repeat(DefaultTheme::LEFT.lines().count());
        if index != 0 {
            base = Self::SEPARATOR.into();
        }
        let charset = if let Slot::Free = slot {
            [" ", " ", "─", "─", "─", "─"]
        } else {
            ["│", "│", "┤", "├", "┘", "└"]
        };
        base.append_multiline(
            Self::SLOT
                .replace("<left>", charset[0])
                .replace("<right>", charset[1])
                .replace("<topleft>", charset[2])
                .replace("<topright>", charset[3])
                .replace("<bottomleft>", charset[4])
                .replace("<bottomright>", charset[5])
                .replace("<padding>", " ")
                .replace("<slotindex>", format!("{:02}", index).as_str()),
        )
    }

    fn render_panel(&self, panel: &Panel) -> String {
        panel
            .slots
            .iter()
            .enumerate()
            .map(|(index, slot)| self.render_slot(slot, index))
            .fold(Self::LEFT.into(), |left: String, slot| {
                left.append_multiline(slot)
            })
            .append_multiline(Self::RIGHT.into())
    }

    fn style_group(&mut self, group: &str) -> Color {
        self.groups.insert(group.to_string());
        match self.groups.len() - 1 {
            0 => Color {
                red: 255,
                green: 0,
                blue: 0,
            },
            1 => Color {
                red: 255,
                green: 128,
                blue: 0,
            },
            2 => Color {
                red: 255,
                green: 255,
                blue: 0,
            },
            3 => Color {
                red: 0,
                green: 255,
                blue: 0,
            },
            4 => Color {
                red: 64,
                green: 128,
                blue: 255,
            },
            5 => Color {
                red: 255,
                green: 0,
                blue: 255,
            },
            6 => Color {
                red: 0,
                green: 255,
                blue: 255,
            },
            7 => Color {
                red: 255,
                green: 255,
                blue: 255,
            },
            _ => Color {
                red: 192,
                green: 192,
                blue: 192,
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoundedTheme {
    delegate: DefaultTheme,
}

impl Default for RoundedTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundedTheme {
    const LEFT: &'static str = include_str!("../resources/rounded/left.txt");

    const RIGHT: &'static str = include_str!("../resources/rounded/right.txt");

    pub fn new() -> Self {
        Self {
            delegate: DefaultTheme::new(),
        }
    }
}

/// This one doesn't seem to work on Linux terminals unfortunately
impl Theme for RoundedTheme {
    fn render(&self, placeholder: PlaceHolder) -> String {
        match placeholder {
            PlaceHolder::Text(c, color) => {
                return c
                    .to_string()
                    .color(colored::Color::TrueColor {
                        r: color.red,
                        g: color.green,
                        b: color.blue,
                    })
                    .to_string()
            }
            PlaceHolder::None | PlaceHolder::Padding => ' ',
            PlaceHolder::North | PlaceHolder::South => '─',
            PlaceHolder::East | PlaceHolder::West => '│',
            PlaceHolder::NorthEast | PlaceHolder::TransitionLeft => '┐',
            PlaceHolder::NorthWest | PlaceHolder::TransitionRight => '┌',
            PlaceHolder::SouthEast => '┘',
            PlaceHolder::SouthWest => '└',
            PlaceHolder::TransitionLeftEdge | PlaceHolder::TransitionRightEdge => '│',
            PlaceHolder::ArrowLeft | PlaceHolder::ArrowRight => '│',
            PlaceHolder::ArrowOverlayNorthLeft | PlaceHolder::ArrowOverlayNorthRight => '┴',
            PlaceHolder::ArrowOverlaySouthLeft | PlaceHolder::ArrowOverlaySouthRight => '┬',
        }
        .to_string()
    }

    fn render_slot(&self, slot: &Slot, index: usize) -> String {
        self.delegate.render_slot(slot, index)
    }

    fn render_panel(&self, panel: &Panel) -> String {
        panel
            .slots
            .iter()
            .enumerate()
            .map(|(index, slot)| self.render_slot(slot, index))
            .fold(Self::LEFT.into(), |left: String, slot| {
                left.append_multiline(slot)
            })
            .append_multiline(Self::RIGHT.into())
    }

    fn style_group(&mut self, group: &str) -> Color {
        self.delegate.style_group(group)
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::AppendMultiline;

    #[test]
    fn test_multiline_append() {
        let left = "a\nb\nc";
        let right = "1\n2\n3";
        assert_eq!("a1\nb2\nc3", left.append_multiline(right.into()))
    }
}
