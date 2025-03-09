use std::{collections::HashMap, fs, io, ops::Add, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long, default_value_t = 2)]
    n: usize,

    #[arg(short, long)]
    symbols_only: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let walk = ignore::WalkBuilder::new(&args.path).build();

    let mut ngrams: HashMap<String, u64> = HashMap::new();
    for f in walk
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().is_ok_and(|m| !m.is_dir()))
    {
        let content = fs::read_to_string(f.path())?;
        for seq in content.split_whitespace() {
            for ngram in seq.chars().collect::<Vec<_>>().windows(args.n) {
                if args.symbols_only && ngram.iter().any(|c| c.is_ascii_alphabetic()) {
                    continue;
                }

                let str = String::from_iter(ngram);
                *ngrams.entry(str).or_default() += 1;
            }
        }
    }

    let mut csv = csv::Writer::from_writer(io::stdout());
    let mut entries: Vec<(String, u64)> = ngrams.into_iter().collect();
    entries.sort_by(|a, b| a.1.cmp(&b.1));

    for (ngram, count) in entries.into_iter().rev() {
        csv.write_record(&[ngram, count.to_string()])?;
    }

    Ok(())
}
