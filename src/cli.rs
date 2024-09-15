use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Optional path
    pub path: Option<String>,
    /// Do not ignore hidden elements
    #[arg(short, long)]
    pub all: bool,
}
