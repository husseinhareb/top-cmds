use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::fs;


fn get_parent_pid(pid: u32) -> Option<u32> {
    if let Ok(status) = fs::read_to_string(format!("/proc/{}/status", pid)) {
        for line in status.lines() {
            if line.starts_with("PPid:") {
                if let Some(ppid_str) = line.split_whitespace().nth(1) {
                    if let Ok(ppid) = ppid_str.parse::<u32>() {
                        return Some(ppid);
                    }
                }
            }
        }
    }
    None
}

fn get_shell(pid: u32) -> Option<String> {
    if let Ok(cmdline) = fs::read_to_string(format!("/proc/{}/cmdline", pid)) {
        let parts: Vec<&str> = cmdline.split('\0').collect();
        if let Some(shell_path) = parts.get(0) {
            if let Some(shell_name) = Path::new(shell_path).file_name() {
                return Some(shell_name.to_string_lossy().to_string());
            }
        }
    }
    None
}

fn fetch_file(shell: &str) -> String {
    let file_path: &str;

    if shell.contains("shell") {
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
    
    // ANSI escape codes for colors
    let red = "\x1b[31m";
    let blue = "\x1b[34m";
    let purple = "\x1b[35m";
    let reset = "\x1b[0m";
    // ANSI escape codes for styles
    let bold = "\x1b[1m";

    let art = [
        " ╔══════════════════════════════════════════════════════╗",
        &format!(" ║{}║", " ".repeat(54)),    
        &format!(" ║  {}{}1.{} ({} times) {} {}║",red,bold, first_name, first_count,reset, " ".repeat(50 - first_name.len() - 11 - first_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(first_per as usize), "░".repeat((50 - first_per as usize) as usize)),   
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {}{}2.{} ({} times) {} {}║",blue,bold, second_name, second_count,reset, " ".repeat(50 - second_name.len() - 11 - second_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(second_per as usize), "░".repeat((50 - second_per as usize) as usize)),     
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {}{}3.{} ({} times) {} {}║",purple,bold, third_name, third_count,reset, " ".repeat(50 - third_name.len() - 11 - third_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(third_per as usize), "░".repeat((50 - third_per as usize) as usize)),       
        &format!(" ║{}║", " ".repeat(54)),                                                      
        " ╚══════════════════════════════════════════════════════╝"
    ];
    

    for line in art.iter() {
        println!("{}", line);
    }
}




fn main() {
    // ANSI escape codes for colors
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    let blue = "\x1b[34m";
    // ANSI escape codes for styles
    let bold = "\x1b[1m";

    let mut shell = String::new(); // Initialize shell variable

    let mut current_pid = std::process::id();
    loop {
        if let Some(parent_pid) = get_parent_pid(current_pid) {
            if parent_pid == 1 {
                println!("Failed to determine the current shell.");
                break;
            }
            if let Some(shell_name) = get_shell(parent_pid) {
                println!("Current shell: {}", shell_name);
                shell = shell_name; // Assign value to shell
                break;
            }
            current_pid = parent_pid;
        } else {
            println!("Failed to determine the current shell.");
            break;
        }
    }

    let file_path = fetch_file(&shell);
    println!("•Default Shell: {}{}{}{}", green, bold, shell, reset);
    let history = fetch_history(&file_path, &shell);
    println!("•History length: {}{}{}{}", green, bold, history.len(), reset);

    if shell.contains("fish") {
        println!("Note: The Fish shell does not save every command invocation individually, but rather records the last time a command was executed. As a result, the occurrence count of a command may not exceed a few instances.");
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
        println!("{}Insufficient data to generate art.{}{}", blue, bold, reset);
    }
}