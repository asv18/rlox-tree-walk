use clap::Parser;
use rlox_tree_walk::{Args, run_file};

fn main() {
    let args = Args::parse();

    run_file(&args.file_name);
}
