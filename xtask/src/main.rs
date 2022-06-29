use std::process;
use xtask_wasm::{anyhow::Result, clap};

#[derive(clap::Parser)]
enum Cli {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
    /// Update Blueprint CSS
    UpdateCSS,
}

fn main() -> Result<()> {
    let cli: Cli = clap::Parser::parse();

    env_logger::builder()
        .filter(Some("xtask"), log::LevelFilter::Info)
        .init();

    match cli {
        Cli::Dist(args) => {
            download_css(false)?;

            args.static_dir_path("static").run("run")?;
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
        Cli::UpdateCSS => download_css(true)?,
    }

    Ok(())
}

fn download_css(force: bool) -> Result<()> {
    let static_path = PathBuf::from("static");
    let css_path = static_path.join("blueprint.css");

    if force || !css_path.exists() {
        yewprint_css::download_css(&css_path)?;
    }

    Ok(())
}
