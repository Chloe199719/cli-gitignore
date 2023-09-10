use std::env;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
enum Language {
    Typescript,
    Javascript,
    Rust,
}

fn main() {
    let mut language: Option<Language> = None;
    let mut custom = Vec::new();
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        match arg.to_lowercase().as_str() {
            "-h" | "--help" => {
                println!(
                    r#"--help, -h: Show this help message
--version, -v: Show the version
--language, -l: Specify a Programming Language (typescript, javascript, rust)
--remove, -r: Remove the current .gitignore file if it exists and generate a new one
--custom, -c: Add custom entries to the .gitignore file
ex -c .idea .vscode
                "#
                );
                return;
            }
            "-v" | "--version" => {
                println!("Version 0.1.0");
                return;
            }
            "-l" | "--language" => {
                match args[i + 1].to_lowercase().as_str() {
                    "typescript" => {
                        language = Some(Language::Typescript);
                    }
                    "javascript" => {
                        language = Some(Language::Javascript);
                    }
                    "rust" => {
                        language = Some(Language::Rust);
                    }
                    _ => {}
                }
            }
            "-r" | "--remove" => {
                match fs::remove_file(".gitignore") {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            "-c" | "--custom" => {
                for arg in args.iter().skip(i + 1) {
                    if arg.starts_with("-") {
                        break;
                    }
                    custom.push(format!("{}\n", arg));
                }
            }
            _ => {}
        }
    }
    if language.is_none() {
        while language.is_none() {
            println!("Please enter a language: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.to_lowercase().trim() {
                "typescript" => {
                    language = Some(Language::Typescript);
                }
                "javascript" => {
                    language = Some(Language::Javascript);
                }
                "rust" => {
                    language = Some(Language::Rust);
                }
                _ => {
                    println!("Invalid language");
                    continue;
                }
            }
        }
    }
    let mut file = match File::open(".gitignore") {
        Ok(_) => std::fs::OpenOptions::new().write(true).append(true).open(".gitignore").unwrap(),
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create(".gitignore") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    }
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        }
    };

    match language {
        Some(Language::Typescript) => {
            let git = r#"
/node_modules
/dist
/coverage
/build
.env
"#.trim_start();
            file.write_all(git.as_bytes()).expect("Unable to write data");
            for custom in custom.iter() {
                file.write_all(custom.as_bytes()).expect("Unable to write data");
            }
        }
        Some(Language::Javascript) => {
            let git = r#"
/node_modules
/dist
/coverage
/build
.env
"#.trim_start();
            file.write_all(git.as_bytes()).expect("Unable to write data");
            for custom in custom.iter() {
                file.write_all(custom.as_bytes()).expect("Unable to write data");
            }
        }
        Some(Language::Rust) => {
            let git = r#"
/target
"#.trim_start();
            file.write_all(git.as_bytes()).expect("Unable to write data");
            for custom in custom.iter() {
                file.write_all(custom.as_bytes()).expect("Unable to write data");
            }
        }
        None => { println!("There was an error") }
    }
}
