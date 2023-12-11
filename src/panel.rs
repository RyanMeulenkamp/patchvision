use serde::Deserialize;

use crate::balloon::{max_shift, Balloon};
use crate::slot::Slot;
use crate::template::Template;
use crate::theme::{GenericTheme, Theme};

const MAX_ROWS: usize = 6;

#[derive(Clone, Deserialize, Debug)]
pub struct Input {
    pub(crate) slots: Vec<Slot>,
    pub(crate) theme: GenericTheme,
}

pub struct Panel {
    pub(crate) slots: Vec<Slot>,
    pub(crate) theme: Box<dyn Theme>,
}

impl From<Input> for Panel {
    fn from(input: Input) -> Self {
        Panel {
            slots: input.slots,
            theme: input.theme.new(),
        }
    }
}

impl Panel {
    pub fn render(&mut self) -> String {
        format!(
            "{}\n{}",
            self.layout().render(&self.theme),
            self.theme.render_panel(self)
        )
    }
    fn shift(
        &mut self,
        index: usize,
        text: &str,
        group: &str,
        row: usize,
        stack: &mut Vec<Balloon>,
    ) -> bool {
        let previous = stack.last().unwrap();
        for shift in (0..=max_shift(text, index)).rev() {
            let attempt = Balloon::new(
                self.theme.style_group(group),
                text.to_string(),
                index,
                row,
                shift,
            );
            if let Ok(balloon) = attempt {
                if !previous.overlaps(&balloon) && balloon.end() <= Balloon::WIDTH {
                    stack.push(balloon);
                    return true;
                }
            }
        }
        false
    }

    fn place(&mut self, grid: &mut Vec<Vec<Balloon>>, index: usize, text: String, group: String) {
        for row in 0..MAX_ROWS {
            match grid.get_mut(row) {
                None => {
                    grid.push(vec![Balloon::left(
                        self.theme.style_group(&group),
                        text.to_string(),
                        index,
                        row,
                    )
                    .unwrap()]);
                    return;
                }
                Some(stack) => {
                    if self.shift(index, &text, &group, row, stack) {
                        return;
                    }
                }
            }
        }
    }

    fn layout(&mut self) -> Template {
        let mut grid: Vec<Vec<Balloon>> = Default::default();
        for (index, slot) in self.slots.clone().iter().cloned().enumerate() {
            if let Slot::Occupied { text, group } = slot {
                self.place(&mut grid, index, text, group);
            }
        }
        grid.iter()
            .flatten()
            .fold(Template::default(), |template, balloon| {
                template.overlay(balloon.pre_render())
            })
    }
}
