use std::io;
use std::fs::{self, File};
use std::path::Path;
use std::str::FromStr;
use std::io::prelude::*;
use std::process::Command;

use crate::pure;
use crate::utils;

const CONFIGPATH: &str = "~/.config/koneko/config.ini";

fn read_raw() -> Option<String> {
    let mut f = File::open(CONFIGPATH).ok()?;
    let mut result = String::new();
    f.read_to_string(&mut result).ok()?;

    Some(result.to_lowercase())
}

fn get_section(section_name: &str) -> Option<String> {
    let file: String = read_raw()?;
    let section_head = file
        .split(&format!("[{}]", section_name.to_lowercase()))
        .last()?;
    let section = section_head.split('[').next()?.trim();
    Some(section.to_string())
}

fn get_setting(section_name: &str, setting_name: &str) -> Option<String> {
    let section = get_section(&section_name.to_lowercase())?;
    let startpos = section.find(&setting_name.to_lowercase())?;
    let endpos = section.get(startpos..)?.find('\n')?;
    let kv_vec: &Vec<&str> = &section[startpos..endpos].split('=').collect();

    Some(kv_vec[1].to_string())
}

fn parse_setting<T: FromStr>(section: &str, setting: &str, fallback: T) -> T {
    match get_setting(section, setting) {
        Some(b) => b.parse::<T>().unwrap_or(fallback),
        _ => fallback,
    }
}

pub fn check_image_preview() -> bool {
    parse_setting("experimental", "image_mode_previews", false)
}

pub fn check_print_info() -> bool {
    parse_setting("misc", "print_info", true)
}

fn width_padding(side: &str, dimension: &str, fallbacks: (i32, i32)) -> (i32, i32) {
    (
        parse_setting("lscat", &format!("image_{}", side), fallbacks.0),
        parse_setting(
            "lscat",
            &format!("image_{}_spacing", dimension),
            fallbacks.1,
        ),
    )
}

pub fn ncols_config() -> i32 {
    let (width, padding) = width_padding("width", "x", (18, 2));
    pure::ncols(utils::term_width(), width, padding)
}

pub fn nrows_config() -> i32 {
    let (height, padding) = width_padding("height", "x", (8, 2));
    pure::nrows(utils::term_height(), height, padding)
}

pub fn xcoords_config(offset: i32) -> Vec<i32> {
    let (width, padding) = width_padding("width", "x", (18, 2));
    pure::xcoords(utils::term_width(), width, padding, offset)
}

pub fn ycoords_config() -> Vec<i32> {
    let (height, padding) = width_padding("height", "x", (8, 2));
    pure::ycoords(utils::term_height(), height, padding)
}

pub fn gallery_page_spacing_config() -> i32 {
    parse_setting("lscat", "page_spacing", 23)
}

pub fn users_page_spacing_config() -> i32 {
    gallery_page_spacing_config() - 3
}

pub fn thumbnail_size_config() -> i32 {
    parse_setting("lscat", "image_thumbnail_size", 310)
}

pub fn get_gen_users_settings() -> (i32, i32) {
    (
        parse_setting("lscat", "users_print_name_xcoord", 18),
        parse_setting("lscat", "images_x_spacing", 2),
    )
}

pub fn image_text_offset() -> i32 {
    parse_setting("experimental", "image_mode_text_offset", 4)
}

pub fn gallery_print_spacing_config() -> Vec<i32> {
    let setting =
        get_setting("lscat", "gallery_print_spacing").unwrap_or("9,17,17,17,17".to_string());
    // TODO: if there is any parsing error, return the default
    setting
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub your_id: String,
}

pub fn credentials_from_config() -> Credentials {
    Credentials {
        username: get_setting("Credentials", "username").unwrap(),
        password: get_setting("Credentials", "password").unwrap(),
        your_id: get_setting("Credentials", "ID").unwrap()
    }
}

pub fn begin_config() -> Credentials {
    Command::new("clear").spawn();
    if Path::new(CONFIGPATH).exists() {
        credentials_from_config()
    } else {
        init_config()
    }
}

fn init_config() -> Credentials {
    let (username, password) = ask_credentials();
    let your_id = ask_your_id();
    let creds = Credentials {
        username,
        password,
        your_id,
    };
    let samecreds = write_config(creds);
    append_default_config();
    samecreds
}

fn ask_credentials() -> (String, String) {
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    // TODO: hide password entry
    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    (username.to_string(), password.to_string())
}

fn ask_your_id() -> String {
    println!("Do you want to save your pixiv ID? It will be more convenient");
    println!("to view artists you are following");

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    match ans.as_str() {
        "y" | "" => {
            println!("Please enter your pixiv ID:");
            let mut your_id = String::new();
            io::stdin().read_line(&mut your_id).unwrap();
            your_id
        }
        _ => "".to_string(),
    }
}

fn write_config(creds: Credentials) -> Credentials {
    Command::new("clear").spawn();
    fs::create_dir_all(Path::new(CONFIGPATH).parent().unwrap());
    let mut buffer = File::create("config.ini").unwrap();
    write!(buffer, "username={}\npassword={}",creds.username, creds.password);
    if creds.your_id != "" {
        write!(buffer, "\nID={}", creds.your_id).unwrap()
    }
    creds
}

fn append_default_config() {
    let example_cfg = Path::new("~/.local/share/koneko/example_config.ini");
    // FIXME
    //Command::new("tail").args([example_cfg, "-n", "+9"]).stdout(CONFIGPATH);
}
