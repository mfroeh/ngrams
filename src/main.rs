use std::{collections::HashMap, fs, io, path::PathBuf, process::exit};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// the paths to recursively search for files
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// the n in n-grams
    #[arg(short, long, default_value_t = 2)]
    n: usize,

    #[arg(short, long, default_value = "all")]
    mode: Mode,

    /// whether whitespace only ngrams should be excluded
    #[arg(short, long, default_value_t = true)]
    exclude_whitespace_only: bool,
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    Alpha,
    Numeric,
    Alnum,
    Symbols,
    All,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for path in args.paths.iter() {
        if !path.exists() {
            eprintln!("{}: no such file or directory", path.to_str().unwrap());
            exit(1);
        }
    }

    let mut walk = ignore::WalkBuilder::new(&args.paths[0]);
    for path in args.paths.iter().skip(1) {
        walk.add(path);
    }
    let walk = walk.build();

    let mut ngrams: HashMap<String, u64> = HashMap::new();
    for f in walk
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().is_ok_and(|m| !m.is_dir()))
    {
        let content = fs::read_to_string(f.path());
        if let Err(e) = content {
            eprintln!("{}: {}", f.path().to_str().unwrap(), e);
            continue;
        }
        let content = content.unwrap();

        for ngram in content.chars().collect::<Vec<_>>().windows(args.n) {
            match args.mode {
                Mode::Alpha => {
                    if ngram.iter().any(|c| !c.is_alphabetic()) {
                        continue;
                    }
                }
                Mode::Numeric => {
                    if ngram.iter().any(|c| !c.is_numeric()) {
                        continue;
                    }
                }
                Mode::Alnum => {
                    if ngram.iter().any(|c| !c.is_alphanumeric()) {
                        continue;
                    }
                }
                Mode::Symbols => {
                    if ngram.iter().any(|c| c.is_alphanumeric()) {
                        continue;
                    }
                }
                Mode::All => {}
            }

            if args.exclude_whitespace_only {
                if ngram.iter().all(|c| c.is_whitespace()) {
                    continue;
                }
            }

            let str = String::from_iter(ngram);
            *ngrams.entry(str).or_default() += 1;
        }
    }

    let mut entries: Vec<(String, u64)> = ngrams.into_iter().collect();
    entries.sort_by(|a, b| a.1.cmp(&b.1));

    for (ngram, count) in entries.into_iter().rev() {
        let ngram = ngram
            .replace(" ", "␣")
            .replace("\r\n", "\n")
            .replace("\n", "↵")
            .replace("\t", "\\t");
        println!("{} {}", ngram, count)
    }

    Ok(())
}
