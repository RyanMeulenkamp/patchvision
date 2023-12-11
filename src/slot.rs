use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub enum Slot {
    Occupied { text: String, group: String },
    Free,
}
