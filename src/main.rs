use std::path::Path;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let project_root = Path::new("../my_c_project");
    let test_repo_path = project_root.join("dependencies.txt");
    
    if !test_repo_path.exists() {
        println!("This file doesn't exist!");

        } else {
        println!("This file exists!");
        
        let content = fs::read_to_string(test_repo_path)?;
        
        let lines: Vec<&str> = content.lines().collect();

        println!("\nFound the following dependencies to check:\n{:#?}", lines);

        for line in lines {
            let full_path = project_root.join(line);

            if full_path.exists() {
                println!("Exists: {:#?}", full_path);
                
            } else {

                println!("Mssing (ENOENT): {:#?}", full_path);
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
