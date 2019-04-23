extern crate clap;
extern crate dirs;
extern crate rustyline;
extern crate shellfn;

use clap::{App, Arg};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use shellfn::shell;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[shell]
fn build_file(dir: &str) -> Result<String, Box<Error>> {
    r"
    cd $DIR
    cargo run -q -- --cap-lints allow
"
}

fn main() -> Result<(), Box<Error>> {
    let matches = App::new("RsEPL")
        .version("0.2.0")
        .author("vonforum <vonforum@windowslive.com>")
        .about("Rust REPL")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .value_name("DIRECTORY")
                .help("Run REPL in this directory")
                .takes_value(true),
        )
        .get_matches();

    let mut dir = (match matches.value_of("directory") {
        Some(dir) => PathBuf::from(dir),
        None => dirs::data_dir()
            .expect(
                r"No default data directory on this platform.
Rerun with -d <directory>",
            )
            .join("rsepl"),
    })
    .join("src");

    fs::create_dir_all(dir.as_path())?;

    dir.pop();
    fs::write(
        &dir.join("Cargo.toml"),
        r#"[package]
name = "_"
version = "0.1.0"
edition = "2018"
"#,
    )?;

    let mut rl = Editor::<()>::new();
    if rl.load_history(&dir.join("history.txt")).is_err() {
        println!("No previous history.");
    }

    let mut buffer = Vec::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                if line.starts_with(':') {
                    match line.as_ref() {
                        ":buffer" => println!("{}", buffer.join("\n")),

                        ":clear" => {
                            buffer.clear();
                            println!("Cleared buffer");
                        }

                        ":exit" => break,

                        ":h" | ":help" => println!(
                            r"Available commands:
:h | :help      - print this help message
:exit | CTRL-D  - exit
:buffer         - show the buffer
:clear          - clear the buffer, starting over
:pop            - remove the last successful line from the buffer"
                        ),

                        ":pop" => match buffer.pop() {
                            Some(pl) => {
                                println!("Removed line: {}", pl);
                            }
                            None => {
                                println!("Buffer empty");
                            }
                        },

                        _ => println!("Unknown command: {}", line),
                    }
                } else {
                    buffer.push(line);

                    fs::write(
                        &dir.join("src").join("main.rs"),
                        format!(
                            r#"
#![allow(warnings)]
fn main() {{
    print!("{{:?}}", {{
        {}
    }});
}}
"#,
                            buffer.join(";\n")
                        ),
                    )?;

                    match build_file(dir.to_str().unwrap()) {
                        Ok(out) => {
                            println!("{}", out);
                        }
                        Err(_) => {
                            buffer.pop();
                        }
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(&dir.join("history.txt")).unwrap();
    Ok(())
}
