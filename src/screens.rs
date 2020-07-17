use std::process::Command;
use std::io;

use crate::{__version__, KONEKODIR};

pub fn begin_prompt(printmessages: bool) -> String {
    let messages = [
        &format!("Welcome to koneko v{}\n", __version__),
        "Select an action:",
        "1. View artist illustrations",
        "2. Open pixiv post",
        "3. View following artists",
        "4. Search for artists",
        "5. View illustrations of all following artists",
        "f. Frequent modes and user inputs", "",
        "?. Info",
        "m. Manual",
        "c. Clear koneko cache",
        "q. Quit",
    ];
    if printmessages {
        messages.iter().for_each(|m| println!("{}", m))
    };
    // TODO: show pixcat image
    let size = Command::new("du").args(&["-hs", "--apparent-size", &KONEKODIR]).output().unwrap();
    // FIXME: KONEKODIR not found
    //du -hs --apparent-size {KONEKODIR} | cut -f1
    //println!("Current cache size = {}", size.stdout);
    print!("Enter a command: ");
    let mut command = String::new();
    io::stdin().read_line(&mut command);
    command
}

pub fn show_man_loop() {
    Command::new("clear").spawn();
    // println!(docs);
    println!("{}{}", " ".repeat(3), "=".repeat(30));
    let mut command = String::new();
    loop {
        print!("\n\nEnter any key to return: ");
        match io::stdin().read_line(&mut command) {
            Ok(_) => break,
            _ => continue,
        }
    };
    Command::new("clear").spawn();
}

pub fn clear_cache_loop() {
    println!("Do you want to remove all cached images?");
    println!("This will not remove images you explicitly downloaded to ~/Downloads.");
    println!("Directory to be deleted: {}", KONEKODIR);
    let mut command = String::new();
    loop {
        print!("\nEnter y to confirm: ");
        match io::stdin().read_line(&mut command) {
            Ok(2) => break, // usize for 'y'
            _ => println!("Operation aborted!")
        }
    }
    Command::new("clear").spawn();
}

pub fn info_screen_loop() {
    let messages = [
        &format!("koneko こねこ version {} beta\n", __version__),
        "Browse pixiv in the terminal using kitty's icat to display",
        "images with images embedded in the terminal\n",
        "1. View an artist's illustrations",
        "2. View a post (support multiple images)",
        "3. View artists you followed",
        "4. Search for artists and browse their works.",
        "5. View latest illustrations from artist you follow.\n",
        "Thank you for using koneko!",
        "Please star, report bugs and contribute in:",
        "https://github.com/twenty5151/koneko",
        "GPLv3 licensed\n",
        "Credits to amasyrup (甘城なつき):",
        "Welcome image: https://www.pixiv.net/en/artworks/71471144",
        "Current image: https://www.pixiv.net/en/artworks/79494300",
    ];
    messages.iter().for_each(|m| println!("{}", m));
    // TODO: show pixcat image
    let mut command = String::new();
    loop {
        print!("\nEnter any key to return: ");
        match io::stdin().read_line(&mut command) {
            Ok(_) => return,
            _ => continue,
        }
    };
}
