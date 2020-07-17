mod lscat;
mod api;

use std::fs;
use std::thread;
use std::string::String;
use std::convert::TryInto;
use std::sync::mpsc::{channel, Receiver, Sender};

use pyo3::prelude::Python;
use pyo3::types::{PyDict,IntoPyDict};


// Concurrency
fn producer(sender: Sender<String>, files: &[String]) {
    // TODO: Actually implement the downloader
    files.iter().for_each(|f| {
        sender.send(f.to_string()).unwrap();
    });
    drop(sender);
}

fn consumer(tracker: Receiver<String>, image: Sender<String>) {
    let mut downloaded: Vec<i32> = vec![];
    let mut paths: Vec<String> = vec![];
    let mut orders: Vec<i32> = (0..=29).collect();

    consumer_loop(&tracker, &image, &mut downloaded, &mut paths, &mut orders);
}

fn consumer_loop(
    tracker: &Receiver<String>,
    image: &Sender<String>,
    downloaded: &mut Vec<i32>,
    paths: &mut Vec<String>,
    orders: &mut Vec<i32>,
) {
    loop {
        let number: i32 = match orders.iter().next() {
            Some(num) => *num,
            _ => return,
        };

        if downloaded.iter().any(|&x| x == number) {
            // Send to display channel
            image
                .send(int_to_file(&number, paths))
                .unwrap_or_else(|_| panic!("Can not send to display channel!"));

            // Inspect int list with next accepted number
            downloaded.retain(|&x| x != number);
            orders.remove(0);
            continue;
        }

        // On receiving a completed download, store the filename number
        let msg = &tracker.recv();
        match msg {
            Ok(img) => {
                downloaded.push(file_to_int(img));
                paths.push(img.to_string());
                continue;
            }
            Err(_) => return,
        }
    }
}

fn int_to_file(num: &i32, paths: &[String]) -> String {
    paths
        .iter()
        .filter(|x| file_to_int(x) == *num)
        .next()
        .unwrap_or_else(|| panic!("Can not find a file with the leading int!"))
        .to_string()
}

fn file_to_int(x: &str) -> i32 {
    x.split('/')
        .last()
        .unwrap_or_else(|| panic!("Path does not contain any '/'s!"))
        .split('_')
        .next()
        .unwrap_or_else(|| panic!("Filename does not contain any '_'s!"))
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Failed to parse leading int in filename!"))
}


fn display(image: Receiver<String>, py: Python, locals: &PyDict) {
    let leftshifts: [i32; 5] = [0, 20, 38, 56, 74];
    let rowspaces: [i32; 2] = [0, 9];
    let number_of_cols = 5;
    let number_of_rows = 2;

    loop {
        let msg = image.recv();
        match msg {
            Ok(img) => {
                let num: usize = file_to_int(&img)
                    .try_into()
                    .unwrap_or_else(|_| panic!("Image does not have a leading int!"));
                let x = num.rem_euclid(5);
                let y = (num / number_of_cols).rem_euclid(number_of_rows);

                if num.rem_euclid(number_of_cols * number_of_rows) == 0 && num != 0 {
                    println!("{}", "\n".repeat(23));
                }

                let data = lscat::Data {
                    path: &img,
                    size: 310,
                    x: leftshifts[x],
                    y: rowspaces[y],
                };
                lscat::run(py, locals, data);
            }
            Err(_) => return,
        }
    }
}

fn sample_files_setup() -> Vec<String> {
    let homedir = dirs::home_dir().unwrap();
    let path = homedir.join(".local/share/koneko/cache/2232374/1");
    let files = fs::read_dir(path).unwrap();

    files.map(|f| {
        f.unwrap()
            .path()
            .as_path()
            .to_str()
            .unwrap()
            .to_string()
    }).collect()
}

fn main() {
    // Setup python
    let import_err = |_| panic!("Failed to import pixcat! Make sure it is installed in your Python environment");
    let gil = Python::acquire_gil();
    let py = gil.python();

    // API test
    //let locals = [
    //    ("api", py.import("koneko.api").unwrap_or_else(import_err)),
    //    ("json", py.import("json").unwrap_or_else(import_err))
    //].into_py_dict(py);

    // Your username and password goes here
    //let data = api::Data {username: "XXX", password: "XXX"};
    //api::run(py, locals, data);

    let locals = [("pixcat", py.import("pixcat").unwrap_or_else(import_err))].into_py_dict(py);
    // Setup channels and threads
    let (tracker_s, tracker_r) = channel();
    let (image_s, image_r) = channel();

    thread::spawn(move || {
        producer(tracker_s, &sample_files_setup());
    });

    thread::spawn(move || {
        consumer(tracker_r, image_s);
    });

    display(image_r, py, locals);
}
