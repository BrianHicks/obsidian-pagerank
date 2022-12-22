use clap::Parser;
use color_eyre::Result;
use eyre::WrapErr;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long, default_value("."))]
    root: PathBuf,
}

impl Opts {
    fn run(&self) -> Result<()> {
        let files = self.discover_files();
        println!("{files:#?}");

        Ok(())
    }

    fn discover_files(&self) -> Result<Vec<PathBuf>> {
        let mut types_builder = ignore::types::TypesBuilder::new();
        types_builder.add_defaults();
        types_builder.select("markdown");

        let mut out = Vec::new();
        for entry_res in ignore::WalkBuilder::new(&self.root)
            .types(
                types_builder
                    .build()
                    .wrap_err("could not build matching types")?,
            )
            .build()
        {
            match entry_res {
                Ok(entry) => match entry.file_type() {
                    Some(ft) if ft.is_file() => out.push(entry.into_path()),
                    _ => continue,
                },
                Err(problem) => return Err(problem).wrap_err("could not read files"),
            }
        }

        Ok(out)
    }
}

fn main() {
    let opts = Opts::parse();
    if let Err(problem) = opts.run() {
        eprintln!("{problem:?}");
        std::process::exit(1);
    }
}
