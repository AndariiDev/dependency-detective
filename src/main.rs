use std::path::PathBuf;
use std::fs;
use std::error::Error;
use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "AndariiDev", version = "0.1.3", about = "A dependency checker", long_about = None)] // strings must be quoted
struct Args {
    // triple slashes are doc comments
    /// The root directory of the project to scan
    #[arg(short, long)]
    path: Option<String>,

    /// The name of the file containing the dependencies (#include statements in C, etc)
    #[arg(short = 'f', long = "source-file", default_value_t = String::from("main.c"))] // provide default
    source_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // let project_root = Path::new("../my_c_project");
    let project_root: PathBuf = match args.path {
        // Case 1: user provided path
        Some(p) => PathBuf::from(p),

        // Case 2: No path provided; use CWD
        None => env::current_dir()?,
    };
    
    let test_repo_path = project_root.join(&args.source_file);

    if !test_repo_path.exists() {
        println!(
            "Error: Source file not found. Could not find {:#?} in project root.",
            test_repo_path
        );
        return Ok(());

        } else {

        let content = fs::read_to_string(test_repo_path)?;
        
        let dependencies: Vec<String> = content.lines()
            
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

        println!("\nFound the following dependencies to check:\n{:#?}", dependencies);

        for line in dependencies {
            let full_path = project_root.join(&line);

            if full_path.exists() {
                println!("Exists: {:#?}", full_path);
                
            } else {

                println!("Mssing dependency: {}", line);
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
