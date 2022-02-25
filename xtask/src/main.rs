use std::process;

use xtask_wasm::{anyhow::Result, default_dist_dir};

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(clap::Parser)]
enum Command {
    Start(xtask_wasm::Watch),
    Serve(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    env_logger::init();
    let args: Cli = clap::Parser::parse();
    match args.cmd {
        Command::Start(arg) => start(arg)?,
        Command::Serve(arg) => serve(arg)?,
    }

    Ok(())
}

fn start(arg: xtask_wasm::Watch) -> Result<()> {
    let mut build = process::Command::new("cargo");
    build.arg("run").arg("--features").arg("editor");
    arg.run(build)?;

    Ok(())
}

fn serve(mut dev_server: xtask_wasm::DevServer) -> Result<()> {
    let mut build = process::Command::new("cargo");
    build
        .arg("build")
        // .arg("--features")
        // .arg("dev")
        .arg("--target")
        .arg("wasm32-unknown-unknown");

    dev_server = dev_server.command(build);
    let port = dev_server.port;
    println!("dev server running on http://localhost:{}/", port);
    dev_server.start(default_dist_dir(false))?;

    Ok(())
}
