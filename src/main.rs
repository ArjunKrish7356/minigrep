use std::env;
use std::process;

use minigrep::Config;



fn main(){
    let args :Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err |{
        println!("Problem passing arguments{}",err);
        process::exit(1)
    });

    println!("Searching for {}",config.query);
    println!("In {}",config.filename);
    
    let _ =minigrep::run(config);

}




