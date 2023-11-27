use std::fmt::{Display, Formatter};

use crate::template::Template;
use crate::theme::Theme;

pub struct Image {
    template: Template,
    theme: Box<dyn Theme>,
}

impl From<(Template, Box<dyn Theme>)> for Image {
    fn from((template, theme): (Template, Box<dyn Theme>)) -> Self {
        Image {
            template, theme
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for row in 0..self.template.grid.rows() {
            string.push('\n');
            for placeholder in self.template.grid.iter_row(row) {
                string.push(self.theme.render(*placeholder));
            }
        }
        write!(f, "{}", string)
    }
}
