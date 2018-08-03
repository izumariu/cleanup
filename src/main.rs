extern crate clap;

use clap::{Arg, App};
use std::fs;

fn main() {

    let cfg = App::new("cleanup")
                .about("removes empty files and directories recursively")
                .version("0.1.0")
                .arg(Arg::with_name("level")
                    .short("l")
                    .long("level")
                    .value_name("DEPTH")
                    .help("Max search depth")
                    .takes_value(true)
                ).get_matches();

    let depth = match cfg.value_of("level") {
        Some(v) => DepthLevel::from(&v),
        None => DepthLevel::Infinity
    };

    let mut prg_lang = Language::en_US;

    if let Ok(lang) = std::env::var("LANG") {

        prg_lang = match &lang[0..5] {
            "de_DE" => Language::de_DE,
            _ => Language::en_US,
        };

    }

    run(String::from("."), prg_lang, depth, 1);

}

#[allow(non_camel_case_types)]
enum Language {
    de_DE,
    en_US,
}

enum Message {
    DirEntry(String),
    DirLeave(String),
    FileRemove(String),
    DirRemove(String),
}

enum DepthLevel {
    Bound(u32),
    Infinity,
}

impl DepthLevel {

    fn allows_increment(&self, current: &u32) -> bool {

        match &self {

            DepthLevel::Bound(bound) => {
                if *current >= *bound {
                    false
                } else {
                    true
                }
            },

            DepthLevel::Infinity => true,

        }

    }

    fn from(inval: &str) -> DepthLevel {

        let level: u32 = inval.parse().unwrap();
        DepthLevel::Bound(level)

    }

}

fn prgmsg(lang: &Language, msg: &Message) {

    match msg {

        Message::DirEntry(dir) => {
            match lang {
                Language::de_DE => println!("cleanup: Verzeichnis '{}' wird betreten", dir),
                _ => println!("cleanup: Entering directory '{}'", dir)
            }
        },

        Message::DirLeave(dir) => {
            match lang {
                Language::de_DE => println!("cleanup: Verzeichnis '{}' wird verlassen", dir),
                _ => println!("cleanup: Leaving directory '{}'", dir)
            }
        },

        Message::FileRemove(file) => {
            match lang {
                Language::de_DE => println!("cleanup: Datei '{}' wird gelöscht", file),
                _ => println!("cleanup: Deleting file '{}'", file)
            }
        },

        Message::DirRemove(dir) => {
            match lang {
                Language::de_DE => println!("cleanup: Verzeichnis '{}' wird gelöscht", dir),
                _ => println!("cleanup: Deleting directory '{}'", dir)
            }
        },

    }

}

fn run(path: String, lang: Language, depth: DepthLevel, depth_current: u32) {

    for entry in fs::read_dir(&path).unwrap() {

        let entry_path = entry.unwrap().path();

        if entry_path.is_dir() {

            if fs::read_dir(&entry_path).unwrap().count() > 0 {

                if depth.allows_increment(&depth_current) {

                    let new_path = String::from(entry_path.to_str().unwrap());

                    prgmsg(&lang, &Message::DirEntry(String::from(entry_path.to_str().unwrap())));
                    deeper(&new_path, &lang, &depth, &(depth_current + 1));
                    prgmsg(&lang, &Message::DirLeave(String::from(entry_path.to_str().unwrap())));

                }

            } else {

                prgmsg(&lang, &Message::DirRemove(String::from(entry_path.to_str().unwrap())));
                fs::remove_dir(entry_path).unwrap();

            }

        } else if entry_path.is_file() {


            if entry_path.metadata().unwrap().len() == 0 {
                prgmsg(&lang, &Message::FileRemove(String::from(entry_path.to_str().unwrap())));
                fs::remove_file(entry_path).unwrap();
            }

        }

    }

}

// TODO join methods run() and deeper()

fn deeper(path: &str, lang: &Language, depth: &DepthLevel, depth_current: &u32) {

    for entry in fs::read_dir(path).unwrap() {

        let entry_path = entry.unwrap().path();

        if entry_path.is_dir() {

            if fs::read_dir(&entry_path).unwrap().count() > 0 {

                if depth.allows_increment(&depth_current) {

                    let new_path = String::from(entry_path.to_str().unwrap());

                    prgmsg(lang, &Message::DirEntry(String::from(entry_path.to_str().unwrap())));
                    deeper(&new_path, lang, depth, &(depth_current + 1));
                    prgmsg(lang, &Message::DirLeave(String::from(entry_path.to_str().unwrap())));

                }

            } else {

                prgmsg(&lang, &Message::DirRemove(String::from(entry_path.to_str().unwrap())));
                fs::remove_dir(entry_path).unwrap();

            }

        } else if entry_path.is_file() {

            if entry_path.metadata().unwrap().len() == 0 {
                prgmsg(lang, &Message::FileRemove(String::from(entry_path.to_str().unwrap())));
                fs::remove_file(entry_path).unwrap();
            }

        }

    }

}
