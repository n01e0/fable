use is_executable::IsExecutable;
use std::env;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::var("PATH")?
        .split(":")
        .map(WalkDir::new)
        .map(|w| w.max_depth(1))
        .map(|w| w.into_iter())
        .flatten()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file())
        .filter(|f| f.path().is_executable())
        .for_each(|f| println!("{}", f.path().display()));

    Ok(())
}
