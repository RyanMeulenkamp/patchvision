use std::fmt::{Display, Formatter};

use serde::{de, Deserialize, Deserializer, Serializer};
use serde::de::{SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq};

use crate::balloon::{Balloon, max_shift};
use crate::slot::Slot;
use crate::template::Template;

const LEFT: &'static str = r###" ┌───
 │O ┌
 │  │
 │O └
 └───"###;

const RIGHT: &'static str = r###"───┐
┐ O│
│  │
┘ O│
───┘"###;

const SEPARATOR: &'static str = r###"─
╥
║
╨
─"###;

const SLOT: &'static str = r###"<topleft><topright>
<bottomleft><bottomright>
<slotindex>
──
──"###;

const MAX_ROWS: usize = 6;

impl<'a, const SLOTS: usize> Serialize for Panel<'a, SLOTS> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut slots = serializer.serialize_seq(Some(SLOTS))?;
        for slot in self.clone().slots.iter_mut() {
            slots.serialize_element(slot)?;
        }
        slots.end()
    }
}



struct PanelVisitor<const SLOTS: usize>;

impl<'a, const SLOTS: usize> Visitor<'a> for PanelVisitor<SLOTS> {
    type Value = Panel<'a, SLOTS>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a list of slots")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'a> {
        let mut slots: Vec<Slot<'a>> = vec![];
        while let Some(slot) = seq.next_element()? {
            slots.push(slot);
        }
        Ok(Panel {
            slots: slots.try_into().map_err(|_| de::Error::custom("Not the expected size.") )?
        })
    }
}

impl<'a, const SLOTS: usize> Deserialize<'a> for Panel<'a, SLOTS> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        deserializer.deserialize_seq(PanelVisitor::<SLOTS>)
    }
}

#[derive(Copy, Clone)]
pub struct Panel<'a, const SLOTS: usize> {
    pub slots: [Slot<'a>; SLOTS],
}

trait AppendMultiline {
    fn append_multiline(&self, other: String) -> String;
}

impl AppendMultiline for &str {
    fn append_multiline(&self, other: String) -> String {
        self.lines()
            .zip(other.lines())
            .map(|(a, b)| format!("{}{}", a, b))
            .intersperse("\n".into())
            .fold(
                String::new(),
                |string, line| format!("{}{}", string, line),
            )
    }
}

impl AppendMultiline for String {
    fn append_multiline(&self, other: String) -> String {
        self.as_str().append_multiline(other)
    }
}

impl<'a, const SLOTS: usize> Panel<'a, SLOTS> {
    fn render_slot(&self, index: usize) -> String {
        let mut base = "\n".repeat(5);
        if index != 0 {
            base = SEPARATOR.into();
        }
        let charset = if let Slot::Free = self.slots[index] {
            ["─"; 4]
        } else {
            ["┤", "├", "┘", "└"]
        };
        base.append_multiline(
            SLOT
                .replace("<topleft>", charset[0])
                .replace("<topright>", charset[1])
                .replace("<bottomleft>", charset[2])
                .replace("<bottomright>", charset[3])
                .replace("<slotindex>", format!("{:02}", index).as_str())
        )
    }

    pub fn render_panel(&self) -> String {
        (0..SLOTS).into_iter()
            .map(|slot| self.render_slot(slot))
            .fold(LEFT.into(), |left: String, slot| left.append_multiline(slot))
            .append_multiline(RIGHT.into())
    }

    fn shift(index: usize, text: &str, row: usize, stack: &mut Vec<Balloon>) -> bool {
        let previous = stack.last().unwrap();
        for shift in (0..=max_shift(text, index)).rev() {
            let attempt = Balloon::new(text.to_string(), index, row, shift);
            if let Ok(balloon) = attempt {
                if !previous.overlaps(&balloon) && balloon.end() <= Balloon::WIDTH {
                    stack.push(balloon);
                    return true;
                }
            }
        }
        false
    }

    fn place(grid: &mut Vec<Vec<Balloon>>, index: usize, text: &str) {
        for row in 0..MAX_ROWS {
            match grid.get_mut(row) {
                None => {
                    grid.push(vec![Balloon::left(text.to_string(), index, row).unwrap()]);
                    return;
                }
                Some(stack) => {
                    if Self::shift(index, text, row, stack) {
                        return;
                    }
                }
            }
        }
    }

    pub fn layout(&self) -> Template {
        let mut grid: Vec<Vec<Balloon>> = Default::default();
        for (index, slot) in self.slots.iter().enumerate() {
            if let Slot::Occupied { text, .. } = slot {
                Self::place(&mut grid, index, text);
            }
        }
        grid.iter()
            .flatten()
            .fold(
                Template::default(),
                |template, balloon| template.overlay(balloon.pre_render())
            )
    }
}

impl<'a, const SLOTS: usize> Default for Panel<'a, SLOTS> {
    fn default() -> Self {
        Self {
            slots: [Slot::Free; SLOTS]
        }
    }
}

impl<'a, const SLOTS: usize> From<[&str; SLOTS]> for Panel<'a, SLOTS> {
    fn from(_map: [&str; SLOTS]) -> Self {
        todo!()
    }
}

impl<'a, const SLOTS: usize> Display for Panel<'a, SLOTS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let balloons = self.slots
        //     .into_iter()
        //     .flatten()
        //     .map(Balloon::pre_render)
        //     .reduce(Template::overlay)
        //     .map(|template| template.render(Box::new(theme::Default{})))
        //     .unwrap_or("".into());

        // write!(f, "{}\n{}", balloons, self.render_panel())
        write!(f, "{}", self.render_panel())
    }
}

// impl From<[Slot; 24]> for Vec<Balloon> {
//     fn from(slots: [Slot; 24]) -> Self {
//         slots.iter()
//             .enumerate()
//             .flat_map()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::panel::AppendMultiline;

    #[test]
    fn test_multiline_append() {
        let left = "a\nb\nc";
        let right = "1\n2\n3";
        assert_eq!("a1\nb2\nc3", left.append_multiline(right.into()))
    }
}
