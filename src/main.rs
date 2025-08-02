use std::env;
use std::process;

use minigrep::Config;

#[tokio::main]
async fn main(){
    let args :Vec<String> = env::args().collect();

    // Check if user wants to start web server
    if args.len() == 2 && args[1] == "--web" {
        minigrep::start_web_server().await;
        return;
    }

    // Original CLI functionality
    let config= Config::new(&args).unwrap_or_else(|err |{
        eprintln!("Problem parsing arguments: {}",err);
        eprintln!("Usage: {} <query> <filename>", args[0]);
        eprintln!("   or: {} --web", args[0]);
        process::exit(1)
    });

    println!("Searching for '{}'",config.query);
    println!("In file '{}'",config.filename);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}




