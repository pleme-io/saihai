//! `saihai-forge` — regenerate every surface from the action catalog.
//!
//! `build` writes the matrix; `check` regenerates in memory and diffs, exiting
//! non-zero on drift. The second is the one that matters: a "DO NOT EDIT"
//! header is a wish, a differential is a gate.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use saihai_forge::{Coverage, matrix};
use saihai_spec::Catalog;

const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");

fn load() -> Result<Catalog, String> {
    let c = Catalog::from_source(CATALOG).map_err(|e| format!("catalog failed to compile: {e}"))?;
    c.validate()
        .map_err(|e| format!("catalog failed to validate: {e}"))?;
    Ok(c)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let run = || -> Result<ExitCode, String> {
        let cat = load()?;
        match args.first().map(String::as_str) {
            Some("coverage") => {
                print!("{}", Coverage::measure(&cat).report());
                Ok(ExitCode::SUCCESS)
            }
            Some("list") => {
                for a in &cat.actions {
                    println!(
                        "{:<28} {:<11} {:<4} {}",
                        a.id.as_str(),
                        format!("{:?}", a.class()).to_lowercase(),
                        a.rung().as_str(),
                        a.gloss
                    );
                }
                println!("\n{} actions", cat.actions.len());
                Ok(ExitCode::SUCCESS)
            }
            Some("build") => {
                let dir = Path::new(args.get(1).ok_or("build needs an <out-dir>")?);
                for (name, body) in matrix(&cat) {
                    let path: PathBuf = dir.join(&name);
                    if let Some(p) = path.parent() {
                        std::fs::create_dir_all(p).map_err(|e| format!("{}: {e}", p.display()))?;
                    }
                    std::fs::write(&path, body).map_err(|e| format!("{}: {e}", path.display()))?;
                    println!("wrote {}", path.display());
                }
                Ok(ExitCode::SUCCESS)
            }
            Some("check") => {
                let dir = Path::new(args.get(1).ok_or("check needs an <out-dir>")?);
                let mut drift = 0usize;
                for (name, want) in matrix(&cat) {
                    let path = dir.join(&name);
                    match std::fs::read_to_string(&path) {
                        Err(_) => {
                            eprintln!("MISSING  {}", path.display());
                            drift += 1;
                        }
                        Ok(got) if got != want => {
                            let at = got
                                .lines()
                                .zip(want.lines())
                                .position(|(a, b)| a != b)
                                .map_or_else(
                                    || "length".to_string(),
                                    |i| format!("line {}", i + 1),
                                );
                            eprintln!("DRIFT    {} (first difference at {at})", path.display());
                            drift += 1;
                        }
                        Ok(_) => {}
                    }
                }
                if drift == 0 {
                    println!("saihai-forge check: no drift");
                    Ok(ExitCode::SUCCESS)
                } else {
                    eprintln!("saihai-forge check: {drift} artifact(s) drifted");
                    Ok(ExitCode::FAILURE)
                }
            }
            _ => {
                eprintln!(
                    "saihai-forge — generate the desktop API surfaces\n\n\
                     USAGE:\n    \
                       saihai-forge build <out-dir>\n    \
                       saihai-forge check <out-dir>\n    \
                       saihai-forge list\n    \
                       saihai-forge coverage\n"
                );
                Ok(ExitCode::from(2))
            }
        }
    };
    match run() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("saihai-forge: {e}");
            ExitCode::FAILURE
        }
    }
}
