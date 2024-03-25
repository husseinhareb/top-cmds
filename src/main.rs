use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FISH_SHELL: [&str; 2] = ["/usr/bin/fish", "/bin/fish"];
const ZSH_SHELL: [&str; 2] = ["/usr/bin/zsh", "/bin/zsh"];
const BASH_SHELL: [&str; 2] = ["/usr/bin/bash", "/bin/bash"];

fn fetch_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"grep "^$(whoami):" /etc/passwd | cut -d: -f7"#)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let shell = String::from_utf8_lossy(&output.stdout);
        shell.trim().to_string()
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing command:\n{}", error);
        String::new()
    }
}

fn fetch_file(shell: &str) -> String {
    let file_path: &str;

    if FISH_SHELL.contains(&shell) {
        println!("Using Fish shell");
        file_path = ".local/share/fish/fish_history";
    } else if ZSH_SHELL.contains(&shell) {
        println!("Using Zsh shell");
        file_path = ".zsh_history";
    } else if BASH_SHELL.contains(&shell) {
        println!("Using Bash shell");
        file_path = ".bash_history";
    } else {
        println!("Unknown shell");
        file_path = "";
    }

    file_path.to_string()
}

fn fetch_history(file_path: &str, shell: &str) -> Vec<String> {
    let mut history = Vec::new();

    if let Ok(file) = File::open(Path::new(&std::env::var("HOME").unwrap()).join(file_path)) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(command) = line {
                if FISH_SHELL.contains(&shell) && command.starts_with("- cmd:") {
                    let cleaned_command = command.chars().skip(6).collect::<String>();
                    history.push(cleaned_command);
                } else {
                    history.push(command);
                }
            }
        }
    } else {
        eprintln!("Failed to open history file");
    }

    history
}

fn main() {
    let shell = fetch_shell();
    let file_path = fetch_file(&shell);
    println!("Default Shell: {}", shell);
    let history = fetch_history(&file_path,&shell);
    println!("History length: {}", history.len());
    for command in &history {
        println!("{}", command);
    }
}
