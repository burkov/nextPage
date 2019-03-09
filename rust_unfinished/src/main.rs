extern crate byteorder;
extern crate clipboard;
extern crate reqwest;
extern crate scraper;

use std::collections::HashMap;
use std::fs::{create_dir, OpenOptions, read_to_string};
use std::fs;
use std::io::{ErrorKind, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Once;
use std::time::Duration;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use clipboard::ClipboardProvider;
use clipboard::x11_clipboard::{Clipboard, X11ClipboardContext};
use dirs::data_dir;
use scraper::{Html, Selector};
use spinners::{Spinner, Spinners};

fn copy_to_clipboard(s: &str) {
    let mut ctx: X11ClipboardContext<Clipboard> = clipboard::ClipboardProvider::new().unwrap();
    ctx.set_contents(s.to_string()).unwrap();
}

fn wait_and_exit(d: Duration) {
    assert!(d > Duration::from_secs(1), "should be longer than 1 second!");
    fn message(n: u64) -> String {
        format!("You have {} seconds to paste your text", n)
    }
    let spinner = Spinner::new(Spinners::Dots12, message(d.as_secs()));
    for i in (0..d.as_secs()).rev() {
        spinner.message(message(i));
        std::thread::sleep(Duration::from_secs(1));
    }
    spinner.stop();
    print!("\n");
    exit(0);
}

#[derive(Debug)]
struct Tokens {
    csrf: String,
    cookie: String,
}


fn next_page(tokens: &Tokens) {
//    let mut map = HashMap::new();
//    map.insert("booknumber", BOOKNUMBER);
//    CLIENT.post("http://recommendmeabook.com/next_book").form()
}

struct NextBook {
    current_booknumber: u64,
    tokens: Tokens,
    client: reqwest::Client,
}

static CREATE_DATA_DIR: Once = Once::new();

impl NextBook {
    fn new() -> NextBook {
        NextBook {
            tokens: NextBook::get_tokens(),
            client: reqwest::Client::new(),
            current_booknumber: NextBook::load_booknumber(),
        }
    }

    fn store_booknumber(n: u64) {
        std::fs::write(NextBook::booknumber_path(), n.to_string()).unwrap();
    }

    fn datadir_path() -> PathBuf {
        let result = dirs::data_local_dir().unwrap().join("nextpage");
        CREATE_DATA_DIR.call_once(|| {
            match create_dir(&result) {
                Ok(_) => (),
                Err(ref x) if x.kind() == ErrorKind::AlreadyExists => (),
                other => panic!("{:?}", other),
            }
        });
        result
    }

    fn booknumber_path() -> PathBuf {
        NextBook::datadir_path().join("booknumber")
    }

    fn load_booknumber() -> u64 {
        match std::fs::read_to_string(NextBook::booknumber_path()) {
            Ok(s) => s.parse().unwrap(),
            Err(_) => 0
        }
    }

    fn page(&mut self) -> String {
        let mut body = HashMap::new();
        let mut result = String::new();
        loop {
            body.insert("booknumber", self.current_booknumber);
            let mut response = self.client.post("https://recommendmeabook.com/home/next_book")
                .header("X-CSRF-Token", self.tokens.csrf.clone())
                .header("Cookie", self.tokens.cookie.clone())
                .form(&body)
                .send()
                .unwrap();
            match response.status() {
                reqwest::StatusCode::OK => {
                    result = response.text().unwrap();
                    break;
                }
                reqwest::StatusCode::NOT_FOUND => {
                    println!("{:?}", response.status());
                    println!("No book with id = {}", self.current_booknumber);
                }
                _ => panic!("Unexpected status code = {}", response.status())
            }
            self.current_booknumber += 1;
        }
        result
    }

    fn get_tokens() -> Tokens {
        let mut response = reqwest::get("https://recommendmeabook.com/").unwrap();
        let value: &str = response.headers().get(reqwest::header::SET_COOKIE)
            .and_then(|s| s.to_str().ok()).unwrap();
        let cookie = value[..value.find(';').unwrap()].to_string();

        let document = Html::parse_document(&response.text().unwrap());
        let selector = Selector::parse("meta[name=csrf-token]").unwrap();
        let mut iterator = document.select(&selector);
        let node = iterator.next().unwrap();
        assert_eq!(iterator.next(), None);
        Tokens {
            csrf: node.value().attr("content").unwrap().to_string(),
            cookie,
        }
    }
}

impl Drop for NextBook {
    fn drop(&mut self) {
        NextBook::store_booknumber(self.current_booknumber);
    }
}

fn main() {
//    let mut nb = NextBook::new();
//    println!("{:?}", nb.page());
    fn sanitize(s: &str) -> String {
        s.replace("\\", " ").replace('\n', " ").trim().to_string()
    }
    let string = std::fs::read_to_string("page.txt").unwrap();
    let parts = string.lines();
    println!("{:?}", parts.next());
    let x: HashMap<_, _> = parts.filter_map(|s| if s.starts_with("var") {
        let index = s.find('=').unwrap();
        Some((sanitize(&s[4..index]), sanitize(&s[index+2..])))
    } else { None }).into_iter().collect();

    println!("{:?}", x);
//    println!("{:?}", get_tokens())
//    copy_to_clipboard("dickbutt");
//    wait_and_exit(Duration::from_secs(10));
}
