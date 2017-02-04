extern crate hyper;
extern crate play;

use hyper::client::Client;
use hyper::status::StatusCode;
use std::{env, process};
use std::error::Error;

const ROOT_URL: &'static str = "http://www.oxfordlearnersdictionaries.com";

#[derive(Clone, Copy)]
enum English {
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

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != 2 {
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
    let url = locate(&word, 1, English::American, Format::MP3);
    let client = Client::new();
    let response = match client.get(&url).send() {
        Ok(response) => response,
        Err(error) => abort(error.description()),
    };
    if response.status != StatusCode::Ok {
        abort("failed to find the word");
    }
}

fn abort(message: &str) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}

fn locate(word: &str, variant: usize, english: English, format: Format) -> String {
    let (slag1, slag2) = match english {
        English::American => ("us", "us"),
        English::British => ("uk", "gb"),
    };
    let extension = match format {
        Format::OGG => "ogg",
        Format::MP3 => "mp3",
    };
    let mut word = word.to_string();
    word.push_str("__");
    word.push_str(slag2);
    let mut url = ROOT_URL.to_string();
    url.push_str("/media/english/");
    url.push_str(slag1);
    url.push_str("_pron/");
    url.push_str(&word[..1]);
    url.push_str("/");
    url.push_str(&word[..3]);
    url.push_str("/");
    url.push_str(&word[..5]);
    url.push_str("/");
    url.push_str(&word);
    url.push_str("_");
    url.push_str(&variant.to_string());
    url.push_str(".");
    url.push_str(&extension);
    url
}
