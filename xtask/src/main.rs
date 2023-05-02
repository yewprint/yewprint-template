use std::{path::Path, process};
use xtask_wasm::{anyhow::Result, clap, DistResult};

#[derive(clap::Parser)]
enum Cli {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    let cli: Cli = clap::Parser::parse();

    env_logger::builder()
        .filter(Some("xtask"), log::LevelFilter::Info)
        .init();

    match cli {
        Cli::Dist(args) => {
            let DistResult { dist_dir, .. } =
                args.static_dir_path("static").run("{{project-name}}")?;

            download_css(&dist_dir)?;
        }
        Cli::Watch(args) => {
            let mut command = process::Command::new("cargo");
            command.arg("check");

            args.run(command)?;
        }
        Cli::Start(args) => {
            args.arg("dist")
                .start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}

fn download_css(path: &Path) -> Result<()> {
    let css_path = path.join("blueprint.css");

    if !css_path.exists() {
        yewprint_css::download_css(&css_path)?;
    }

    Ok(())
}
