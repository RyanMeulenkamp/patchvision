use std::cmp::max;

use grid::Grid;

use crate::placeholder::PlaceHolder;
use crate::theme::Theme;

pub struct Template {
    pub grid: Grid<PlaceHolder>,
}

impl Default for Template {
    fn default() -> Self {
        Self::new(Grid::new(1, 1))
    }
}

impl Template {
    pub fn new(grid: Grid<PlaceHolder>) -> Self {
        Template { grid }
    }

    pub fn render(self, theme: &Box<dyn Theme>) -> String {
        let mut string = String::new();
        for row in 0..self.grid.rows() {
            string.push('\n');
            for placeholder in self.grid.iter_row(row) {
                string.push_str(&theme.render(*placeholder));
            }
        }
        string
    }

    pub(crate) fn overlay(self, another: Template) -> Self {
        let (one_grid, another_grid) = (self.grid, another.grid);
        let (longer, shorter) = if one_grid.rows() > another_grid.rows() {
            (one_grid, another_grid)
        } else {
            (another_grid, one_grid)
        };

        let rows = longer.rows();
        let diff = rows - shorter.rows();
        let cols = max(shorter.cols(), longer.cols());
        let mut result = longer.clone();

        if result.cols() < cols {
            for _ in 0..(cols - result.cols()) {
                result.push_col(vec![PlaceHolder::None; rows]);
            }
        }

        for row in diff..rows {
            for col in 0..cols {
                if let Some(placeholder) = shorter.get(row - diff, col) {
                    result[row][col] = result[row][col].overlay(placeholder);
                }
            }
        }

        Self { grid: result }
    }

    pub(crate) fn _overlay_all(all: Vec<Template>) -> Self {
        all.into_iter()
            .reduce(|acc, template| acc.overlay(template))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::balloon::{max_shift, Balloon};
    use crate::placeholder::Color;
    use crate::template::Template;

    use crate::theme::GenericTheme;

    const COLOR: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    #[test]
    fn test_render() {
        let theme = GenericTheme::ASCII.new();
        let text = "Paarden";
        for slot in 0..24 {
            for shift in 0..max_shift(text, slot) {
                println!(
                    "{}",
                    Balloon::new(COLOR, text.into(), slot, 0, shift)
                        .unwrap()
                        .pre_render()
                        .render(&theme)
                );
            }
        }
        println!(
            "{}",
            Balloon::new(COLOR, "Ferkels".into(), 4, 1, 1)
                .unwrap()
                .pre_render()
                .render(&theme)
        );
    }

    #[test]
    fn test_overlay() {
        let theme = GenericTheme::ASCII.new();
        println!(
            "{}",
            Template::_overlay_all(
                vec![
                    Balloon::new(COLOR, "Paarden".into(), 0, 0, 1),
                    Balloon::new(COLOR, "Ferkels".into(), 2, 1, 3),
                    Balloon::new(COLOR, "Johans".into(), 3, 2, 2),
                    Balloon::_right(COLOR, "Bacon is good for me".into(), 4, 0),
                    Balloon::new(COLOR, "Lorum ipsum jonge".into(), 5, 1, 1),
                ]
                .into_iter()
                .map(|balloon| balloon.unwrap().pre_render())
                .collect()
            )
            .render(&theme),
        );
        println!(
            "{}",
            Balloon::_right(COLOR, "Gekkenhuis joh hier".into(), 2, 0)
                .unwrap()
                .pre_render()
                .overlay(
                    Balloon::_right(COLOR, "In petersburg is poardenmarkt".into(), 3, 1)
                        .unwrap()
                        .pre_render()
                )
                .render(&theme)
        );
    }
}
