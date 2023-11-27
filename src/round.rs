
pub(crate) trait Round: Copy {
    fn up(self, multiple: Self) -> Self;
    fn down(self, multiple: Self) -> Self;
}

impl Round for usize {
    fn up(self, multiple: Self) -> Self {
        if self > 0 {
            ((self + multiple - 1) / multiple) * multiple
        } else {
            (self / multiple) * multiple
        }
    }

    fn down(self, multiple: Self) -> Self {
        if self > 0 {
            (self / multiple) * multiple
        } else {
            ((self - multiple + 1) / multiple) * multiple
        }
    }
}
