use std::collections::HashMap;
use crate::shell::get_shell;
use crate::history::fetch_file;
use crate::shell::get_parent_pid;
use crate::history::fetch_history;
use crate::config::read_nb_cmds;
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";


//Calculating the most used commands
fn top_commands(history: &[String], num_to_get: i32) -> Vec<(&String, usize)> {
    let mut counts = HashMap::new();

    for command in history {
        *counts.entry(command).or_insert(0) += 1;
    }

    let mut sorted_counts: Vec<(&String, usize)> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_counts.into_iter().take(num_to_get as usize).collect()
}

//Graph Creataion
fn ascii_graph(commands: Vec<(&str, usize)>, total_commands: usize) {
    let mut percentages: Vec<f32> = commands.iter().map(|(_, count)| *count as f32 / total_commands as f32).collect();
    let max_chars = 44;

    let max_percentage = percentages.iter().cloned().fold(0.0, f32::max);
    if max_percentage > 0.0 {
        let scaling_factor = max_chars as f32 / max_percentage;
        percentages.iter_mut().for_each(|p| *p *= scaling_factor);
    }

    let mut art = vec![
        " ╔════════════════════════════════════════════════╗".to_string(),
        format!(" ║{}║", " ".repeat(max_chars+4)),
    ];

    for (i, (command, count)) in commands.iter().enumerate() {
        let bar_length = percentages[i] as usize;
        let remainder = max_chars - bar_length;
        let num: usize = i+1;
        let num_str: &str = &num.to_string();
        let command_display = if command.len() > 30 {
            let truncated_command = &command[..30];
            format!("{}..", truncated_command)
        } else {
            command.to_string()
        };
        let mut str_len = command_display.len() + num_str.len() + 9 + count.to_string().len();
        if str_len >= 44
        {
            str_len = 44;
        }
        art.push(format!(
            " ║  {}{}{}.{} {} ({} times){}║ \n ║  {}{}  ║", BLUE, BOLD, num, RESET, command_display, count, " ".repeat(44 - str_len), "█".repeat(bar_length), "░".repeat(remainder)
        ));
    }
    

    art.push(format!(" ║{}║", " ".repeat(max_chars+4)));
    art.push(" ╚════════════════════════════════════════════════╝".to_string());

    for line in art {
        println!("{}", line);
    }
}






pub fn graph() {
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
    println!("• Current Shell: {}{}{}{}", GREEN, BOLD, shell, RESET);
    let history = fetch_history(&file_path, &shell);
    println!("• History length: {}{}{}{}", GREEN, BOLD, history.len(), RESET);

    if shell.contains("fish") {
        println!(
            "{}{}Note: The Fish shell does not save every command invocation\nindividually, but rather records the last time a command\nwas executed. As a result, the occurrence count of a\ncommand may not exceed a few instances.{} ",
            RED, BOLD, RESET
        );
    }

    let top: Vec<(&String, usize)>;

    match read_nb_cmds() {
        Ok(num_to_show) => {
            top = top_commands(&history, num_to_show);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            top = Vec::new();
        }
    }

    if !top.is_empty() {

        let total_len = history.len() as i32;
        let commands: Vec<(&str, usize)> = top.iter().map(|(cmd, count)| (cmd.as_str(), *count)).collect();
        ascii_graph(commands, total_len as usize);
    } else {
        println!("{}No top commands found.{}", RED, RESET);
    }
}
