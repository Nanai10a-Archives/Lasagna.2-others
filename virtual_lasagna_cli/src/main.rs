#![feature(try_blocks)]

mod commands;
mod constant;
mod schema;
mod serde;
mod test;
mod types;

fn main() {
    let stdin = std::io::stdin();

    let code = loop {
        eprint!("input: ");

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        buf = buf.replace("\n", "").replace("\\n", "\n");

        match process(buf) {
            None => continue,
            Some(code) => break code,
        }
    };

    std::process::exit(code);
}

/// in: "[any]"
fn process(mut s: String) -> types::ExitStatus {
    if s.is_empty() {
        return commands::nop();
    }

    if s.starts_with(constant::PREFIX) {
        for _ in 0..constant::PREFIX_LEN {
            s.remove(0);
        }

        return commands::command(s);
    }

    commands::post(s)
}
