use std::io::{self, Read};
use std::fs;
use flate2::read::ZlibDecoder;

// Handle the 'cat-file' command

pub fn read(args:&Vec<String>) -> io::Result<()>{
    if args[2] == "-p"{
        let hash = &args[3];
        let folder_name:&str = &hash[..2];
        let file_name:&str = &hash[2..];
        let blob_path = format!(".git/objects/{}/{}", folder_name, file_name);
        let compressed = fs::read(blob_path)?;
        
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)?;

        if let Some(content) = decompressed.split('\0').nth(1) {
            print!("{}", content);
        } else {
            println!("Error: Invalid object format.");
        }

        Ok(())
    }
    else {
        println!("Invalid argument to read the Blob object; instead, use '-p'");
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid argument"))
    }
}