extern crate regex;
extern crate uuid;
extern crate zip;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;
use zip::read::ZipArchive;

fn get_dummy_turtl_space_id() -> String {
    lazy_static::lazy_static! {
        static ref space_id: String = get_uuid();
    }
    space_id.to_string()
}

fn extract_title(html: &str) -> Result<String, std::io::Error> {
    let mut ret = String::new();
    lazy_static::lazy_static! {
        static ref re: Regex = Regex::new(r"<title>\s*(.+?)\s*</title>").unwrap();
    }
    for cap in re.captures_iter(html) {
        ret = cap[1].to_string();
        break;
    }
    Ok(ret)
}

fn get_uuid() -> String {
    lazy_static::lazy_static! {
        static ref re: Regex = Regex::new(r"-").unwrap();
    };
    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let id3 = Uuid::new_v4();
    let mut ret = id1.to_string();
    ret += id2.to_string().as_str();
    ret += id3.to_string().as_str();
    ret = re.replace_all(ret.as_str(), "").to_string();
    ret.truncate(80);
    ret
}

pub fn convert_evernote_file_contents_to_json(
    contents: std::string::String,
    user_id: u32,
) -> Result<json::JsonValue, std::io::Error> {
    let html_start = contents.find("<body").unwrap();
    let html = contents.get(html_start..).unwrap();
    let title = extract_title(&contents)?;
    let md = html2md::parse_html(&html);
    let j = object! {
        "id" => get_uuid(),
        "space_id" => get_dummy_turtl_space_id() ,
        "user_id" => user_id ,
        "has_file" => false ,
        "tags" => array![] ,
        "title" => title ,
        "text" => md.trim_matches(|c|c==' '||c=='\n') ,
        "type" => "text"
    };
    Ok(j)
}

pub fn convert_evernote_file_to_json(
    file_name: &str,
    user_id: u32,
) -> Result<json::JsonValue, std::io::Error> {
    let mut f = File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    convert_evernote_file_contents_to_json(contents, user_id)
}

pub fn get_turtl_backup_object(user_id: u32) -> Result<json::JsonValue, std::io::Error> {
    let ret = object! {
        "body" => json::Null ,
        "boards" => array![],
        "files" => array![],
        "notes" => array![],
        "schema_version" => 2 ,
        "spaces" => array![
            object!{
                "color" => "#000000",
                "id" => get_dummy_turtl_space_id(),
                "user_id" => user_id ,
                "invites" => array![],
                "keys" => array![],
                "members" => array![],
                "title" => "Evernote import",
            }
        ]
    };
    Ok(ret)
}

pub fn create_turtl_backup_from_zipfile(
    zipfile: &str,
    user_id: u32,
) -> Result<json::JsonValue, std::io::Error> {
    lazy_static::lazy_static! {
        static ref re_html: Regex = Regex::new(r"\.html$").unwrap();
        static ref re_hidden1: Regex = Regex::new(r"/\.").unwrap();
        static ref re_hidden2: Regex = Regex::new(r"^\.").unwrap();
    };
    let mut ret = get_turtl_backup_object(user_id)?;
    let f = File::open(zipfile)?;
    let mut zip = ZipArchive::new(f)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let filename = file.name();
        if re_html.is_match(filename)
            && !re_hidden1.is_match(filename)
            && !re_hidden2.is_match(filename)
            && file.size() > 0
        {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let note = convert_evernote_file_contents_to_json(contents, user_id)?;
            ret["notes"].push(note).unwrap();
        }
    }
    Ok(ret)
}

pub fn create_turtl_backup_from_directory(
    path: &str,
    user_id: u32,
) -> Result<json::JsonValue, std::io::Error> {
    lazy_static::lazy_static! {
        static ref re: Regex = Regex::new(r"\.html$").unwrap();
    };
    let mut ret = get_turtl_backup_object(user_id)?;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path().to_string_lossy().into_owned();
                if re.is_match(file_path.as_str()) {
                    let note = convert_evernote_file_to_json(file_path.as_str(), user_id)?;
                    ret["notes"].push(note).unwrap();
                }
            }
        }
    }
    Ok(ret)
}
