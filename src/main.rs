use std::path::Path;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let project_root = Path::new("../my_c_project");
    let test_repo_path = project_root.join("../my_c_project/dependencies.txt");
    
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
