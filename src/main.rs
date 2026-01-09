use clap::Parser;
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;
use std::default::Default;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;
use toml::de::Error as TomlDeserializeError;

#[derive(Parser, Debug)]
#[command(author = "AndariiDev", version = "0.2.0", about = "A dependency checker", long_about = None)] // strings must be quoted
struct Args {
    // triple slashes are doc comments
    /// The root directory of the project to scan
    #[arg(short, long)]
    path: Option<String>,

    /// The name of the file containing the dependencies (#include statements in C, etc)
    #[arg(short = 'f', long = "source-file")]
    // provide default
    source_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParsingRules {
    filenames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    rules: Option<ParsingRules>,
}

#[derive(Error, Debug)]
enum DetectiveError {
    #[error("Source file not found at: {0}")]
    SourceFileNotFound(PathBuf),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] TomlDeserializeError),
}

// Write out instead of deriving to adjust default behavior:
impl Default for ParsingRules {
    fn default() -> Self {
        Self {
            filenames: vec!["main.c".to_string()],
        }
    }
}

impl Config {
    pub fn load(root: &Path) -> Result<Self, DetectiveError> {
        let config_path = root.join("detective.toml");

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?; // Read file
            let parsed_config: Config = toml::from_str(&content)?; // Parse string
            Ok(parsed_config)
        } else {
            Ok(Config::default())
        }
    }
}

fn main() -> Result<(), DetectiveError> {
    let args = Args::parse();

    let project_root: PathBuf = match args.path {
        // Case 1: user provided path
        Some(p) => PathBuf::from(p),

        // Case 2: No path provided; use CWD
        None => env::current_dir()?,
    };

    if !project_root.exists() {
        return Err(DetectiveError::SourceFileNotFound(project_root));
    }

    let config = Config::load(&project_root)?;

    let target_files = if let Some(cli_file) = args.source_file {
        // if user typed -f, use only that one
        vec![cli_file]
    } else {
        if let Some(actual_rules) = config.rules {
            actual_rules.filenames
        } else {
            println!(
                "{}",
                "Warning: Config file found but no [rules] section defined. Using defaults."
                    .yellow()
            );
            ParsingRules::default().filenames
        }
    };

    scan_directory(&project_root, &project_root, &target_files)?;

    Ok(())
}

// TODO: implement fallback: Check 1 local/relative, check 2 global/project_root
fn scan_directory(
    global_root: &Path,
    current_dir: &Path,
    filenames: &[String],
) -> Result<(), DetectiveError> {
    // 1. Start the loop to process all entries in the directory
    for entry in fs::read_dir(current_dir)? {
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
            scan_directory(global_root, &path, filenames)?;
        }
        // 3. Check if the item is the specific source file we want to scan
        else if path.file_name().map_or(false, |name| {
            filenames.contains(&name.to_string_lossy().into_owned())
        }) {
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
                let local_path = current_dir.join(&line);

                let global_path = global_root.join(&line);

                if local_path.exists() {
                    // Success, local path exists
                    println!("Exists (local): {}", line.green());
                } else if global_path.exists() {
                    // Local failed, but global path exists: Success
                    println!("Exists (global): {}", line.green());
                } else {
                    // Failure, neither exists
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
    fn test_recurse_scan_no_panic() -> Result<(), DetectiveError> {
        // ARRANGE: Set up smallest possible environment; only need root and a dep here
        let global_root = Path::new(".");
        let current_dir = Path::new(".");
        let filenames = vec!["test_file.c".to_string()];

        // ACT & ASSERT: Call the function and use the '?' operator
        // This tests that the function runs to completion without panicking on I/O operations
        // (even though the directories don't exist yet, it tests the function's structural integrity)
        scan_directory(global_root, current_dir, &filenames)?;

        // Test passed if no error was returned
        Ok(())
    }

    #[test]
    fn test_config_loading_fails_initially() -> Result<(), DetectiveError> {
        // define
        let test_toml = r#"
            [rules]
            filenames = ["main.c", "Cargo.toml", "dep.h"]
        "#;

        // try to parse struct
        // call struct
        let config: Config = toml::from_str(test_toml)?;

        if let Some(r) = config.rules {
            assert_eq!(r.filenames.len(), 3);
            assert_eq!(r.filenames, vec!["main.c", "Cargo.toml", "dep.h"]);
        } else {
            panic!("Config rules should have been Some, but were None!");
        }

        Ok(())
    }
}
