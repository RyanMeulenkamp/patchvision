use anyhow::anyhow;
use patchvision::panel::{Input, Panel};
use std::env::args;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let yaml = read_to_string(
        args()
            .nth(1)
            .ok_or(anyhow!("Please pass input yaml file"))?,
    )?;
    let input: Input = serde_yaml::from_str(&yaml)?;
    let mut panel: Panel = input.into();
    println!("{}", panel.render());

    Ok(())
}
