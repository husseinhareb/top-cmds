use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs;

//Fetching the history file path of the shell
fn fetch_file(shell: &str) -> String {
    let file_path: &str;

    if shell.contains("fish") {
        file_path = ".local/share/fish/fish_history";
    } else if shell.contains("zsh") {
        file_path = ".zsh_history";
    } else if shell.contains("bash") {
        file_path = ".bash_history";
    } else {
        println!("Unknown shell");
        file_path = "";
    }

    file_path.to_string()
}


//Fetching the history of commands for the shell history file
fn fetch_history(file_path: &str, shell: &str) -> Vec<String> {
    let mut history = Vec::new();

    if let Ok(file) = File::open(Path::new(&std::env::var("HOME").unwrap()).join(file_path)) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(command) = line {
                match shell {
                    s if "fish".contains(&s) => {
                        if command.starts_with("- cmd:") {
                            let cleaned_command = command.chars().skip(7).collect::<String>();
                            history.push(cleaned_command);
                        }
                    }
                    _ => {
                        history.push(command);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open history file");
    }

    history
}