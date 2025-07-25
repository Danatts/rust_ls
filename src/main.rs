use clap::Parser;
use rust_ls::cli::Cli;

fn main() {
    let Cli { path, all } = Cli::parse();
    let (files, folders) = rust_ls::get_entries(path, all);

    for folder in folders {
        println!("{folder}")
    }

    for file in files {
        println!("{file}")
    }
}
