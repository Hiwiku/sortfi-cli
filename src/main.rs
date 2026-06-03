mod constants;
mod core;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let home_path: PathBuf = core::get_home_path();

    if args.len() <= 1 {
        constants::help_message();
    } else {
        let currect_path = PathBuf::from(args.get(1).unwrap());
        let result: Vec<core::File> = core::analysis(currect_path, home_path);
        let _ = core::run_actions(result);
    }
}
