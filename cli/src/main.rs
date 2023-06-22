//use std::fs::read_to_string;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    ///PathBuf is like a String but for a file system paths that work cross-platform
    path: std::path::PathBuf,
}
fn iterate_content(c: &String, p: &String) {
    for line in c.lines(){
        if line.contains(p){
            println!("{}", line);
        }
    }
}


fn main() {
    let args = Cli::parse();
    // to do -> better error messaging instead of expect
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let result = std::fs::read_to_string("text.txt");
    match result{
        Ok(content) => { content },
        Err(error) => { return Err(error.into()) }
    };
    iterate_content(&content, &args.pattern);
    //println!("Pattern to search {}", args.pattern);
    //print!(" File to search in {}", args.path.display());
    //println!("Hello, world!");
    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no path given");
}
