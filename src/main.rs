#[macro_use]
extern crate json;
extern crate regex;

use regex::Regex;
use std::env;
mod evernote2turtl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Usage: {} [zip file or unzipped directory path] [numeric user ID]\n",
            args[0]
        );
        return;
    }
    let user_id = u32::from_str_radix(args[2].as_str(), 10).unwrap();
    let re: Regex = Regex::new(r"\.zip$").unwrap();
    if re.is_match(args[1].as_str()) {
        // Zip file
        let j =
            evernote2turtl::create_turtl_backup_from_zipfile(args[1].as_str(), user_id).unwrap();
        println!("{:#}", j);
    } else {
        // unzipped directory
        let j =
            evernote2turtl::create_turtl_backup_from_directory(args[1].as_str(), user_id).unwrap();
        println!("{:#}", j);
    }
}
