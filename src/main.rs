use std::env;
use std::path::Path;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pdedupe")]
struct Options {
    /// Environment variable to use.
    #[structopt(long, default_value = "PATH")]
    var: String,

    /// When true, print output informational output to stderr.
    #[structopt(short, long)]
    verbose: bool,

    /// When true, only allow paths that are resolvable to be included.
    #[structopt(short, long)]
    exists: bool,
}

fn main() {
    let opts = Options::from_args();

    let original = get_env_values(opts.var.clone());
    let num_original = original.len();

    if opts.verbose {
        eprintln!("Original number of path entries: {}", num_original);
    }

    let mut uniques: Vec<String> = Vec::new();

    original.iter().for_each(|p| {
        if opts.exists && !Path::new(&p).exists() {
            if opts.verbose {
                eprintln!("Not adding non-existing path: {}", p);
            }
            return;
        }
        if !uniques.contains(p) {
            uniques.push(p.to_string());
        } else if opts.verbose {
            eprintln!("Ignoring duplicate entry: {}", p);
        }
    });

    if opts.verbose {
        eprintln!("Removed {} duplicate entries, leaving {} unique entries.", num_original - uniques.len(), uniques.len());
    }

    println!("{}", env::join_paths(uniques).unwrap().into_string().unwrap());
}

fn get_env_values(var: String) -> Vec<String> {
    let value = match env::var(var) {
        Ok(v) => v,
        _ => "".to_string()
    };

    env::split_paths(value.as_str())
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect()
}
