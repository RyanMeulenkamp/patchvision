use std::env::args;
use std::fs::read_to_string;
use anyhow::anyhow;
use patchvision::panel::Panel;
use patchvision::theme;

fn main() -> anyhow::Result<()> {
    let yaml = read_to_string(
        args()
            .nth(1)
            .ok_or(anyhow!("Please pass input yaml file"))?
    )?;
    let panel: Panel<24> = serde_yaml::from_str(&yaml)?;

    println!(
        "{}\n{}",
        panel.layout().render(Box::new(theme::Default {})),
        panel.render_panel(),
    );

    Ok(())
}