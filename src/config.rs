use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;

use crate::pure;
use crate::utils;

const CONFIGPATH: &str = "~/.config/koneko/config.ini";

fn read_raw() -> Option<String> {
    let mut f = File::open(CONFIGPATH).ok()?;
    let mut result = String::new();
    f.read_to_string(&mut result).ok()?;

    Some(result)
}

fn get_section(section_name: &str) -> Option<String> {
    let file: String = read_raw()?;
    let section_head = file.split(&format!("[{}]", section_name)).last()?;
    let section = section_head.split('[').next()?.trim();
    Some(section.to_string())
}

fn get_setting(section_name: &str, setting_name: &str) -> Option<String> {
    let section = get_section(section_name)?;
    let startpos = section.find(setting_name)?;
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
