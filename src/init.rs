use std::fs;
use std::io;

// Handle the 'init' command

pub fn initialize(args:&Vec<String>) -> io::Result<()>{
    if args.len() > 1 && args[1] == "init" {
        fs::create_dir(".git")?;
        fs::create_dir(".git/objects")?;
        fs::create_dir(".git/refs")?;
        fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
        println!("Initialized git directory");
        Ok(())
    } 
    else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid command"))
    }
}
    
