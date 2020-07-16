use std::path::PathBuf;
use std::convert::TryInto;

use regex::Regex;
use serde_json::*;

pub fn str_is_digit(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

pub fn split_backslash_last(s: &str) -> &str {
    s.split('/').last().unwrap()
}

pub fn generate_filepath(filename: &str) -> PathBuf {
    dirs::home_dir().unwrap().join("Downloads").join(filename)
}

pub fn prefix_filename(oldname_with_ext: &str, newname: &str, number: i32) -> String {
    let image_ext = oldname_with_ext.split('.').last().unwrap();
    let number_prefix = format!("{:0>3}", number);
    format!("{}_{}.{}", number_prefix, newname, image_ext)
}

pub fn prefix_artist_name(name: &str, number: i32) -> String {
    let number_prefix = format!("{:0>2}", number);
    format!("{}{}{}", number_prefix, " ".repeat(19), name)
}

pub fn url_given_size(post_json: &Value, size: &str) -> String {
    post_json["image_urls"][size].to_string().replace("\"", "")
}

pub fn post_title(page_illusts: &Value, post_number: usize) -> String {
    page_illusts[post_number]["title"]
        .to_string()
        .replace("\"", "")
}

pub fn medium_urls(page_illusts: &Value) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut i = 0;
    loop {
        let post_json = &page_illusts[i];
        if post_json.is_null() {
            break;
        };
        result.push(url_given_size(post_json, "square_medium"));
        i += 1;
    }
    result
}

pub fn post_titles_in_page(page_illusts: &Value) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut i = 0;
    loop {
        let post_json = &page_illusts[i];
        if post_json.is_null() {
            break;
        };
        result.push(post_title(page_illusts, i));
        i += 1;
    }
    result
}

pub fn page_urls_in_post(post_json: &Value, size: &str) -> Vec<String> {
    let number_of_pages: usize = post_json["page_count"]
        .as_i64()
        .unwrap()
        .try_into()
        .unwrap();
    if number_of_pages > 1 {
        let list_of_pages = &post_json["meta_pages"];
        let mut result: Vec<String> = vec![];
        for i in 0..number_of_pages {
            result.push(url_given_size(&list_of_pages[i], size));
        }
        result
    } else {
        vec![url_given_size(&post_json, size)]
    }
}

pub fn change_url_to_full(url: &str, png: bool) -> String {
    let re1 = Regex::new(r"_master\d+").unwrap();
    let re2 = Regex::new(r"c/\d+x\d+_\d+_\w+/img-master").unwrap();
    let newurl1 = re1.replace(url, "");
    let newurl2 = re2.replace(&newurl1, "img-original");

    if png {
        let newurl3 = newurl2.replace("jpg", "png");
        newurl3
    } else {
        newurl2.to_string()
    }
}

pub fn process_user_url(url_or_id: &str) -> &str {
    if url_or_id.contains("users") {
        split_backslash_last(url_or_id)
    } else {
        url_or_id
    }
}

pub fn process_artwork_url(url_or_id: &str) -> &str {
    if url_or_id.contains("artworks") {
        split_backslash_last(url_or_id)
            .split('\\')
            .collect::<Vec<&str>>()[0]
    } else if url_or_id.contains("illust_id") {
        let re = Regex::new(r"&illust_id.*").unwrap();
        re.captures(url_or_id)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .split('=')
            .last()
            .unwrap()
    } else {
        url_or_id
    }
}

pub fn newnames_with_ext(
    urls: Vec<String>,
    oldname_with_ext: Vec<String>,
    newnames: Vec<String>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 0..urls.len() {
        result.push(prefix_filename(
            &oldname_with_ext[i],
            &newnames[i],
            i.try_into().unwrap(),
        ));
    }
    result
}

pub fn full_image_details(url: &str, png: bool) -> (String, &str, PathBuf) {
    let fullurl = change_url_to_full(url, png);
    let filename = split_backslash_last(url);
    let filepath = generate_filepath(filename);
    (fullurl, filename, filepath)
}

pub fn concat_seq_to_int(keyseqs: Vec<&str>, start: i32) -> i32 {
    let idx: usize = start.try_into().unwrap();
    let first = keyseqs[idx];
    let second = keyseqs[idx + 1];
    format!("{}{}", first, second).parse::<i32>().unwrap()
}

pub fn ncols(term_width: u16, image_width: i32, padding: i32) -> i32 {
    (term_width as f32 / (image_width as f32 + padding as f32)).round() as i32
}

pub fn nrows(term_height: u16, image_height: i32, padding: i32) -> i32 {
    ((term_height as f32).div_euclid(image_height as f32 + padding as f32)) as i32
}

pub fn xcoords(term_width: u16, image_width: i32, padding: i32, offset: i32) -> Vec<i32> {
    let number_of_columns = ncols(term_width, image_width, padding);
    (0..number_of_columns)
        .map(|col| {
            ((col as f32).rem_euclid(number_of_columns as f32) * image_width as f32
                + padding as f32
                + offset as f32) as i32
        })
        .collect()
}

pub fn ycoords(term_height: u16, image_height: i32, padding: i32) -> Vec<i32> {
    let number_of_rows = nrows(term_height, image_height, padding);
    (0..number_of_rows)
        .map(|row| row * (image_height + padding))
        .collect()
}

pub fn generate_orders(total_pics: i32, artist_count: i32) -> Vec<i32> {
    let range: Vec<i32> = (0..artist_count).collect();
    let mut i = 0;
    let mut order: Vec<i32> = (0..total_pics)
        .map(|x| x + artist_count - 1 - ((x as f32 / 4f32).floor() as i32))
        .collect();

    for (idx, num) in order.iter_mut().enumerate() {
        if idx.rem_euclid(4) == 0 {
            *num = range[i];
            i += 1
        }
    }
    order
}

pub fn line_width(spacings: Vec<i32>, ncols: i32) -> i32 {
    spacings.iter().sum::<i32>() + ncols
}

pub fn all_isdigit(keyseqs: Vec<&str>) -> bool {
    keyseqs
        .iter()
        .all(|&s| s.chars().next().unwrap().is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_split_backslash_last() {
        assert_eq!(
            split_backslash_last(&"https://www.pixiv.net/en/users/2232374"),
            "2232374"
        );
        assert_eq!(
            split_backslash_last(&"https://www.pixiv.net/en/artworks/78823485"),
            "78823485"
        );
    }

    #[test]
    fn test_generate_filepath() {
        assert_eq!(
            generate_filepath(&"78823485_p0.jpg"),
            dirs::home_dir().unwrap().join("Downloads/78823485_p0.jpg")
        );
    }

    #[test]
    fn test_prefix_filename() {
        assert_eq!(prefix_filename(&"old.jpg", &"new", 2), "002_new.jpg");
        assert_eq!(prefix_filename(&"old.jpg", &"new", 10), "010_new.jpg");
    }

    #[test]
    fn test_prefix_artist_name() {
        assert_eq!(prefix_artist_name("name1", 2), "02                   name1");
        assert_eq!(
            prefix_artist_name("name1", 10),
            "10                   name1"
        );
    }

    #[test]
    fn all() {
        let file = fs::File::open("testing/files/mode1.json").unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        // current_page_illusts
        let current_illust = json.get("illusts").expect("file should have illust key");

        let x = &current_illust[0]; // post_json()
        assert_eq!(url_given_size(x, &"medium"), "https://i.pximg.net/c/540x540_70/img-master/img/2020/05/14/06/45/24/81547984_p0_master1200.jpg".to_string());
        assert_eq!(url_given_size(&current_illust[1], &"large"), "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2020/05/12/06/36/27/81501385_p0_master1200.jpg".to_string());

        assert_eq!(post_title(current_illust, 0usize), "みこっちゃん");
        assert_eq!(post_title(current_illust, 1usize), "おりじなる");

        assert_eq!(medium_urls(current_illust)[0], "https://i.pximg.net/c/540x540_10_webp/img-master/img/2020/05/14/06/45/24/81547984_p0_square1200.jpg".to_string());

        assert_eq!(post_titles_in_page(current_illust).len(), 30);
        assert_eq!(
            post_titles_in_page(current_illust)[0],
            "みこっちゃん".to_string()
        );
        assert_eq!(
            post_titles_in_page(current_illust)[1],
            "おりじなる".to_string()
        );

        assert_eq!(page_urls_in_post(&current_illust[22], "medium"), vec!["https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p0_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p1_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p2_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p3_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p4_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p5_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p6_master1200.jpg", "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p7_master1200.jpg"]);
        assert_eq!(page_urls_in_post(&current_illust[22], "medium").len(), 8);

        assert_eq!(page_urls_in_post(&current_illust[0], "medium").len(), 1);
        assert_eq!(page_urls_in_post(&current_illust[0], "medium"), vec!["https://i.pximg.net/c/540x540_70/img-master/img/2020/05/14/06/45/24/81547984_p0_master1200.jpg"]);
    }

    #[test]
    fn test_change_url_to_full() {
        assert_eq!(change_url_to_full("https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p0_master1200.jpg", false), "https://i.pximg.net/c/540x540_70/img-master/img/2019/09/09/04/32/38/76695217_p0.jpg");
    }

    #[test]
    fn test_process_user_url() {
        assert_eq!(
            process_user_url("https://www.pixiv.net/en/users/2232374"),
            "2232374"
        );
        assert_eq!(process_user_url("2232374"), "2232374");
    }

    #[test]
    fn test_process_artwork_url() {
        assert_eq!(
            process_artwork_url("https://www.pixiv.net/en/artworks/76695217"),
            "76695217"
        );
        assert_eq!(
            process_artwork_url(
                "http://www.pixiv.net/member_illust.php?mode=medium&illust_id=76695217"
            ),
            "76695217"
        );
    }

    #[test]
    fn test_newnames_with_ext() {
        assert_eq!(
            newnames_with_ext(
                vec![
                    "www.example.com/image1.png".to_string(),
                    "www.example.com/image2.png".to_string(),
                    "www.example.com/image3.png".to_string()
                ],
                vec![
                    "image1.png".to_string(),
                    "image2.png".to_string(),
                    "image3.png".to_string()
                ],
                vec!["pic1".to_string(), "pic2".to_string(), "pic3".to_string()]
            ),
            vec!["000_pic1.png", "001_pic2.png", "002_pic3.png"]
        );
    }

    #[test]
    fn test_xcoords() {
        assert_eq!(xcoords(100, 18, 2, 0), vec![2, 20, 38, 56, 74]);
    }

    #[test]
    fn test_ycoords() {
        assert_eq!(ycoords(20, 8, 1), vec![0, 9]);
    }

    #[test]
    fn test_generate_orders() {
        assert_eq!(
            generate_orders(120, 30),
            vec![
                0, 30, 31, 32, 1, 33, 34, 35, 2, 36, 37, 38, 3, 39, 40, 41, 4, 42, 43, 44, 5, 45,
                46, 47, 6, 48, 49, 50, 7, 51, 52, 53, 8, 54, 55, 56, 9, 57, 58, 59, 10, 60, 61, 62,
                11, 63, 64, 65, 12, 66, 67, 68, 13, 69, 70, 71, 14, 72, 73, 74, 15, 75, 76, 77, 16,
                78, 79, 80, 17, 81, 82, 83, 18, 84, 85, 86, 19, 87, 88, 89, 20, 90, 91, 92, 21, 93,
                94, 95, 22, 96, 97, 98, 23, 99, 100, 101, 24, 102, 103, 104, 25, 105, 106, 107, 26,
                108, 109, 110, 27, 111, 112, 113, 28, 114, 115, 116, 29, 117, 118, 119
            ]
        );
    }

    #[test]
    fn test_all_isdigit() {
        assert_eq!(all_isdigit(vec!["1", "4"]), true);
        assert_eq!(all_isdigit(vec!["1", "x"]), false);
        assert_eq!(all_isdigit(vec!["1", "f"]), false);
    }
}
