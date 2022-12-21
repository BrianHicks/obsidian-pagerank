use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long, default_value("."))]
    root: PathBuf,
}

fn main() {
    let opts = Opts::parse();
    println!("{:#?}", opts);
}
