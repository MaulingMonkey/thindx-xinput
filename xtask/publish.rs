use crate::{cmd, run};
use mmrbi::*;
use std::io;
use std::path::*;



pub fn publish(mut args: std::env::Args) {
    let version = args.next()                       .unwrap_or_else(|| fatal!("expected a `v0.0.0-unsound.n` version tag"));
    let version_no_v = version.strip_prefix("v")    .unwrap_or_else(|| fatal!("expected a `v0.0.0-unsound.n` version tag, instead got `{}`", version));
    if !version.contains("-unsound.") {                                fatal!("expected a `v0.0.0-unsound.n` version tag, instead got `{}`", version) }
    let _ = args.next().map(|unexpected| fatal!("unexpected positinoal argument: {:?}", unexpected));

    run(r"cargo publish --allow-dirty --dry-run --no-verify -p thindx"); // early check for e.g. git dependencies
    run(r"git worktree add target\pub");
    let err = work_in_pub(&version, &version_no_v);
    let err = err.map_err(|err| error!("{}", err));
    run(r"git worktree remove target\pub --force");
    run(r"git branch -D pub"); // created by worktree commands
    if err.is_err() { std::process::exit(1) }
}

fn work_in_pub(version: &str, version_no_v: &str) -> io::Result<()> {
    let cargo_toml_path = r"target\pub\thindx\Cargo.toml";
    let cargo_toml = std::fs::read_to_string(cargo_toml_path)?;
    let cargo_toml = cargo_toml.replace("0.0.0-git", &version_no_v);
    std::fs::write(cargo_toml_path, cargo_toml)?;
    run_in_nonfatal(r"target\pub", "git add thindx/Cargo.toml")?;
    run_in_nonfatal(r"target\pub", format!("git commit -m {:?}", version))?;
    run_in_nonfatal(r"target\pub", "cargo b")?;
    run_in_nonfatal(r"target\pub", "cargo publish --allow-dirty -p thindx")?;
    run_in_nonfatal(r"target\pub", format!("git tag {version}"))?;
    run_in_nonfatal(r"target\pub", format!("git push github {version}"))?;
    Ok(())
}

fn run_in_nonfatal(dir: impl AsRef<Path>, command: impl AsRef<str>) -> io::Result<()> {
    let dir = dir.as_ref();
    let mut c = cmd(command);
    c.current_dir(dir);
    status!("Running", "{:?}", c);
    c.status0()?;
    Ok(())
}