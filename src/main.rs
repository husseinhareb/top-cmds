use std::env;

mod config;
mod history;
mod graph;
mod shell;

fn help() {
    println!("Usage: top-cmds [options]");
    println!("Options:");   
    println!("-h               Display this help message"); 
    println!("-s <number>      Set the number of most used commands");         
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = config::create_config();
        let _ = graph::graph();
        return;
    }

    let mut iter = args.iter().skip(1); // Skip the first argument (program name)

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" => {
                help();
                return;
            }
            "-s" => {
                if let Some(nb_cmds) = iter.next() {
                    match nb_cmds.parse::<i32>() {
                        Ok(nb) => {
                            let _ = config::write_nb_cmds(nb);
                        }
                        Err(_) => {
                            eprintln!("Invalid value provided for -s flag: {}", nb_cmds);
                            help();
                            return;
                        }
                    }
                } else {
                    eprintln!("Unit value not provided for the -s flag.");
                    help();
                    return;
                }
            }
            _ => {
                eprintln!("Invalid argument: {}", arg);
                help();
                return;
            }
        }
    }
}
