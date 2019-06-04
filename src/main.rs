use std::env;
use std::path::Path;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print help information");
    opts.optflag("v", "verbose", "Print output to stderr");
    opts.optflag("e", "exists", "Don't add any paths that aren't resolvable");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program_name, opts);
        return;
    }

    let original = get_path_env();
    let num_original = original.len();

    if matches.opt_present("v") {
        eprintln!("Original number of path entries: {}", num_original);
    }

    let mut uniques: Vec<String> = Vec::new();
    original.iter().for_each(|p| {
        if matches.opt_present("e") && !Path::new(&p).exists() {
            if matches.opt_present("v") {
                eprintln!("Not adding non-existing path: {}", p);
            }
            return;
        }
        if !uniques.contains(p) {
            uniques.push(p.to_string());
        } else if matches.opt_present("v") {
            eprintln!("Ignoring duplicate entry: {}", p);
        }
    });

    if matches.opt_present("v") {
        eprintln!("Removed {} duplicate entries, leaving {} unique entries.", num_original - uniques.len(), uniques.len());
    }

    println!("{}", env::join_paths(uniques).unwrap().into_string().unwrap());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("Usage: {} [options]", opts.usage(&brief));
}

fn get_path_env() -> Vec<String> {
    env::split_paths(&env::var("PATH").unwrap())
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect()
}