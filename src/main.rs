#[macro_use]
extern crate json;
extern crate regex;

use regex::Regex;
use std::env;
mod evernote2turtl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        println!(
            "Usage: {} [zip file or unzipped directory path] [numeric user ID] [input format:evernote (default)|keep]\n",
            args[0]
        );
        return;
    }
    let format;
    if args.len() >= 4 {
        format = args[3].as_str();
    } else {
        format = "evernote";
    }
    let path = args[1].as_str();
    let user_id = u32::from_str_radix(args[2].as_str(), 10).unwrap();
    let re: Regex = Regex::new(r"\.zip$").unwrap();
    if re.is_match(path) {
        // Zip file
        assert_eq!(format, "evernote"); // For now, no Keep ZIPs
        let j = evernote2turtl::create_turtl_backup_from_zipfile(path, user_id).unwrap();
        println!("{:#}", j);
    } else {
        // unzipped directory
        let j = evernote2turtl::create_turtl_backup_from_directory(path, user_id, format).unwrap();
        println!("{:#}", j);
    }
}
