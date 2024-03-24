use std::process::Command;

const FISH_SHELL: &str = "/usr/bin/fish";
const ZSH_SHELL: &str = "/usr/bin/zsh";
const BASH_SHELL: &str = "/usr/bin/bash";

fn fetch_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"grep "^$(whoami):" /etc/passwd | cut -d: -f7"#)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let shell = String::from_utf8_lossy(&output.stdout);
        return shell.trim().to_string();
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing command:\n{}", error);
        return String::new();
    }
}

fn fetch_file(shell: &str) -> String {
    match shell {
        FISH_SHELL => {
            println!("Using Fish shell");
            
        }
        ZSH_SHELL => {
            println!("Using Zsh shell");
        }
        BASH_SHELL => {
            println!("Using Bash shell");
        }
        _ => {
            println!("Unknown shell");
        }
    }
}

fn main() {
    let shell = fetch_shell();
    fetch_file(&shell);
}
