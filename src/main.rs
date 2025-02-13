use std::process::exit;

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
        //check filename extension
        if filename.len() < 6 || &filename[filename.len()-5..] != ".echo" {
            println!("Invalid source file!");
            exit(1);
        }
    }else {
        println!("Too many arguments!");
    }
}
