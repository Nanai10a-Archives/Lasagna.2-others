use io::Read;
use std::{env, fs, io};

use serde::{Deserialize, Serialize};
use toml::{de, ser};

fn main() {
    let mut args = env::args();
    args.next();

    if args.len() != 2 {
        println!("accept only 2 arg. ( args: [user] [path] )");
        println!("exiting...");
        return;
    }

    let user = args.next().unwrap();
    let path = args.next().unwrap();

    println!("user: {}", user);

    let f_opt = fs::OpenOptions::new().read(true).open(path);

    let mut f = match f_opt {
        Ok(f) => f,
        Err(e) => {
            println!("error occurred (file open): {}", e);
            println!("exiting...");
            return;
        }
    };

    let mut buf = String::new();
    let bytes = f.read_to_string(&mut buf).unwrap();

    println!("read {} bytes.", bytes);

    let OldSchema { mut posts } = match de::from_str(buf.as_str()) {
        Ok(s) => s,
        Err(e) => {
            println!("error occurred (deserialize from old_schema): {}", e);
            println!("exiting...");
            return;
        }
    };

    let incorrect_fmt = posts
        .iter()
        .map(|v| v.created.as_str())
        .filter(|v| chrono::DateTime::parse_from_str(v, "%F %T").is_ok())
        .count();

    if incorrect_fmt != 0 {
        println!(
            "detected {} incorrect format on old_schema#created.",
            incorrect_fmt
        );
        println!("exiting...");
        return;
    }

    let converted_posts = posts
        .drain(..)
        .enumerate()
        .map(|(i, OldPost { content, created })| NewPost {
            num: i as u32,
            content,
            created,
            updated: None,
            is_deleted: None,
        })
        .collect::<Vec<_>>();

    let converted_schema = NewSchema {
        user,
        max_num: (converted_posts.len() - 1) as u32,
        posts: converted_posts,
    };

    let converted = match ser::to_string(&converted_schema) {
        Ok(s) => s,
        Err(e) => {
            println!("error occurred (serialize to new_schema): {}", e);
            println!("exiting...");
            return;
        }
    };

    println!("converted, printing...");
    println!("{}", converted);
    println!();
    println!("successfully convert!");
    println!("exiting...");
}

pub type Date = chrono::prelude::DateTime<chrono::Local>;

#[derive(Serialize)]
struct NewSchema {
    user: String,
    max_num: u32,
    posts: Vec<NewPost>,
}

#[derive(Serialize)]
struct NewPost {
    num: u32,
    content: String,
    created: String,
    updated: Option<String>,
    is_deleted: Option<bool>,
}

#[derive(Deserialize)]
struct OldSchema {
    posts: Vec<OldPost>,
}

#[derive(Deserialize)]
struct OldPost {
    content: String,
    created: String,
}
