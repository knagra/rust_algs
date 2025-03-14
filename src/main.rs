use std::env;

mod print_time;
mod menu;
mod kadane;
mod common_chars;
mod lpts_hash_table;
mod lprh_hash_table;
mod qpts_hash_table;


fn main() {
    let all_algs = ["menu", "kadane", "common_chars", "lpts_hash_table", "lprh_hash_table", "qpts_hash_table"];
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
    } else if alg_name == "lpts_hash_table" {
        lpts_hash_table::run();
    } else if alg_name == "lprh_hash_table" {
        lprh_hash_table::run();
    } else if alg_name == "qpts_hash_table" {
        qpts_hash_table::run();
    } else {
        panic!("Could not find \"{}\" as a run target", alg_name);
    }
}
