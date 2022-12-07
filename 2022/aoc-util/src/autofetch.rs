//! Auto-fetch the puzzle input.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub async fn autofetch() -> anyhow::Result<()> {
    let (day, root) = determine_day_root()?;
    let input_path = root.join("input");
    if input_path.exists() {
        println!("Auto-fetch: the input file already exists, skipping fetch.");
        return Ok(());
    }

    println!("Auto-fetch: fetching today's input file.");

    let cookie = get_cookie(root)?;

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    let client = reqwest::Client::new();
    let input_text = client
        .get(url)
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    File::create(input_path)?.write_all(input_text.as_bytes())?;
    Ok(())
}

fn determine_day_root() -> anyhow::Result<(u8, PathBuf)> {
    let mut dir = std::env::current_dir()?;
    loop {
        let cargo_path = dir.join("Cargo.toml");
        if cargo_path.exists() {
            break;
        }
        if !dir.pop() {
            return Err(anyhow::anyhow!("failed to find Cargo.toml"));
        }
    }

    let dir_name = dir
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("couldn't get directory file name?"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("could not turn directory name into string"))?;
    let n = dir_name
        .strip_prefix("d")
        .ok_or_else(|| anyhow::anyhow!("directory name did not start with 'd'"))?;
    let n = n.parse()?;
    Ok((n, dir))
}

fn get_cookie<P: AsRef<Path>>(root: P) -> anyhow::Result<String> {
    let mut p = root.as_ref().join("..");
    p.push("aoc-cookie");
    Ok(fs::read_to_string(p)?)
}
