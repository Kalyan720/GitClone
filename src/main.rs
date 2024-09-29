// module to collect input from command-line-argument
use std::env;


#[allow(unused_imports)]
use anyhow::Ok;

mod init;
mod cat_file;
mod hash_object;
fn main(){
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();  
    if args.len()>1{
        match args[1].as_str(){
            "init" => {if let Err(e) = init::initialize(&args) {
                eprintln!("Error initializing: {}", e);
            }}
            "cat-file" =>  {if let Err(e) = cat_file::read(&args) {
                eprintln!("Error reading file: {}", e);
            }}
            "hash-object" =>  {if let Err(e) = hash_object::hash(&args) {
                eprintln!("Error reading file: {}", e);
            }}
            _ => {println!("There is an Error")}
        }
    }
    else{
        eprintln!("No command provided. Usage: {} <command>", args[0]);
    }
}