use clap::Parser;
use owo_colors::OwoColorize;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author = "AndariiDev", version = "0.2.0", about = "A dependency checker", long_about = None)] // strings must be quoted
struct Args {
    // triple slashes are doc comments
    /// The root directory of the project to scan
    #[arg(short, long)]
    path: Option<String>,

    /// The name of the file containing the dependencies (#include statements in C, etc)
    #[arg(short = 'f', long = "source-file", default_value_t = String::from("main.c"))]
    // provide default
    source_file: String,
}

#[derive(Error, Debug)]
enum DetectiveError {
    #[error("Source file not found at: {0}")]
    SourceFileNotFound(PathBuf),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), DetectiveError> {
    let args = Args::parse();

    let dir_path: PathBuf = match args.path {
        // Case 1: user provided path
        Some(p) => PathBuf::from(p),

        // Case 2: No path provided; use CWD
        None => env::current_dir()?,
    };

    if !dir_path.exists() {
        return Err(DetectiveError::SourceFileNotFound(dir_path));
    }

    scan_directory(&dir_path, &args.source_file)?;

    Ok(())
}

// TODO: implement fallback: Check 1 local/relative, check 2 global/project_root
fn scan_directory(dir_path: &Path, source_filename: &str) -> Result<(), DetectiveError> {
    // 1. Start the loop to process all entries in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path(); // Full path to the current item

        // 2. Check if the item is a directory
        if path.is_dir() {
            // if dir, call function itself (recursively)
            // skip hidden directories
            if path
                .file_name()
                .map_or(false, |name| name.to_string_lossy().starts_with("."))
            {
                continue;
            }
            // Recursive step
            scan_directory(&path, source_filename)?;
        }
        // 3. Check if the item is the specific source file we want to scan
        else if path
            .file_name()
            .map_or(false, |name| name == source_filename)
        {
            // If its source file, execute checking logic
            let content = fs::read_to_string(path)?;

            let dependencies: Vec<String> = content
                .lines()
                // 1. Filter: Keep only lines that start with "#include"
                .filter(|line| line.starts_with("#include"))
                // 2. Map: For each line, extract the filename (e.g., "dep.h")
                .map(|line| {
                    let dependency_part = line.trim_start_matches("#include").trim();
                    dependency_part.trim_matches('"')
                })
                // 3. Collect: Gather the results into a vector
                .map(|s| s.to_string())
                .collect();

            println!(
                "\nFound the following dependencies to check:\n{:#?}",
                dependencies
            );

            for line in dependencies {
                let full_path = dir_path.join(&line);

                if full_path.exists() {
                    println!("Exists: {}", line.green());
                } else {
                    println!("Missing dependency: {}", line.red().bold());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)] // first ever attempt at writing a unit test

mod tests {

    use super::*; // bring everything from main into scope
    use std::path::Path;

    #[test]

    fn success() {
        // ARRANGE: Set up smallest possible environment; only need root and a dep here
        let project_root = Path::new("dummy_root");
        let dependency_name = "missing_file.h";

        // ACT: now execute logic to test (path joining)
        let full_path = project_root.join(dependency_name);

        // ASSERT: Check expected outcome
        // 1. check PathBuf was constructed correctly (optional, but for learning purposes)
        assert_eq!(full_path.to_string_lossy(), "dummy_root/missing_file.h");

        // 2. crucially, we haven't created the file, so it must not exist
        assert!(!full_path.exists(), "Test failed: Path should not exist");
    }
}
