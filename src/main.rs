use std::env;
use std::fs;

const HELP: &str = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.\

Mandatory arguments to long options are mandatory for short options too.
  -p, --parents     no errors if existing, make parent directories as needed
";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut targets: Vec<String> = Default::default();
    let mut parents = false;

    if args.len() == 1 {
        println!("{}", HELP);
        return Ok(());
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            "-p" => parents = true,
            "--parents" => parents = true,
            _ => {
                if args[i].to_string().starts_with("-") {
                    println!("{}", HELP);
                    return Ok(());
                }
                targets.push(args[i].to_string());
            }
        }
    }

    for i in targets {
        if parents {
            fs::create_dir_all(i.to_string())?;
            continue;
        }
        fs::create_dir(i.to_string())?;
    }

    Ok(())
}
