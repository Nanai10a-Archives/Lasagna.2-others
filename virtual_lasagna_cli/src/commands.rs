use crate::{constant, schema, serde, types};

pub fn nop() -> types::ExitStatus {
    println!("no input detected. no-operated.");
    None
}

/// in: "[command] [args...]"
pub fn command(s: String) -> types::ExitStatus {
    let splitted = split_command(&s);

    let mut args = match splitted {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };

    match args.remove(0) {
        "exit" => exit(args),
        "help" => help(args),
        "remove" => remove(args),
        "edit" => edit(args),
        "check" => check(args),
        "init" => init(args),
        "show" => show(args),
        _ => {
            println!(r#"unknown command. see ":help"."#);
            None
        }
    }
}

fn show_command_parse(args: types::Args) -> anyhow::Result<(usize, usize), String> {
    if args.len() != 2 {
        return Err(format!(
            "excepted 2 args, but supplied {} args.",
            args.len()
        ));
    }

    let once_show = match args[0].parse() {
        Ok(n) => {
            if n == 0 {
                return Err("parse error (once_show): cannot specify 0 or less".to_string());
            } else {
                n
            }
        }
        Err(e) => return Err(format!("parse error (once_show): {}", e)),
    };

    let page_num = match args[1].parse() {
        Ok(n) => {
            if n == 0 {
                return Err("parse error (page_num): cannot specify 0 or less".to_string());
            } else {
                n
            }
        }
        Err(e) => return Err(format!("parse error (page_num): {}", e)),
    };

    Ok((once_show, page_num))
}

fn show(args: types::Args) -> types::ExitStatus {
    let (once_show, page_num) = match show_command_parse(args) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            None?
        }
    };

    let mut s = serde::de_inner().unwrap();

    println!("user: {}", s.user);
    println!("max_num: {}", s.max_num);
    println!("once_show: {} | page_num: {}", once_show, page_num);

    let mut tmp_vec = s
        .posts
        .drain(..)
        .filter(|v| match v.is_deleted {
            None => true,
            Some(b) => !b,
        })
        .collect::<smallvec::SmallVec<[_; 1024]>>();

    tmp_vec.sort_by(|v1, v2| v1.num.cmp(&v2.num));

    let paging_offset = once_show * (page_num - 1);
    // (0 + paging_offset)..(once_show + paging_offset);
    let show_range = paging_offset..(once_show + paging_offset);
    println!("show: {}..{} | end_index: {}", show_range.start, show_range.end - 1, tmp_vec.len() - 1);

    if tmp_vec.len() <= show_range.start || tmp_vec.len() < show_range.end {
        println!("out of range: {:?} in {:?}", show_range, 0..tmp_vec.len());
        None?
    }

    println!();

    tmp_vec.drain(show_range).for_each(|v| {
        println!(
            "num: {} | created: {} | updated {:?}",
            v.num, v.created, v.updated
        );
        println!("content:");
        println!("{}", v.content);
        println!();
    });

    None
}

fn exit(_: types::Args) -> types::ExitStatus {
    println!("args are ignored, exiting...");
    Some(0)
}

fn help(_: types::Args) -> types::ExitStatus {
    println!(
        r#"args are ignored, showing HELP_TEXT...

{}
"#,
        *constant::HELP_TEXT
    );
    None
}

fn init(args: types::Args) -> types::ExitStatus {
    let user = match init_command_parse(args) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }
    .to_string();

    let init_data = schema::Schema {
        user,
        max_num: 0,
        posts: smallvec::smallvec![],
    };

    serde::ser(init_data);

    println!("successfully initialized file, will continue to check file integrity...");
    match serde::de_inner() {
        Ok(_) => {
            println!("checked file integrity!");
            None
        }
        Err(e) => {
            println!("failed checking file integrity, error: {}", e);
            Some(1)
        }
    }
}

fn init_command_parse(args: types::Args) -> anyhow::Result<&str, String> {
    if args.len() != 1 {
        return Err(format!(
            "excepted 1 args, but supplied {} args.",
            args.len()
        ));
    }

    Ok(args[0])
}

fn check(_: types::Args) -> types::ExitStatus {
    println!("args are ignored, checking...");

    match serde::de_inner() {
        Ok(_) => println!("no error detected."),
        Err(e) => println!("error: {}", e),
    }
    None
}

fn split_command(raw: &str) -> anyhow::Result<types::Args, &str> {
    let args = raw
        .split(' ')
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect::<smallvec::SmallVec<_>>();

    Ok(args)
}

pub fn post(s: String) -> types::ExitStatus {
    let mut data = serde::de();

    let post = schema::Post::new(s, data.max_num + 1);
    data.posts.push(post.clone());
    data.max_num += 1;

    serde::ser(data);

    println!("successfully post: {:?}", post);
    None
}

fn search_post(data: &schema::Schema, num: u32) -> anyhow::Result<usize, String> {
    let searched = data
        .posts
        .iter()
        .enumerate()
        .filter(|(_, p)| p.num == num)
        .map(|(i, _)| i)
        .collect::<smallvec::SmallVec<[_; 4]>>();

    if searched.len() != 1 {
        Err(format!("excepted 1 match, but {} matched.", searched.len()))
    } else {
        Ok(searched[0])
    }
}

/// in: [arg, arg, arg...]
fn remove(args: types::Args) -> types::ExitStatus {
    let num = match remove_command_parse(args) {
        Err(e) => {
            println!("{}", e);
            None?
        }
        Ok(n) => n,
    };

    let mut data = serde::de();

    let searched = search_post(&data, num);

    let index = match searched {
        Err(e) => {
            println!("{}", e);
            None?
        }
        Ok(i) => i,
    };

    let post = data.posts.get_mut(index).unwrap();
    let num = post.num;

    if post.is_deleted.is_some() && post.is_deleted.unwrap() {
        println!("already deleted {}th post.", num);
        None?
    }
    post.is_deleted = Some(true);
    post.updated = Some(chrono::Local::now());

    serde::ser(data);

    println!("successfully delete {}th post.", num);
    None
}

fn remove_command_parse(args: types::Args) -> anyhow::Result<u32, String> {
    if args.len() != 1 {
        return Err(format!(
            "excepted 1 args, but supplied {} args.",
            args.len()
        ));
    }

    match args[0].parse() {
        Err(e) => Err(format!("{}", e)),
        Ok(n) => Ok(n),
    }
}

/// in: [arg, arg, arg...]
fn edit(args: types::Args) -> types::ExitStatus {
    let (num, new_content) = match edit_command_parse(args) {
        Err(e) => {
            println!("{}", e);
            None?
        }
        Ok(t) => t,
    };

    let mut data = serde::de();

    let searched = search_post(&data, num);

    let index = match searched {
        Err(e) => {
            println!("{}", e);
            None?
        }
        Ok(i) => i,
    };

    let post = data.posts.get_mut(index).unwrap();

    post.content = new_content;
    post.updated = Some(chrono::Local::now());

    serde::ser(data);

    println!("successfully edit {}th post.", index);

    None
}

fn edit_command_parse(mut args: types::Args) -> anyhow::Result<(u32, String), String> {
    if args.len() <= 1 {
        return Err(format!(
            "excepted 2 and more args, bug supplied {} args.",
            args.len()
        ));
    }

    let num = match args.remove(0).parse() {
        Err(e) => return Err(format!("{}", e)),
        Ok(n) => n,
    };

    let mut content = String::new();
    args.drain(..).for_each(|v| content += v);

    Ok((num, content))
}
