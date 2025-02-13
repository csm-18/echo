use std::io::Read;
use std::{path::Path, process::exit};
use std::fs::File;
// echo version
const VERSION:&str = "1.0.0";
fn main() {
    // cli args
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    
    // parse args
    if args.is_empty() {
        println!("echo v{VERSION}");
        println!("To compile:");
        println!(" echo <filename.echo>");
    }else if args.len() == 1 {
        let filename = &args[0];
        // check filename extension
        if filename.len() < 6 || &filename[filename.len()-5..] != ".echo" {
            println!("Invalid source file!");
            exit(1);
        }

        // source file path
        let source_file_path = Path::new(filename);
        
        // open file
        let mut source_file = match File::open(source_file_path){
            Err(_err) => {println!("Unable to open '{filename}'");exit(1)},
            Ok(file) => file,
        };

        // read file
        let mut code = String::new();
        if let Err(_err) = source_file.read_to_string(&mut code) {println!("Unable to read '{filename}'");exit(1);}
        println!("{code}");
    }else {
        println!("Too many arguments!");
    }
}
