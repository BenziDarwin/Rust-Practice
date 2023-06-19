use minigrep::greptools::Config;
use minigrep::run;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    //let config: Config = match parse_config(&args) {
    //    Ok(config) => config,
    //  Err(_) => Config{ query:"".to_string(), filepath:"".to_string(), content: "No Content".to_string()}
    //};
    let config = Config::build(&args).unwrap_or_else(|err| {
      // eprintln can be used to print error messages.
        println!("An error has occurred!: {}", err);
        process::exit(1);
    });
    run(config);
    
}


