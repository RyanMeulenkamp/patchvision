use crate::placeholder::PlaceHolder;

pub trait Theme {

    fn render(&self, placeholder: PlaceHolder) -> char {
        match placeholder {
            PlaceHolder::Text(c) => c,
            PlaceHolder::None | PlaceHolder::Padding => ' ',
            PlaceHolder::North | PlaceHolder::South => '─',
            PlaceHolder::East | PlaceHolder::West => '│',
            PlaceHolder::NorthEast | PlaceHolder::TransitionLeft => '╮',
            PlaceHolder::NorthWest | PlaceHolder::TransitionRight => '╭',
            PlaceHolder::SouthEast => '╯',
            PlaceHolder::SouthWest => '╰',
            PlaceHolder::TransitionLeftEdge | PlaceHolder::TransitionRightEdge => '│',
            PlaceHolder::ArrowLeft | PlaceHolder::ArrowRight => '│',
            PlaceHolder::ArrowOverlayNorthLeft | PlaceHolder::ArrowOverlayNorthRight => '┴',
            PlaceHolder::ArrowOverlaySouthLeft | PlaceHolder::ArrowOverlaySouthRight => '┬',
        }
    }
}

pub struct Default {}

impl Theme for Default {

}
