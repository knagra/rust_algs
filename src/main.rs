use std::env;

mod print_time;
mod menu;
mod kadane;
mod common_chars;


fn main() {
    let all_algs = ["menu", "kadane", "common_chars"];
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "Usage: alg [algorithm_name].\n\talgorith_name can be one of: {:?}",
            &all_algs,
        );
    }
    let alg_name = &args[1];
    if alg_name == "menu" {
        menu::run();
    } else if alg_name == "kadane" {
        kadane::run();
    } else if alg_name == "common_chars" {
        common_chars::run();
    }
}
