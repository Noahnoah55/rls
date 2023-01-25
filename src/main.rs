use clap::Parser;
use colored::*;


#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    for entry in args.path.read_dir().expect("FUCK") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let str = path.file_name().expect("Non-existant file").to_str().expect("Unable to convert to string");
            if path.is_dir() {
                println!("{}", str.bold().bright_blue());
            }
            else {
                println!("{}", str);
            }
        }
    }
}
