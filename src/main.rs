use anyhow::Result;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main("app", pre_build = pre_build, other_cli_commands = other_cli_commands)]
#[derive(StructOpt, Debug)]
enum Cli {
    UpdateCss,
}

fn pre_build(args: &DefaultBuildArgs, profile: BuildProfile, command: &mut Command) -> Result<()> {
    let package = args.frontend_package();

    download_blueprint_css(package, false)?;

    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            command.args(&["--features", "console_error_panic_hook"]);
        }
    }

    Ok(())
}

fn other_cli_commands(cli: Cli, _metadata: &Metadata, package: &Package) -> Result<()> {
    match cli {
        Cli::UpdateCss => {
            download_blueprint_css(package, true)?;
        }
    }

    Ok(())
}

fn download_blueprint_css(package: &Package, force: bool) -> Result<()> {
    let css_path = package
        .manifest_path
        .parent()
        .unwrap()
        .join("static")
        .join("blueprint.css");

    if force || !css_path.exists() {
        yewprint_css::download_css(&css_path)?;
    }

    Ok(())
}
