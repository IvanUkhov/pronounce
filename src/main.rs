extern crate hyper;
extern crate play;
extern crate temporary;

use hyper::client::Client;
use hyper::status::StatusCode;
use std::{env, mem, process};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use temporary::Directory;

const ROOT_URL: &'static str = "http://www.oxfordlearnersdictionaries.com";

#[derive(Clone, Copy)]
enum Accent {
    American,
    #[allow(dead_code)]
    British,
}

#[derive(Clone, Copy)]
enum Format {
    #[allow(dead_code)]
    OGG,
    MP3,
}

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => abort(error.description()),
    });
);

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() < 2 {
        abort("expected a word");
    }
    let word = arguments[1].trim().to_lowercase();
    if word.is_empty() {
        abort("expected a nonempty word");
    }
    for letter in word.chars() {
        match letter {
            'a'...'z' => {},
            _ => abort("expected the word to contain only letters"),
        }
    }
    let (filename, url) = locate(&word, 1, Accent::American, Format::MP3);
    let client = Client::new();
    let mut response = ok!(client.get(&url).send());
    if response.status != StatusCode::Ok {
        abort("failed to find the word");
    }
    let mut buffer = Vec::new();
    ok!(response.read_to_end(&mut buffer));
    let directory = ok!(Directory::new("pronounce"));
    let path = directory.join(&filename);
    let mut file = ok!(File::create(&path));
    ok!(file.write_all(&buffer));
    mem::drop(file);
    ok!(play::play(&path));
    if arguments.len() > 2 {
        ok!(fs::copy(&path, &filename));
    }
}

fn abort(message: &str) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}

fn locate(word: &str, variant: usize, accent: Accent, format: Format) -> (String, String) {
    let (slag1, slag2) = match accent {
        Accent::American => ("us", "us"),
        Accent::British => ("uk", "gb"),
    };
    let extension = match format {
        Format::OGG => "ogg",
        Format::MP3 => "mp3",
    };
    let mut filename = word.to_string();
    filename.push_str("__");
    filename.push_str(slag2);
    filename.push_str("_");
    filename.push_str(&variant.to_string());
    filename.push_str(".");
    filename.push_str(&extension);
    let mut url = ROOT_URL.to_string();
    url.push_str("/media/english/");
    url.push_str(slag1);
    url.push_str("_pron/");
    url.push_str(&filename[..1]);
    url.push_str("/");
    url.push_str(&filename[..3]);
    url.push_str("/");
    url.push_str(&filename[..5]);
    url.push_str("/");
    url.push_str(&filename);
    (filename, url)
}
