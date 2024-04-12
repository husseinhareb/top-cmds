use std::fs;

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
pub fn get_parent_pid(pid: u32) -> Option<u32> {
    read_proc_file(pid, "PPid:").and_then(|ppid_str| ppid_str.parse().ok())
}

//Getting the name of the shell
pub fn get_shell(pid: u32) -> Option<String> {
    read_proc_file(pid, "Name:").map(|name| name.to_string())
}
