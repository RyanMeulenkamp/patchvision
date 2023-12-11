use grid::Grid;
use std::ops::Add;

use crate::placeholder::{Color, PlaceHolder};
use crate::round::Round;
use crate::template::Template;

#[derive(PartialEq, Eq)]
pub(crate) struct ProtoBalloon {
    color: Color,
    text: String,
    slot: usize,
}

#[derive(PartialEq, Eq)]
pub(crate) struct Balloon {
    proto: ProtoBalloon,
    row: usize,
    shift: usize,
}

impl ProtoBalloon {
    pub(crate) fn inner_width(&self) -> usize {
        inner_width(&self.text)
    }

    pub(crate) fn width(&self) -> usize {
        width(&self.text)
    }

    fn padding(&self) -> usize {
        self.width() - self.inner_width() - 2
    }

    pub(crate) fn left_padding(&self) -> usize {
        self.padding() / 2
    }

    pub(crate) fn right_padding(&self) -> usize {
        self.padding() - self.left_padding()
    }

    pub(crate) fn arrow(&self) -> usize {
        self.slot * 9 + 16
    }

    pub(crate) fn max_shift(&self) -> usize {
        max_shift(&self.text, self.slot)
    }
}

pub(crate) fn inner_width(text: &str) -> usize {
    text.chars().count()
}

pub(crate) fn width(text: &str) -> usize {
    inner_width(text).up(3).add(4)
}

pub(crate) fn max_shift(text: &str, slot: usize) -> usize {
    slot.add(1).min(width(text) / 9)
}

impl Balloon {
    pub(crate) const WIDTH: usize = 240;
    pub(crate) const RANGE: usize = 23;

    fn from_proto(proto: ProtoBalloon, row: usize, shift: usize) -> Result<Balloon, String> {
        if shift > proto.max_shift() {
            return Err(format!(
                "Shift {} is too large for balloon {}",
                shift, proto.text
            ));
        }
        Ok(Balloon { proto, row, shift })
    }

    pub fn new(
        color: Color,
        text: String,
        slot: usize,
        row: usize,
        shift: usize,
    ) -> Result<Balloon, String> {
        if slot > Self::RANGE {
            return Err(format!("{} is not in a valid slot!", slot));
        }
        Self::from_proto(ProtoBalloon { color, text, slot }, row, shift)
    }

    pub fn _right(color: Color, text: String, slot: usize, row: usize) -> Result<Balloon, String> {
        Self::new(color, text, slot, row, 0)
    }

    pub(crate) fn left(
        color: Color,
        text: String,
        slot: usize,
        row: usize,
    ) -> Result<Balloon, String> {
        let shift = max_shift(&text, slot);
        Self::new(color, text, slot, row, shift)
    }

    pub(crate) fn x(&self) -> usize {
        self.proto.arrow() - self.shift * 9 - 1
    }

    pub(crate) fn y(&self) -> usize {
        self.row * 3
    }

    pub(crate) fn _height(&self) -> usize {
        self.y() + 3
    }

    pub(crate) fn start(&self) -> usize {
        self.x()
    }

    pub(crate) fn end(&self) -> usize {
        self.x() + self.proto.width()
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        let (left, right) = if self.x() < other.x() {
            (self, other)
        } else {
            (other, self)
        };
        left.end() >= right.start()
    }

    pub fn pre_render(&self) -> Template {
        let mut grid = Grid::init(1, Self::WIDTH, PlaceHolder::None);
        let x = self.x();

        grid.pop_row();
        grid.push_row(
            [
                &[PlaceHolder::None].repeat(x)[..],
                &[PlaceHolder::NorthWest],
                &[PlaceHolder::North].repeat(self.proto.width() - 2)[..],
                &[PlaceHolder::NorthEast],
            ]
            .concat(),
        );

        grid.push_row(
            [PlaceHolder::None]
                .repeat(x)
                .into_iter()
                .chain([PlaceHolder::West])
                .chain([PlaceHolder::Padding].repeat(self.proto.left_padding()))
                .chain(
                    self.proto
                        .text
                        .chars()
                        .map(|c| PlaceHolder::Text(c, self.proto.color)),
                )
                .chain([PlaceHolder::Padding].repeat(self.proto.right_padding()))
                .chain([PlaceHolder::East])
                .collect(),
        );

        grid.push_row(
            [
                &[PlaceHolder::None].repeat(x)[..],
                &[PlaceHolder::SouthWest],
                &[PlaceHolder::South].repeat(self.proto.width() - 2)[..],
                &[PlaceHolder::SouthEast],
            ]
            .concat(),
        );

        let arrow = self.proto.arrow();

        match grid[2][arrow] {
            PlaceHolder::SouthWest => {
                grid[2][arrow] = PlaceHolder::TransitionLeft;
                grid[2][arrow + 1] = PlaceHolder::TransitionRightEdge;
            }
            PlaceHolder::SouthEast => {
                grid[2][arrow] = PlaceHolder::TransitionLeft;
                grid[2][arrow + 1] = PlaceHolder::TransitionRightEdge;
            }
            _ => {
                grid[2][arrow] = PlaceHolder::TransitionLeft;
                grid[2][arrow + 1] = PlaceHolder::TransitionRight;
            }
        }

        [[PlaceHolder::None]
            .repeat(arrow)
            .into_iter()
            .chain([PlaceHolder::ArrowLeft, PlaceHolder::ArrowRight])
            .chain([PlaceHolder::None].repeat(x + self.proto.width() - arrow - 2))
            .collect()]
        .into_iter()
        .cycle()
        .take(self.y())
        .for_each(|row| grid.push_row(row));

        Template::new(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::balloon::{max_shift, Balloon};
    use crate::placeholder::Color;

    #[test]
    fn test_max_shift() {
        assert_eq!(1, max_shift("a", 0));
        assert_eq!(1, max_shift("ab", 0));
        assert_eq!(1, max_shift("abc", 0));
        assert_eq!(1, max_shift("abcd", 0));
        assert_eq!(1, max_shift("a", 10));
        assert_eq!(1, max_shift("ab", 10));
        assert_eq!(1, max_shift("abc", 10));
        assert_eq!(2, max_shift("abcd", 10));
        assert_eq!(2, max_shift("abcde", 10));
        assert_eq!(2, max_shift("abcdef", 10));
        assert_eq!(3, max_shift("abcdefg", 10));
    }

    #[test]
    fn balloon_dimensions() {
        let balloon = Balloon::_right(
            Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            "".into(),
            3,
            1,
        )
        .unwrap();
        assert_eq!(4, balloon.proto.width());
        assert_eq!(6, balloon._height());
        assert_eq!(13, balloon.start());
        assert_eq!(17, balloon.end());
    }
}
