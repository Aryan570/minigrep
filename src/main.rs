use std::env;
use std::process;
use minigrep::{self , Config};
fn main() {
    // step -1 : To read the command line arguments such as cargo run -- needle haystack
    // Read the command line argument with std::env::args func.
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("There was problem parsing the arguments: {err}");
        process::exit(1);
    });
    // Step 2: Read the file
    if let Err(e) = minigrep::run(config) {
        //print to standard error stream
        eprintln!("There was an error, {e}");
        process::exit(1);
    }

}

