use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{self, prelude::*, BufRead, Write,BufReader};

pub fn create_config() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().expect("Unable to determine config directory");
    let folder_path = config_dir.join("top-cmds");

    if !folder_exists(&folder_path) {
        fs::create_dir(&folder_path)?;
    }

    let file_path = folder_path.join("top-cmds.conf");

    if !file_exists(&file_path) {
        fs::write(&file_path, "")?;
    }

    Ok(())
}

fn folder_exists(folder_path: &PathBuf) -> bool {
    if let Ok(metadata) = std::fs::metadata(folder_path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn file_exists(file_path: &PathBuf) -> bool {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}



// Function to write city name according to parameter into the config file
pub fn write_nb_cmds(nb_cmds: i32) -> io::Result<()> {
    let file_path = config_file()?;
    let mut file_content = String::new();

    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut file_content)?;
    }

    let mut updated_content = String::new();
    let mut nb_cmds_found = false;

    for line in file_content.lines() {
        if line.trim().starts_with("nb_cmds") {
            nb_cmds_found = true;
            updated_content.push_str(&format!("nb_cmds {}\n", nb_cmds));
        } else {
            updated_content.push_str(&line);
            updated_content.push('\n');
        }
    }

    if !nb_cmds_found {
        updated_content.push_str(&format!("nb_cmds {}\n", nb_cmds));
    }

    let mut file = File::create(&file_path)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

// Function to read city name from config file
pub fn read_nb_cmds() -> io::Result<i32> {
    let file_path = config_file()?;
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("nb_cmds") {
            let nb_cmds_str = line.split_whitespace().skip(1).next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid format for nb_cmds")
            })?;
            let nb_cmds = nb_cmds_str.parse::<i32>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Failed to parse nb_cmds")
            })?;
            return Ok(nb_cmds);
        }
    }

    // If nb_cmds variable is not found, return 3
    Ok(3)
}



// Function to get the path of the config file
fn config_file() -> Result<PathBuf, io::Error> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found")),
    };

    let file_path = config_dir.join("top-cmds").join("top-cmds.conf");
    Ok(file_path)
}
