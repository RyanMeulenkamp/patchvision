use std::cmp::max;

use grid::Grid;

use crate::image::Image;
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
        Template {
            grid
        }
    }

    pub fn render(self, theme: Box<dyn Theme>) -> String {
        let image: Image = (self, theme).into();
        format!("{}", image)
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

        Self {
            grid: result
        }
    }

    pub(crate) fn _overlay_all(all: Vec<Template>) -> Self {
        all.into_iter()
            .reduce(|acc, template| acc.overlay(template))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::balloon::Balloon;
    use crate::template::Template;
    use crate::theme;

    #[test]
    fn test_render() {
        println!(
            "{}",
            Balloon::_right("Paarden".into(), 2, 0)
                .unwrap()
                .pre_render()
                .render(Box::new(theme::Default {}))
        );
        println!(
            "{}",
            Balloon::new("Ferkels".into(), 4, 1, 1)
                .unwrap()
                .pre_render()
                .render(Box::new(theme::Default {}))
        );
    }

    #[test]
    fn test_overlay() {
        println!(
            "{}",
            Template::_overlay_all(
                vec![
                    Balloon::new("Paarden".into(), 0, 0, 1),
                    Balloon::new("Ferkels".into(), 2, 1, 3),
                    Balloon::new("Johans".into(), 3, 2, 2),
                    Balloon::_right("Bacon is good for me".into(), 4, 0),
                    Balloon::new("Lorum ipsum jonge".into(), 5, 1, 1),
                ].into_iter()
                    .map(|balloon| balloon.unwrap().pre_render())
                    .collect()
            ).render(Box::new(theme::Default {})),
        );
        println!(
            "{}",
            Balloon::_right("Gekkenhuis joh hier".into(), 2, 0)
                .unwrap()
                .pre_render()
                .overlay(
                    Balloon::_right("In petersburg is poardenmarkt".into(), 3, 1)
                        .unwrap()
                        .pre_render()
                )
                .render(Box::new(theme::Default {}))
        );
    }
}
