const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[39m";

pub fn BLUE_N() -> String {
    format!("{}[{}n{}]{}", RED, BLUE, RED, RESET)
}

fn COORDS() -> String {
    format!(
        "{red}{{{blue}x{red}}}{{{blue}y{red}}}{reset}",
        //    ^^            ^^^^            ^^
        //    '{'           '}{'            '}'
        red = RED,
        blue = BLUE,
        reset = RESET
    )
}

fn letter_with_brackets(letter: char) -> String {
    format!("{red}[{}{}{red}]{}", MAGENTA, letter, RESET, red = RED)
}

fn letter_with_coords(letter: char) -> String {
    format!(
        "{red}[{}{}{red}]{}{}",
        MAGENTA,
        letter,
        BLUE_N(),
        RESET,
        red = RED
    )
}

fn two_letter_with_coords(letter: char) -> String {
    format!(
        "{red}[{magenta}{}{reset}{}|{magenta}{}{}{red}]{reset}",
        letter.to_lowercase(),
        COORDS(),
        letter.to_uppercase(),
        BLUE_N(),
        red = RED,
        magenta = MAGENTA,
        reset = RESET
    )
}

pub fn n() -> String {
    letter_with_brackets('n')
}

pub fn p() -> String {
    letter_with_brackets('p')
}

pub fn r() -> String {
    letter_with_brackets('r')
}

pub fn q() -> String {
    letter_with_brackets('q')
}

pub fn m() -> String {
    letter_with_brackets('m')
}

pub fn b() -> String {
    letter_with_brackets('b')
}

pub fn o_() -> String {
    letter_with_brackets('o')
}

pub fn d_() -> String {
    letter_with_brackets('d')
}

pub fn f() -> String {
    letter_with_brackets('f')
}

pub fn i() -> String {
    letter_with_coords('i')
}

pub fn a() -> String {
    two_letter_with_coords('a')
}

pub fn o() -> String {
    two_letter_with_coords('o')
}

pub fn d() -> String {
    two_letter_with_coords('d')
}

pub fn base1() -> [String; 8] {
    [
        COORDS(),
        " view image at (x, y); ".to_string(),
        i(),
        " view nth image; ".to_string(),
        d(),
        " download image;\n".to_string(),
        o(),
        " open image in browser; ".to_string(),
    ]
}

pub fn base2() -> [String; 8] {
    [
        n(),
        "ext page; ".to_string(),
        p(),
        "revious page;\n".to_string(),
        r(),
        "eload and re-download all; ".to_string(),
        q(),
        "uit (with confirmation); ".to_string(),
    ]
}
