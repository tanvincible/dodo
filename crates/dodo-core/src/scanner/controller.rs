use crate::scanner::ignore::build_ignore_matcher;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use ignore::WalkBuilder;
use std::io::BufRead;

/// Main entry point: scan project, run Magika per file, send results to AI
pub fn scan_with_magika(test_dirs: &[String]) -> anyhow::Result<()> {
    let ignore_matcher = build_ignore_matcher(test_dirs)?;

    let walker = WalkBuilder::new(".")
        .hidden(false) // we check hidden manually
        .standard_filters(true) // enables .gitignore, .ignore, etc.
        .build();

    let files_to_process: Vec<PathBuf> = walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                && !is_hidden(path)
                && !ignore_matcher.matched(path, false).is_ignore()
        })
        .map(|entry| entry.into_path())
        .collect();

    files_to_process.par_iter().for_each(|path| {
        if let Err(err) = process_file(path) {
            eprintln!("Error processing {}: {}", path.display(), err);
        }
    });

    Ok(())
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

fn process_file(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("magika")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("Magika failed on {}", path.display());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Magika output for {}:\n{}", path.display(), stdout);
    send_to_ai(path, &stdout)?;

    Ok(())
}

fn send_to_ai(path: &Path, magika_output: &str) -> anyhow::Result<()> {
    println!("Sending to AI: {}\n{}", path.display(), magika_output);
    Ok(())
}
