use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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
        file_path = ".local/share/fish/fish_history";
    } else if ZSH_SHELL.contains(&shell) {
        file_path = ".zsh_history";
    } else if BASH_SHELL.contains(&shell) {
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
                match shell {
                    s if FISH_SHELL.contains(&s) => {
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

fn top_commands(history: &[String]) -> Vec<(&String, usize)> {
    let mut counts = HashMap::new();

    for command in history {
        *counts.entry(command).or_insert(0) += 1;
    }

    let mut sorted_counts: Vec<(&String, usize)> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_counts.into_iter().take(3).collect()
}

fn create_responsive_art(first: i32, first_name: &str, first_count: usize, second: i32, second_name: &str, second_count: usize, third: i32, third_name: &str, third_count: usize, total: i32) {
    let first_per = (first as f32 / total as f32) * 50.0;
    let second_per = (second as f32 / total as f32) * 50.0;
    let third_per = (third as f32 / total as f32) * 50.0;

    let art = [
        " ╔══════════════════════════════════════════════════════╗",
        &format!(" ║{}║", " ".repeat(54)),    
        &format!(" ║  {} ({} times) {}║", first_name, first_count, " ".repeat(50 - first_name.len() - 8 - first_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(first_per as usize), "░".repeat((50 - first_per as usize) as usize)),   
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {} ({} times) {}║", second_name, second_count, " ".repeat(50 - second_name.len() - 8 - second_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(second_per as usize), "░".repeat((50 - second_per as usize) as usize)),     
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {} ({} times) {}║", third_name, third_count, " ".repeat(50 - third_name.len() - 8 - third_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(third_per as usize), "░".repeat((50 - third_per as usize) as usize)),       
        &format!(" ║{}║", " ".repeat(54)),                                                      
        " ╚══════════════════════════════════════════════════════╝"
    ];
    

    for line in art.iter() {
        println!("{}", line);
    }
}




fn main() {
    let shell = fetch_shell();
    let file_path = fetch_file(&shell);
    println!("Default Shell: {}", shell);
    let history = fetch_history(&file_path, &shell);
    println!("History length: {}", history.len());

    if shell.contains("fish"){
        println!("Disclaimer: Fish shell doesn't save each time the command was run but it saves when was the last time ran so the occurrence of a command will never pass 5 or so")
    }

    let top_3 = top_commands(&history);

    if top_3.len() >= 3 {
        let (first_command, first_count) = (&top_3[0].0, top_3[0].1);
        let (second_command, second_count) = (&top_3[1].0, top_3[1].1);
        let (third_command, third_count) = (&top_3[2].0, top_3[2].1);
        
        create_responsive_art(
            first_count as i32, first_command, first_count, 
            second_count as i32, second_command, second_count, 
            third_count as i32, third_command, third_count, 
            history.len() as i32
        );
    } else {
        println!("Insufficient data to generate art.");
    }
}
