use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query1 = args.get(1);
    let file_path = args.get(2);
    println!(
        "Searching for {} in {}",
        query1.unwrap(),
        file_path.unwrap()
    );
    println!("{file_path} should exist");
    let _contents = fs::read_to_string(file_path.unwrap()).expect("{file_path} should exist");
}
