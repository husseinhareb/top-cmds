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
    let file_path: &str;

    match shell {
        FISH_SHELL => {
            println!("Using Fish shell");
            file_path = "HOME/.local/share/fish/fish_history"; // Assign file path
        }
        ZSH_SHELL => {
            println!("Using Zsh shell");
            file_path = "HOME/.zsh_history";
        }
        BASH_SHELL => {
            println!("Using Bash shell");
            file_path = "HOME/.bash_history";
        }
        _ => {
            println!("Unknown shell");
            file_path = ""; 
        }
    }

    file_path.to_string() 
}

fn main() {
    let shell = fetch_shell();
    let file_path = fetch_file(&shell);
    println!("File path: {}", file_path);
}
