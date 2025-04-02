use std::{env, fs, io, path::Path};

use args::Args;
use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;

#[path = "src/args.rs"]
mod args;

fn generate_man_pages(out_dir: impl AsRef<Path>, bin: &str) -> io::Result<()> {
    let out_dir = out_dir.as_ref().join("man");

    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buf = vec![];
    man.render(&mut buf)?;

    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join(format!("{bin}.1")), buf)?;
    Ok(())
}

fn generate_completions(out_dir: impl AsRef<Path>, bin: &str) -> io::Result<()> {
    let out_dir = out_dir.as_ref().join("completions");

    let mut cmd = Args::command();

    fs::create_dir_all(&out_dir)?;
    for &sh in Shell::value_variants() {
        clap_complete::generate_to(sh, &mut cmd, bin, &out_dir)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("cargo::rerun-if-env-changed=PLATES_GEN_COMPLETIONS");
    println!("cargo::rerun-if-env-changed=PLATES_GEN_MAN_PAGES");
    println!("cargo::rerun-if-changed=src");

    let bin = env!("CARGO_PKG_NAME");
    let out_dir = env::var_os("OUT_DIR").unwrap_or("out".into());

    if env::var_os("PLATES_GEN_MAN_PAGES").is_some() {
        generate_man_pages(&out_dir, bin)?;
    }

    if env::var_os("PLATES_GEN_COMPLETIONS").is_some() {
        generate_completions(&out_dir, bin)?;
    }

    Ok(())
}
