use clap::Parser;
use color_eyre::Result;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long, default_value("."))]
    root: PathBuf,
}

impl Opts {
    fn run(&self) -> Result<()> {
        println!("{self:#?}");
        Ok(())
    }
}

fn main() {
    let opts = Opts::parse();
    if let Err(problem) = opts.run() {
        eprintln!("{problem:?}");
        std::process::exit(1);
    }
}
