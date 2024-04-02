use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config)
    // println!("Searching for {} in {}", config.query, config.file_path);
}

fn run(config: Config) {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    fs::read_to_string(config.file_path).expect("should have been able to read the file");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query: query,
            file_path: file_path,
        })
    }
}
