use std::error::Error;
use std::fs;
use std::io::Read;
use std::io;
use flate2::read::ZlibDecoder;

pub fn read_tree(args:&Vec<String>) -> Result<(), Box<dyn Error>> {
    // calculate the folder and file name
    let hash = &args[2];
    println!("this is the given hash : {}", &hash);
    let folder_name:&str = &hash[0..2];
    println!("this is the assigned folder name : {}", &folder_name);
    let file_name:&str = &hash[2..];
    println!("this is the given file name : {}", &file_name);

    // navigating and listing the files
    let path = format!(".git/objects/{}/{}",folder_name,file_name);
    let encoded_vector = fs::read(path)?;
    println!("{:?}",encoded_vector);
    
    // decompress the above contents using zlibdecompressor
    let k = decode_reader(encoded_vector);
    println!("{:?}", k);

    
    Ok(())
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    println!("This is z : {:?}", &mut z);
    let mut s = String::new();
    println!("This is s : {:?}", &mut s);
    z.read_to_string(&mut s)?;
    Ok(s)
}