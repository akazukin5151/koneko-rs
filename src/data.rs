use std::convert::TryInto;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::*;

use crate::pure;
use crate::KONEKODIR;


pub enum DataStruct {
    Gallery,
    Image,
    UserData
}

pub struct Gallery {
    pub page_num: i32,
    pub main_path: PathBuf,
    pub offset: i32,
    pub all_pages_cache: HashMap<i32, Value>,
}

pub struct User {
    pub page_num: i32,
    pub main_path: PathBuf,
    pub offset: i32,
}

pub struct UserData {
    pub page_num: i32,
    pub main_path: PathBuf,
    pub offset: i32,
    pub next_url: String,
    pub ids_cache: HashMap<i32, Vec<String>>,
    pub names_cache: HashMap<i32, Vec<String>>,
    pub profile_pic_urls: Vec<String>,
    // This prevents all the Strings from being &str,
    // because of the loop in the implementation of User,
    // plus lifetime mismatches at `impl Data for UserData`
    pub image_urls: Vec<String>,
    pub splitpoint: i32,
}

pub struct Image<'a> {
    pub image_id: &'a str,
    pub artist_user_id: String,
    pub page_num: i32,
    pub page_urls: Vec<String>,
    pub number_of_pages: i32,
    pub download_path: PathBuf,
}

trait Data {
    fn update(&mut self, raw: &Value);

    fn download_path(&self) -> PathBuf;
    fn artist_user_id(&self, post_number: i32) -> String;
    fn next_url(&self) -> Option<String>;
    fn all_urls(&self) -> Vec<String>;
    fn all_names(&self) -> Vec<String>;

    fn urls_as_names(&self) -> Vec<String> {
        self.all_urls()
            .iter()
            .map(|x| pure::split_backslash_last(&x[..]).to_string())
            .collect()
    }

    fn newnames_with_ext(&self) -> Vec<String> {
        pure::newnames_with_ext(self.all_urls(), self.urls_as_names(), self.all_names())
    }
}

impl Data for Gallery {
    fn update(&mut self, raw: &Value) {
        self.all_pages_cache.insert(self.page_num, raw.clone());
    }

    fn download_path(&self) -> PathBuf {
        Path::new(&self.main_path).join(self.page_num.to_string())
    }

    fn artist_user_id(&self, post_number: i32) -> String {
        self.post_json(post_number)["user"]["id"].to_string()
    }

    fn next_url(&self) -> Option<String> {
        Some(
            self.all_pages_cache.get(&self.page_num)?["next_url"]
                .to_string()
                .replace("\"", ""),
        )
    }

    fn all_urls(&self) -> Vec<String> {
        pure::medium_urls(self.current_illusts().unwrap())
    }

    fn all_names(&self) -> Vec<String> {
        pure::post_titles_in_page(self.current_illusts().unwrap())
    }
}

impl Gallery {
    pub fn current_illusts(&self) -> Option<&Value> {
        Some(&self.all_pages_cache.get(&self.page_num)?["illusts"])
    }

    pub fn post_json(&self, post_number: i32) -> &Value {
        &self.current_illusts().unwrap()[post_number as usize]
    }

    pub fn image_id(&self, post_number: i32) -> String {
        self.post_json(post_number)["id"].to_string()
    }

    pub fn url(&self, number: i32) -> String {
        pure::url_given_size(self.post_json(number), "large")
    }
}

impl Data for UserData {
    fn update(&mut self, raw: &Value) {
        let user = User {
            page_num: self.page_num,
            main_path: self.main_path.clone(),
            offset: self.offset,
        };
        let newuserdata = user.update(raw);
        self.page_num = newuserdata.page_num;
        self.main_path = newuserdata.main_path;
        self.offset = newuserdata.offset;
        self.next_url = newuserdata.next_url;
        self.ids_cache = newuserdata.ids_cache;
        self.names_cache = newuserdata.names_cache;
        self.profile_pic_urls = newuserdata.profile_pic_urls;
        self.image_urls = newuserdata.image_urls;
        self.splitpoint = newuserdata.splitpoint;
    }

    fn download_path(&self) -> PathBuf {
        Path::new(&self.main_path).join(self.page_num.to_string())
    }

    fn artist_user_id(&self, selected_user_num: i32) -> String {
        self.ids_cache.get(&self.page_num).unwrap()[selected_user_num as usize].to_string()
    }

    fn next_url(&self) -> Option<String> {
        Some(self.next_url.clone())
    }

    fn all_urls(&self) -> Vec<String> {
        let mut result = self.profile_pic_urls.clone();
        result.extend(self.image_urls.clone());
        result
    }

    fn all_names(&self) -> Vec<String> {
        let mut preview_names: Vec<String> = vec![];
        self.image_urls.iter().for_each(|x| {
            preview_names.push(
                pure::split_backslash_last(&x)
                    .to_string()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string(),
            )
        });

        let mut result: Vec<String> = vec![];
        self.names().iter().for_each(|x| result.push(x.to_string()));
        preview_names
            .iter()
            .for_each(|x| result.push(x.to_string()));
        result
    }
}

trait Mappable {
    fn map_string<F>(&self, func: F) -> Vec<String>
    where
        F: Fn(&Value) -> &Value;
}

impl Mappable for Value {
    fn map_string<F>(&self, func: F) -> Vec<String>
    where
        F: Fn(&Value) -> &Value,
    {
        let mut result: Vec<String> = vec![];
        let mut i = 0;
        loop {
            let processed_elem = func(&self[i]);
            if processed_elem.is_null() {
                break;
            }
            result.push(processed_elem.to_string().replace("\"", ""));
            i += 1
        }
        result
    }
}

impl User {
    pub fn update(&self, raw: &Value) -> UserData {
        let next_url = raw["next_url"].to_string().replace("\"", "");
        let page = &raw["user_previews"];

        let ids = page.map_string(|x| &x["user"]["id"]);
        let mut ids_cache = HashMap::new();
        ids_cache.insert(self.page_num, ids);

        let names = page.map_string(|x| &x["user"]["name"]);
        let mut names_cache = HashMap::new();
        names_cache.insert(self.page_num, names);

        let profile_pic_urls = page.map_string(|x| &x["user"]["profile_image_urls"]["medium"]);
        let splitpoint = profile_pic_urls.len() as i32;

        let mut image_urls: Vec<String> = vec![];
        let mut i = 0;
        loop {
            let illust = &page[i]["illusts"];
            if illust.is_null() {
                break;
            }
            let mut j = 0;
            loop {
                let url = &illust[j]["image_urls"]["square_medium"];
                if url.is_null() {
                    break;
                }
                image_urls.push(url.to_string());
                j += 1;
            }
            i += 1;
        }

        UserData {
            page_num: self.page_num,
            main_path: self.main_path.clone(),
            offset: self.offset,
            next_url,
            ids_cache,
            names_cache,
            profile_pic_urls,
            image_urls,
            splitpoint,
        }
    }
}

impl UserData {
    pub fn names(&self) -> &Vec<String> {
        self.names_cache.get(&self.page_num).unwrap()
    }
}

pub fn new_imagedata<'a>(raw: &'a Value, image_id: &'a str) -> Image<'a> {
    let artist_user_id = raw["user"]["id"].to_string();
    let page_urls = pure::page_urls_in_post(raw, "large");
    let number_of_pages: i32 = page_urls.iter().len().try_into().unwrap();
    let mut download_path = Path::new(KONEKODIR)
        .join(&artist_user_id)
        .join("individual");
    if number_of_pages != 1 {
        download_path = download_path.join(image_id);
    }
    Image {
        image_id,
        artist_user_id,
        page_num: 0,
        page_urls,
        number_of_pages,
        download_path,
    }
}

impl Image<'_> {
    pub fn current_url(&self) -> &str {
        &self.page_urls[self.page_num as usize]
    }

    pub fn next_img_url(&self) -> &str {
        &self.page_urls[self.page_num as usize + 1]
    }

    pub fn image_filename(&self) -> &str {
        pure::split_backslash_last(self.current_url())
    }

    pub fn filepath(&self) -> PathBuf {
        self.download_path.join(self.image_filename())
    }

    pub fn large_filename(&self) -> &str {
        pure::split_backslash_last(&self.page_urls[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use rstest::*;
    use maplit::hashmap;

    #[fixture]
    fn gallery_json() -> Value {
        let file = fs::File::open("testing/files/mode1.json").unwrap();
        let mode1: serde_json::Value = serde_json::from_reader(file).unwrap();
        mode1.clone()
    }

    #[fixture]
    fn user_json() -> Value {
        let file = fs::File::open("testing/files/mode3.json").unwrap();
        let mode3: serde_json::Value = serde_json::from_reader(file).unwrap();
        mode3.clone()
    }

    #[fixture]
    fn image_json() -> Value {
        let file = fs::File::open("testing/files/mode2.json").unwrap();
        let mode2: serde_json::Value = serde_json::from_reader(file).unwrap();
        mode2["illust"].clone()
    }

    #[fixture]
    fn gallery() -> Gallery {
        Gallery {
            page_num: 1,
            main_path: Path::new(KONEKODIR).join("2232374"),
            offset: 0,
            all_pages_cache: HashMap::new(),
        }
    }

    #[fixture]
    fn gallery_updated(gallery_json: Value) -> Gallery {
        let mut gdata = gallery();
        gdata.update(&gallery_json);
        gdata
    }

    #[fixture]
    fn user() -> User {
        User {
            page_num: 1,
            main_path: Path::new(KONEKODIR).join("following/2232374"),
            offset: 0,
        }
    }

    #[fixture]
    fn user_updated(user_json: Value) -> UserData {
        let udata = user();
        udata.update(&user_json)
    }

    #[rstest]
    fn test_gallery_init() {
        let gdata = gallery();
        assert_eq!(gdata.page_num, 1);
        assert_eq!(gdata.offset, 0);
        assert_eq!(gdata.all_pages_cache, HashMap::new());
    }

    #[rstest]
    fn test_gallery_update(gallery_json: Value) {
        let mut gdata = gallery();
        gdata.update(&gallery_json);
        assert_eq!(gdata.all_pages_cache.keys().next().unwrap(), &1)
    }

    #[rstest]
    fn test_gallery_download_path(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(
            gdata.download_path(),
            Path::new(KONEKODIR).join("2232374/1")
        )
    }

    #[rstest]
    fn test_gallery_current_illusts(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.current_illusts().unwrap().is_array(), true);
        assert_eq!(
            gdata.current_illusts().unwrap().as_array().unwrap().len(),
            30
        )
    }

    #[rstest]
    fn test_gallery_post_json(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.post_json(0).is_object(), true);

        let v: Value = from_str(r#"{"id":81547984,"title":"みこっちゃん","type":"illust","image_urls":{"square_medium":"https://i.pximg.net/c/540x540_10_webp/img-master/img/2020/05/14/06/45/24/81547984_p0_square1200.jpg","medium":"https://i.pximg.net/c/540x540_70/img-master/img/2020/05/14/06/45/24/81547984_p0_master1200.jpg","large":"https://i.pximg.net/c/600x1200_90_webp/img-master/img/2020/05/14/06/45/24/81547984_p0_master1200.jpg"},"caption":"( ˘ω˘ )ﾃﾞｽ","restrict":0,"user":{"id":2232374,"name":"raika9","account":"raika9","profile_image_urls":{"medium":"https://i.pximg.net/user-profile/img/2016/06/30/03/20/52/11132477_4b836884eae72b4e90061719fd75180b_170.jpg"},"is_followed":true},"tags":[{"name":"とある科学の超電磁砲","translated_name":null},{"name":"とある魔術の禁書目録","translated_name":null},{"name":"御坂美琴","translated_name":null}],"tools":["CLIP STUDIO PAINT"],"create_date":"2020-05-14T06:45:24+09:00","page_count":1,"width":764,"height":1087,"sanity_level":2,"x_restrict":0,"series":null,"meta_single_page":{"original_image_url":"https://i.pximg.net/img-original/img/2020/05/14/06/45/24/81547984_p0.jpg"},"meta_pages":[],"total_view":8021,"total_bookmarks":2324,"is_bookmarked":false,"visible":true,"is_muted":false,"total_comments":54}"#).unwrap();
        assert_eq!(gdata.post_json(0), &v);
    }

    #[rstest]
    fn test_gallery_artist_user_id(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.artist_user_id(0), "2232374");
    }

    #[rstest]
    fn test_gallery_image_id(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.image_id(0), "81547984");
    }

    #[rstest]
    fn test_gallery_next_url(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.next_url().unwrap(), "https://app-api.pixiv.net/v1/user/illusts?user_id=2232374&filter=for_ios&type=illust&offset=30");
    }

    #[rstest]
    fn test_gallery_url(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.url(0), "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2020/05/14/06/45/24/81547984_p0_master1200.jpg");
    }

    #[rstest]
    fn test_gallery_all_urls(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(gdata.all_urls()[..3], ["https://i.pximg.net/c/540x540_10_webp/img-master/img/2020/05/14/06/45/24/81547984_p0_square1200.jpg", "https://i.pximg.net/c/540x540_10_webp/img-master/img/2020/05/12/06/36/27/81501385_p0_square1200.jpg", "https://i.pximg.net/c/540x540_10_webp/img-master/img/2020/05/10/23/10/38/81468125_p0_square1200.jpg"])
    }

    #[rstest]
    fn test_gallery_all_names(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);
        assert_eq!(
            gdata.all_names(),
            [
                "みこっちゃん",
                "おりじなる",
                "0510",
                "5.3",
                "おりじなる",
                "ミコ誕オメ画！",
                "5.2",
                "5.1",
                "310",
                "Midnight Sun",
                "222",
                "バレンタイン",
                "祝！！！",
                "あけましておめでとうございます",
                "ミコサンタ",
                "C97告知",
                "ミコバニー",
                "たちかわ楽市2019仕様4人組",
                "ハロミコ",
                "夏服",
                "御坂美琴写真集１０用",
                "常盤台中学指定体操服改",
                "ツイッターまとめ",
                "スクミズミコクロ",
                "ミズミコ",
                "ミコニャン",
                "とある画帖",
                "御坂美琴写真集９",
                "ジャンプ！",
                "シャワミコ"
            ]
        );
    }

    #[rstest]
    fn test_urls_as_names_gdata(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);

        assert_eq!(
            gdata.urls_as_names(),
            vec![
                "81547984_p0_square1200.jpg",
                "81501385_p0_square1200.jpg",
                "81468125_p0_square1200.jpg",
                "81416496_p0_square1200.jpg",
                "81368866_p0_square1200.jpg",
                "81276257_p0_square1200.jpg",
                "80923496_p0_square1200.jpg",
                "80701898_p0_square1200.jpg",
                "80017594_p0_square1200.jpg",
                "79799236_p0_square1200.jpg",
                "79658392_p0_square1200.jpg",
                "79549991_p0_square1200.jpg",
                "78823485_p0_square1200.jpg",
                "78628383_p0_square1200.jpg",
                "78403815_p0_square1200.jpg",
                "78378594_p0_square1200.jpg",
                "78201587_p0_square1200.jpg",
                "77804404_p0_square1200.jpg",
                "77565309_p0_square1200.jpg",
                "77460464_p0_square1200.jpg",
                "77347697_p0_square1200.jpg",
                "77068750_p0_square1200.jpg",
                "76695217_p0_square1200.jpg",
                "76561671_p0_square1200.jpg",
                "76138362_p0_square1200.jpg",
                "75933779_p0_square1200.jpg",
                "75810852_p0_square1200.jpg",
                "75698678_p0_square1200.jpg",
                "75579060_p0_square1200.jpg",
                "75457783_p0_square1200.jpg"
            ]
        )
    }

    #[rstest]
    fn test_newnames_with_ext_gdata(gallery_json: Value) {
        let gdata = gallery_updated(gallery_json);

        assert_eq!(
            gdata.newnames_with_ext(),
            [
                "000_みこっちゃん.jpg",
                "001_おりじなる.jpg",
                "002_0510.jpg",
                "003_5.3.jpg",
                "004_おりじなる.jpg",
                "005_ミコ誕オメ画！.jpg",
                "006_5.2.jpg",
                "007_5.1.jpg",
                "008_310.jpg",
                "009_Midnight Sun.jpg",
                "010_222.jpg",
                "011_バレンタイン.jpg",
                "012_祝！！！.jpg",
                "013_あけましておめでとうございます.jpg",
                "014_ミコサンタ.jpg",
                "015_C97告知.jpg",
                "016_ミコバニー.jpg",
                "017_たちかわ楽市2019仕様4人組.jpg",
                "018_ハロミコ.jpg",
                "019_夏服.jpg",
                "020_御坂美琴写真集１０用.jpg",
                "021_常盤台中学指定体操服改.jpg",
                "022_ツイッターまとめ.jpg",
                "023_スクミズミコクロ.jpg",
                "024_ミズミコ.jpg",
                "025_ミコニャン.jpg",
                "026_とある画帖.jpg",
                "027_御坂美琴写真集９.jpg",
                "028_ジャンプ！.jpg",
                "029_シャワミコ.jpg"
            ]
        )
    }

    #[rstest]
    fn test_user_init() {
        let udata = user();
        assert_eq!(udata.page_num, 1);
        assert_eq!(
            udata.main_path,
            Path::new(KONEKODIR).join("following/2232374")
        );
        assert_eq!(udata.offset, 0);
    }

    #[rstest]
    fn test_user_update(user_json: Value) {
        let data = user();
        let udata = data.update(&user_json);

        assert_eq!(udata.next_url, "https://app-api.pixiv.net/v1/user/following?user_id=2232374&restrict=private&offset=30");

        assert_eq!(
            udata.ids_cache,
            hashmap! {1 => vec!["219621".to_string(), "1510169".to_string(), "12612404".to_string(), "8660134".to_string(), "15063".to_string(), "28245700".to_string(), "33137265".to_string(), "2702224".to_string(), "24218478".to_string(), "625051".to_string(), "95391".to_string(), "9427".to_string(), "1193008".to_string(), "1554775".to_string(), "11103".to_string(), "7309825".to_string(), "5301174".to_string(), "4316556".to_string(), "10573236".to_string(), "29362997".to_string(), "809099".to_string(), "82688".to_string(), "15608555".to_string(), "30803054".to_string(), "18836733".to_string(), "644670".to_string(), "2397243".to_string(), "14211481".to_string(), "8092144".to_string(), "8175661".to_string()]}
        );

        assert_eq!(
            udata.names_cache,
            hashmap! {1 => vec!["畳と桧".to_string(), "ざるつ".to_string(), "春夫".to_string(), "JAM".to_string(), "肋兵器".to_string(), "おてん!!!!!!!!".to_string(), "saber".to_string(), "sola7764".to_string(), "￦ANKE".to_string(), "ToY".to_string(), "sigma99".to_string(), "アマガイタロー".to_string(), "望月けい".to_string(), "米山舞".to_string(), "にえあ@冬コミ新刊委託中です".to_string(), "白萝炖黑兔".to_string(), "Kelinch1".to_string(), "三崎二式.N3".to_string(), "ﾕｳｷ".to_string(), "sunhyunそんひょん선현".to_string(), "うまくち醤油".to_string(), "Prime".to_string(), "哦雅思密乃".to_string(), "ホリセイ".to_string(), "pattsk138".to_string(), "DELF".to_string(), "キンタ".to_string(), "cookies".to_string(), "Aluppia".to_string(), "うにゃりすたー".to_string()]}
        );

        assert_eq!(udata.profile_pic_urls.iter().len(), 30);

        assert_eq!(udata.image_urls.iter().len(), 87);
    }

    #[rstest]
    fn test_user_download_path(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(
            udata.download_path(),
            Path::new(KONEKODIR).join("following/2232374/1")
        );
    }

    #[rstest]
    fn test_user_artist_user_id(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(udata.artist_user_id(0), "219621");
    }

    #[rstest]
    fn test_user_names(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(udata.names(), udata.names_cache.get(&1).unwrap());
    }

    #[rstest]
    fn test_user_all_urls(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(udata.all_urls().iter().len(), 117);
    }

    #[rstest]
    fn test_user_all_names(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(
            udata.all_names()[..10],
            [
                "畳と桧".to_string(),
                "ざるつ".to_string(),
                "春夫".to_string(),
                "JAM".to_string(),
                "肋兵器".to_string(),
                "おてん!!!!!!!!".to_string(),
                "saber".to_string(),
                "sola7764".to_string(),
                "￦ANKE".to_string(),
                "ToY".to_string()
            ]
        );

        assert_eq!(
            udata
                .all_names()
                .iter()
                .rev()
                .take(10)
                .rev()
                .collect::<Vec<&String>>(),
            [
                &"76547709_p0_square1200".to_string(),
                &"79708221_p0_square1200".to_string(),
                &"76623178_p0_square1200".to_string(),
                &"74653820_p0_square1200".to_string(),
                &"81542404_p0_square1200".to_string(),
                &"80414334_p0_square1200".to_string(),
                &"79663557_p0_square1200".to_string(),
                &"79028150_p0_square1200".to_string(),
                &"79027961_p0_square1200".to_string(),
                &"79027291_p0_square1200".to_string()
            ]
        );
    }

    #[rstest]
    fn test_user_splitpoint(user_json: Value) {
        let udata = user_updated(user_json);
        assert_eq!(udata.splitpoint, 30);
    }

    #[rstest]
    fn test_image_artist_user_id(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(idata.artist_user_id, "2232374");
    }

    #[rstest]
    fn test_image_page_num(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(idata.page_num, 0);
    }

    #[rstest]
    fn test_image_number_of_pages(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(idata.number_of_pages, 8);
    }

    #[rstest]
    fn test_image_page_urls(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(idata.page_urls, ["https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p0_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p1_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p2_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p3_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p4_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p5_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p6_master1200.jpg", "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p7_master1200.jpg"]);
    }

    #[rstest]
    fn test_image_download_path(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(
            idata.download_path,
            Path::new(KONEKODIR).join("2232374/individual/76695217")
        );
    }

    #[rstest]
    fn test_image_image_filename(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(idata.image_filename(), "76695217_p0_master1200.jpg");
    }

    #[rstest]
    fn test_image_filepath(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(
            idata.filepath(),
            Path::new(KONEKODIR).join("2232374/individual/76695217/76695217_p0_master1200.jpg")
        );
    }

    #[rstest]
    fn test_image_next_img_url(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(
            idata.next_img_url(),
            "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p1_master1200.jpg"
        );
    }

    #[rstest]
    fn test_image_current_url(image_json: Value) {
        let idata = new_imagedata(&image_json, "76695217");
        assert_eq!(
            idata.current_url(),
            "https://i.pximg.net/c/600x1200_90_webp/img-master/img/2019/09/09/04/32/38/76695217_p0_master1200.jpg"
        );
    }
}
