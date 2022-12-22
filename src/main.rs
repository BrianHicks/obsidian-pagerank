use clap::Parser;
use color_eyre::Result;
use eyre::WrapErr;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref LINK_RE: Regex = Regex::new(r"\[\[(?P<name>[^#]+?)(#\^[\w\d\-])?(\|.+?)?\]\]").unwrap();
}

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long, default_value("."))]
    root: PathBuf,

    /// A value between 0 and 100
    #[clap(long, default_value("80"))]
    damping_factor: u8,
}

impl Opts {
    fn run(&self) -> Result<()> {
        let files = self.discover_files().wrap_err("could not discover files")?;

        let links = self
            .discover_links(&files)
            .context("could not discover links")?;

        println!("{links:?}");

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

    fn discover_links(&self, files: &Vec<PathBuf>) -> Result<HashMap<String, HashSet<String>>> {
        let mut out = HashMap::with_capacity(files.len());

        let links: Vec<Result<(String, HashSet<String>)>> = files
            .par_iter()
            .map(|path| {
                let name = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| {
                        eyre::eyre!(
                            "no stem for `{}`, or filename wasn't unicode",
                            path.display()
                        )
                    })?
                    .to_string();

                let contents = std::fs::read_to_string(path)
                    .wrap_err_with(|| format!("could not read {}", path.display()))?;

                let mut links = HashSet::with_capacity(1);
                for capture in LINK_RE.captures_iter(&contents) {
                    if let Some(name) = capture.name("name") {
                        links.insert(name.as_str().to_string());
                    }
                }

                Ok((name, links))
            })
            .collect();

        for link_res in links {
            match link_res {
                Ok((name, links)) => {
                    out.insert(name, links);
                }
                Err(problem) => return Err(problem).wrap_err("could not calculate link"),
            }
        }

        return Ok(out);
    }
}

fn main() {
    let opts = Opts::parse();
    if let Err(problem) = opts.run() {
        eprintln!("{problem:?}");
        std::process::exit(1);
    }
}
