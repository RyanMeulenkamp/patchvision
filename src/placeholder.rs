use std::cmp::{max, min};

///
/// All of these are theme specific, except for text.
/// The order determines which one should be drawn in case of overlap.
/// The important distinctions here:
///
///  - All goes over None
///  - All except none go over Arrows
///
/// For overlap of South or North and arrow, specific behavior is implemented.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum PlaceHolder {
    /// Empty space
    None,

    /// Padding, which is empty space withing a balloon
    Padding,

    /// Arrow sides
    ArrowLeft, ArrowRight,

    /// Balloon sides
    North, East, South, West,

    /// Balloon corners
    NorthEast, SouthEast, SouthWest, NorthWest,

    /// Balloon transitions (betweeen balloon and arrow)
    TransitionLeft, TransitionRight,

    /// Balloon transition edges (between balloon and arrow)
    TransitionLeftEdge, TransitionRightEdge,

    /// Overlays between arrow and balloon
    ArrowOverlayNorthLeft, ArrowOverlayNorthRight, ArrowOverlaySouthLeft, ArrowOverlaySouthRight,

    /// Text
    Text(char),
}

impl Default for PlaceHolder {
    fn default() -> Self {
        PlaceHolder::None
    }
}

impl PlaceHolder {
    pub(crate) fn overlay(&self, another: &PlaceHolder) -> PlaceHolder {
        let front = max(self, another);
        let back = min(self, another);

        match [front, back] {
            [PlaceHolder::North, PlaceHolder::ArrowLeft] => PlaceHolder::ArrowOverlayNorthLeft,
            [PlaceHolder::North, PlaceHolder::ArrowRight] => PlaceHolder::ArrowOverlayNorthRight,
            [PlaceHolder::South, PlaceHolder::ArrowLeft] => PlaceHolder::ArrowOverlaySouthLeft,
            [PlaceHolder::South, PlaceHolder::ArrowRight] => PlaceHolder::ArrowOverlaySouthRight,
            [PlaceHolder::None, _] => *back,
            [_, PlaceHolder::None] => *front,
            [PlaceHolder::Padding, _] => *front,
            [_, PlaceHolder::Padding] => *back,
            _ => *front
        }
    }

}
