use std::error::Error;
use std::ffi::OsStr;
use std::io::Write;
use std::{env, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut exists = false;
    let mut var_name = None;

    let mut args = env::args();
    let bin_name = args.next().unwrap();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--exists" => exists = true,
            unknown if unknown.starts_with('-') => {
                eprintln!("unknown flag: {}", unknown);
                eprintln!("Usage: {} [--exists] [VAR_NAME]", bin_name);
                std::process::exit(1);
            }
            var if var_name.is_none() => {
                var_name = Some(var.to_string());
            }
            _ => {
                eprintln!("Too many arguments provided");
                eprintln!("Usage: {} [--exists] [VAR_NAME]", bin_name);
                std::process::exit(1);
            }
        }
    }

    let ev = match var_name {
        Some(vn) => env::var(&vn),
        None => env::var("PATH"),
    };
    let ev = match ev {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("Error reading from var: {}", err).into());
        }
    };

    let uniques = env::split_paths(&ev).fold(Vec::new(), |mut acc, path| {
        if !acc.contains(&path) {
            if !exists || path.exists() {
                acc.push(path);
            }
        }
        acc
    });

    match env::join_paths(uniques) {
        Ok(joined) => {
            print_os_string(&joined);
            Ok(())
        }
        Err(err) => Err(format!("Failed to join paths: {err}").into()),
    }
}

#[cfg(unix)]
fn print_os_string(path: &OsStr) {
    use std::os::unix::ffi::OsStrExt;
    let _ = io::stdout().write_all(path.as_bytes());
    let _ = io::stdout().flush();
}

#[cfg(not(unix))]
fn print_os_string(path: &OsStr) {
    print!("{}", path.to_string_lossy());
}
