use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::error::Error;
use std::io::Write;
use hex::encode;
use flate2::write::ZlibEncoder;

// calcluate the hash of the file
pub fn hash(args:&Vec<String>) -> Result<(), Box<dyn Error>> {
    if &args[2] == "-w" {
        // Read the contents of the file and transfer to other variable.
        let content:String = fs::read_to_string(&args[3]).unwrap();
        println!("this is the content of the message : {}", content);

        let blob_content = format!("blob {}\0{}",content.len(),content);
        println!("this is the blob content : {:?}",blob_content);

        // Calculate SHA1-hash for the blob_content
        let mut hasher = Sha1::new();
        hasher.update(&blob_content);
        let result = hasher.finalize();
        // .. here we get an array of <u8,20>.
        //println!("let us see what happens : {:?}", &result);
        let sha1hash = encode(result);
        // .. here we converted the above array into a readable format.
        println!("Finally : {}", sha1hash);

        // Compress the file using Zlib
        let mut compressor = ZlibEncoder::new(Vec::new(),Compression::default());
        compressor.write_all(&content.as_bytes())?;
        let final_compressor = compressor.finish().unwrap();
        //println!("this is the final compressor : {:?}", final_compressor);
        
        // Store it in the .git/objects/{folder_name}/{file_name}
        let folder_name = &sha1hash[..2];
        let file_name = &sha1hash[2..];
        fs::create_dir(format!(".git/objects/{}", folder_name)).unwrap();
        fs::write(format!(".git/objects/{}/{}",folder_name, file_name),final_compressor).unwrap();

        println!("Successfully written the specified files in the directory ... HURRAY!");

        Ok(())
    }
    else{
        println!("there is an Error that need to handled wisely");
        Ok(())
    }
}
