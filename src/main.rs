use std::process;
use tmpltool::run;

fn main() {
    if let Err(e) = run(std::env::args_os()) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
