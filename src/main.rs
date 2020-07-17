#![allow(dead_code)] // Temporary, remove later
#[macro_use]
mod utils;
mod colors;
mod config;
mod data;
mod files;
mod printer;
mod pure;
mod screens;

const KONEKODIR: &str = "~/.local/share/koneko/cache";
const __version__: &str = "0.0.1";

fn main() {
    printer::write("Hello, world!");
    //utils::open_in_browser("76695217")
}
