use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_file(filepath: &str) -> io::Result<String> {
    // Check if the file exists
    let path = Path::new(filepath);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }

    // Open the file
    let mut file = File::open(path)?;
    
    // Create a String to store the contents
    let mut contents = String::new();
    
    // Read the file contents into the string
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    // Example usage
    match read_file(query) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }
}