use std::env;
use std::fs;
use std::os::unix::fs as unix;

const HELP: &str = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.

Mandatory arguments to long options are mandatory for short options too.
  -m, --mode [MODE] set file mode (as in chmod), not a=rwx - umask
  -p, --parents     no error if existing, make parent directories as needed
  -v, --verbose     print a message for each created directory
  -h, --help        display this help and exit
      --version     output version information and exit
";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", HELP);
        return Ok(());
    }

    let mut targets: Vec<String> = Default::default();
    let mut parents = false;
    let mut mode_checking = false;
    let mut mode: u32 = 0;
    let mut verbose = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-p" => parents = true,
            "--parents" => parents = true,
            "-m" => mode_checking = true,
            "--mode" => mode_checking = true,
            "-v" => verbose = true,
            "--verbose" => verbose = true,
            "--help" => return printhelp(),
            "-h" => return printhelp(),
            "--version" => {
                println!("mkdir (rust implementation) {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {
                if args[i].to_string().starts_with("-") {
                    println!("{}", HELP);
                    return Ok(());
                }
                if mode_checking {
                    match u32::from_str_radix(&args[i], 8) {
                        Ok(x) => {
                            if x > 511 {
                                eprintln!("mkdir: invalid mode '{}'", args[i]);
                                return Ok(());
                            }
                            mode = x;
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                    mode_checking = false;
                } else {
                    targets.push(args[i].to_string());
                }
            }
        }
    }

    if targets.len() == 0 {
        println!("{}", HELP);
        return Ok(());
    }

    for i in targets {
        if parents {
            fs::create_dir_all(i.to_string())?;
        } else {
            fs::create_dir(i.to_string())?;
        }

        if mode != 0 {
            let perms = unix::PermissionsExt::from_mode(mode);
            fs::set_permissions(i.to_string(), perms)?;
        }
        if verbose {
            println!("mkdir: created directory '{}'", i);
        }
    }

    Ok(())
}

fn printhelp() -> std::io::Result<()> {
    println!("{}", HELP);
    Ok(())
}
