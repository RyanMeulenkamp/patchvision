use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Slot<'a> {
    Occupied {text: &'a str, group: &'a str}, Free
}

impl<'a> IntoIterator for Slot<'a> {
    type Item = Slot<'a>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Slot::Occupied { .. } => vec![self],
            Slot::Free => vec![],
        }.into_iter()
    }
}
