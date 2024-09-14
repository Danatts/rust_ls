use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Do not ignore hidden elements
    #[arg(short, long)]
    pub all: bool,
}
