use std::process::Command;
use std::io::{self, Write};

use serde_json::Value;

use crate::utils;
use crate::colors::*;

pub fn write(value: &str) {
    io::stdout().write_all(value.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn move_cursor_up(num: i32) {
    if num > 0 {
        write(&format!("\033[{}A", num))
    }
}

pub fn move_cursor_down(num: i32) {
    if num > 0 {
        write(&format!("\033[{}B", num))
    }
}

pub fn erase_line() {
    write(&"\033[K")
}

pub fn print_cols(spacings: Vec<i32>, ncols: i32) {
    for (idx, space) in spacings[..ncols as usize].iter().enumerate() {
        write(&" ".repeat(*space as usize));
        write(&format!("{}", idx + 1))
    }
}

fn print_info(message_xcoord: usize) {
    println!("{}000", " ".repeat(message_xcoord));
    println!("{}Example artist", " ".repeat(message_xcoord));
}

pub fn maybe_print_size(actions: Vec<i32>, size: i32) {
    if actions.contains(&1) || actions.contains(&7) {
        println!("image_thumnail_size = {}", size)
    }
}

pub fn maybe_print_width_xpadding(actions: Vec<i32>, image_width: i32, xpadding: i32) {
    if actions.contains(&2) || actions.contains(&7) {
        println!("image_width = {}", image_width);
        println!("images_x_spacing = {}", xpadding)
    }
}

pub fn maybe_print_height_ypadding(actions: Vec<i32>, image_height: i32, ypadding: i32) {
    if actions.contains(&3) || actions.contains(&7) {
        println!("image_height = {}", image_height);
        println!("images_y_spacing = {}", ypadding)
    }
}

pub fn maybe_print_page_spacing(actions: Vec<i32>, page_spacing: i32) {
    if actions.contains(&4) || actions.contains(&7) {
        println!("page_spacing = {}", page_spacing);
    }
}

pub fn maybe_print_print_spacing(actions: Vec<i32>, gallery_print_spacing: Vec<i32>) {
    if actions.contains(&5) || actions.contains(&7) {
        print!("gallery_print_spacing = ");
        gallery_print_spacing.iter().for_each(|x| print!("{},", x));
        print!("\n");
    }
}

pub fn maybe_print_user_info(actions: Vec<i32>, user_info_xcoord: i32) {
    if actions.contains(&6) || actions.contains(&7) {
        println!("users_print_name_xcoord = {}", user_info_xcoord)
    }
}

pub fn print_doc(doc: &str) {
    Command::new("clear").spawn();
    let number_of_newlines = doc.matches('\n').count() as u16;
    let bottom = utils::term_height() - (number_of_newlines + 2);
    move_cursor_down(bottom as i32);
    println!("{}", doc);
}

pub fn print_multiple_imgs(illusts_json: &Value) {
    let HASHTAG = format!("{}#", RED);
    let HAS = format!("{} has {}", RESET, BLUE);
    let OF_PAGES = format!("{} pages", RESET);

    let mut i = 0;
    loop {
        let processed_elem = &illusts_json[i];
        if processed_elem.is_null() {
            break;
        }
        let number = &processed_elem["page_count"];
        if number.as_i64().unwrap() > 1 {
            print!("{}{}{}{}{},", HASHTAG, i, HAS, number, OF_PAGES);
        }
        i += 1
    }
    print!("\n");
}

pub fn update_gallery_info(spacings: Vec<i32>, ncols: i32, current_selection: i32) {
    move_cursor_up(2);
    erase_line();
    print_cols(spacings, ncols);
    println!(
        "\n\nAdjusting the number of spaces between {} and {}",
        current_selection,
        current_selection + 1
    );
    move_cursor_up(1);
}

pub fn update_user_info(spacing: i32) {
    erase_line(); // Erase the first line
    move_cursor_down(1); // Go down and erase the second line
    erase_line();
    move_cursor_up(1); // Go back up to the original position
    print_info(spacing as usize); // Print info takes up 2 lines
    move_cursor_up(2); // so go back to the top
}

pub fn image_help() {
    println!("");
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        b(),
        "ack; ",
        n(),
        "ext image; ",
        p(),
        "revious image; ",
        d_(),
        "ownload image;",
        o_(),
        "pen image in browser;\n",
        "show image in",
        f(),
        "ull res; ",
        q(),
        "uit (with confirmation); ",
        "view ",
        m(),
        "anual\n"
    )
}

pub fn user_help() {
    println!("");
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "view ",
        BLUE_N(),
        "th artist's illusts ",
        n(),
        "ext page; ",
        p(),
        "revious page; ",
        r(),
        "eload and re-download all;\n",
        q(),
        "uit (with confirmation);",
        "view ",
        m(),
        "anual\n"
    )
}
