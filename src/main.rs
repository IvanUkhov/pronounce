extern crate play;

use std::{env, process};

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
    let url = match locate(&arguments[1], 1, English::American, Format::MP3) {
        Ok(url) => url,
        Err(error) => abort(error),
    };
    println!("{}", url);
}

fn abort(message: &'static str) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}

fn locate(word: &str, variant: usize, english: English, format: Format)
          -> Result<String, &'static str> {

    let mut word = word.trim().to_lowercase();
    if word.is_empty() {
        return Err("expected a nonempty word")
    }
    for c in word.chars() {
        match c {
            'a'...'z' => {},
            _ => return Err("expected the word to contain only letters"),
        }
    }
    let (slag1, slag2) = match english {
        English::American => ("us", "us"),
        English::British => ("uk", "gb"),
    };
    let extension = match format {
        Format::OGG => "ogg",
        Format::MP3 => "mp3",
    };
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
    Ok(url)
}
