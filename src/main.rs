use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs;

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

//Reading the status of a process from the status file of it's process folder
fn read_proc_file(pid: u32, keyword: &str) -> Option<String> {
    if let Ok(status) = fs::read_to_string(format!("/proc/{}/status", pid)) {
        for line in status.lines() {
            if line.starts_with(keyword) {
                if let Some(value) = line.split_whitespace().nth(1) {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}
//Getting parent PID of the current command (The shell the command ran in)
fn get_parent_pid(pid: u32) -> Option<u32> {
    read_proc_file(pid, "PPid:").and_then(|ppid_str| ppid_str.parse().ok())
}

//Getting the name of the shell
fn get_shell(pid: u32) -> Option<String> {
    read_proc_file(pid, "Name:").map(|name| name.to_string())
}

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

//Calculating the most used commands
fn top_commands(history: &[String]) -> Vec<(&String, usize)> {
    let mut counts = HashMap::new();

    for command in history {
        *counts.entry(command).or_insert(0) += 1;
    }

    let mut sorted_counts: Vec<(&String, usize)> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_counts.into_iter().take(3).collect()
}

//Graph Creataion
fn ascii_graph(first: i32, first_name: &str, first_count: usize, second: i32, second_name: &str, second_count: usize, third: i32, third_name: &str, third_count: usize, total: i32) {
    let first_per = (first as f32 / total as f32) * 50.0;
    let second_per = (second as f32 / total as f32) * 50.0;
    let third_per = (third as f32 / total as f32) * 50.0;
    

    let art = [
        " ╔══════════════════════════════════════════════════════╗",
        &format!(" ║{}║", " ".repeat(54)),    
        &format!(" ║  {}{}1.{} ({} times) {} {}║",BLUE,BOLD, first_name, first_count,RESET, " ".repeat(50 - first_name.len() - 11 - first_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(first_per as usize), "░".repeat((50 - first_per as usize) as usize)),   
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {}{}2.{} ({} times) {} {}║",BLUE,BOLD, second_name, second_count,RESET, " ".repeat(50 - second_name.len() - 11 - second_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(second_per as usize), "░".repeat((50 - second_per as usize) as usize)),     
        &format!(" ║{}║", " ".repeat(54)),                                                      
        &format!(" ║  {}{}3.{} ({} times) {} {}║",BLUE,BOLD, third_name, third_count,RESET, " ".repeat(50 - third_name.len() - 11 - third_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(third_per as usize), "░".repeat((50 - third_per as usize) as usize)),       
        &format!(" ║{}║", " ".repeat(54)),                                                      
        " ╚══════════════════════════════════════════════════════╝"
    ];
    

    for line in art.iter() {
        println!("{}", line);
    }
}




fn main() {


    let mut shell = String::new(); 

    let mut current_pid = std::process::id();
    loop {
        if let Some(parent_pid) = get_parent_pid(current_pid) {
            if parent_pid == 1 {
                println!("Failed to determine the current shell.");
                break;
            }
            if let Some(shell_name) = get_shell(parent_pid) {
                shell = shell_name;
                break;
            }
            current_pid = parent_pid;
        } else {
            println!("Failed to determine the current shell.");
            break;
        }
    }

    let file_path = fetch_file(&shell);
    println!("•Current Shell: {}{}{}{}", GREEN, BOLD, shell, RESET);
    let history = fetch_history(&file_path, &shell);
    println!("•History length: {}{}{}{}", GREEN, BOLD, history.len(), RESET);

    if shell.contains("fish") {
        println!("{}{}Note: The Fish shell does not save every command invocation\nindividually, but rather records the last time a command\nwas executed. As a result, the occurrence count of a\ncommand may not exceed a few instances.{} ",RED,BOLD,RESET);
    }

    let top_3 = top_commands(&history);

    if top_3.len() >= 3 {
        let (first_command, first_count) = (&top_3[0].0, top_3[0].1);
        let (second_command, second_count) = (&top_3[1].0, top_3[1].1);
        let (third_command, third_count) = (&top_3[2].0, top_3[2].1);
        
        ascii_graph(
            first_count as i32, first_command, first_count, 
            second_count as i32, second_command, second_count, 
            third_count as i32, third_command, third_count, 
            history.len() as i32
        );
    } else {
        println!("{}Insufficient data to generate art.{}{}", BLUE, BOLD, RESET);
    }
}