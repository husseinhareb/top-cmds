use std::env;

mod shell;
mod history;
mod graph;

fn help() {
    println!("Usage: top-cmds [options] | top-cmds");
    println!("Options:");   
    println!("-h               Display this help message");     
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = config::create_config();
        
        return;
    }

    let mut iter = args.iter().skip(1); // Skip the first argument (program name)

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" => {
                help();
                return;
            }
            _ => {
                eprintln!("Invalid argument: {}", arg);
                help();
                return;
            }
        }
    }
}
