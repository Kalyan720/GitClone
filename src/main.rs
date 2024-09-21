#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::error::Error;
use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;

fn main() -> Result<(), Box<dyn Error>> {

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Handle the 'init' command
    if args.len() > 1 && args[1] == "init" {
        fs::create_dir(".git")?;
        fs::create_dir(".git/objects")?;
        fs::create_dir(".git/refs")?;
        fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
        println!("Initialized git directory");
        return Ok(()); // Return here to exit after initialization
    } 

    // Handle the 'cat-file' command
    else if args.len() > 3 && args[1] == "cat-file" && args[2] == "-p" {
        cat_file(&args[3])?; // Call cat_file and propagate error if it occurs
        return Ok(()); // Return here after successfully calling cat_file
    } 

    // Handle unknown commands
    else {
        println!("unknown command: {}", args[1]);
        return Ok(()); // Return here for unknown commands
    }
}

// Function to read and decompress Git object files from .git/objects/{hash}
fn cat_file(hash: &str) -> Result<(), Box<dyn Error>> {
    let hash_part1 = &hash[..2];
    let hash_part2 = &hash[2..];
    let blob_path = format!(".git/objects/{}/{}", hash_part1, hash_part2);
    let compressed = fs::read(blob_path)?;
    
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    // Split the decompressed content at the null byte and print the content
    if let Some(content) = decompressed.split('\0').nth(1) {
        print!("{}", content);
    } else {
        println!("Error: Invalid object format.");
    }

    Ok(())
}
