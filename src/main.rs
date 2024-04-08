use minigrep::{run, Config};
use std::{env, process};
fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // let _ = run(&config);
    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
    // println!("Searching for {} in {}", config.query, config.file_path);
}
