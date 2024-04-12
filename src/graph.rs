use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs;
use crate::shell:get_shell;
use crate::history::fetch_file;

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";


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
        " ╔══════════════════════════════════════════════════╗",
        &format!(" ║{}║", " ".repeat(50)),    
        &format!(" ║  {}{}1.{} ({} times) {} {}║",BLUE,BOLD, first_name, first_count,RESET, " ".repeat(46 - first_name.len() - 11 - first_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(first_per as usize), "░".repeat((46 - first_per as usize) as usize)),   
        &format!(" ║{}║", " ".repeat(50)),                                                      
        &format!(" ║  {}{}2.{} ({} times) {} {}║",BLUE,BOLD, second_name, second_count,RESET, " ".repeat(46 - second_name.len() - 11 - second_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(second_per as usize), "░".repeat((46 - second_per as usize) as usize)),     
        &format!(" ║{}║", " ".repeat(50)),                                                      
        &format!(" ║  {}{}3.{} ({} times) {} {}║",BLUE,BOLD, third_name, third_count,RESET, " ".repeat(46 - third_name.len() - 11 - third_count.to_string().len())), &format!(" ║  {}{}  ║", "█".repeat(third_per as usize), "░".repeat((46 - third_per as usize) as usize)),       
        &format!(" ║{}║", " ".repeat(50)),                                                      
        " ╚══════════════════════════════════════════════════╝"
    ];
    

    for line in art.iter() {
        println!("{}", line);
    }
}



fn output() {


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

    let top = top_commands(&history);

    if top.len() >= 3 {
        let (first_command, first_count) = (&top[0].0, top[0].1);
        let (second_command, second_count) = (&top[1].0, top[1].1);
        let (third_command, third_count) = (&top[2].0, top[2].1);
        
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