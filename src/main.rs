use std::env;

mod analysis;
mod constants;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!(
            "\n\n
            No argument!\n
            Use: {} directory\n
            For example: {} ~/
            \n\n",
            constants::CLI_COMMAND_NAME,
            constants::CLI_COMMAND_NAME,
        );
    } else {
        let _ = analysis::analysis(args[1].trim());
    }
}
