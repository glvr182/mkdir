use std::env;
use std::fs;

const HELP_NO_ARGS: &str = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.\
";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut targets: Vec<String> = Default::default();

    if args.len() == 1 {
        println!("{}", HELP_NO_ARGS);
        return Ok(());
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            _ => {
                if args[i].to_string().starts_with("-") {
                    println!("{}", HELP_NO_ARGS);
                    return Ok(());
                }
                targets.push(args[i].to_string());
            }
        }
    }

    for i in targets {
        fs::create_dir(i.to_string())?;
    }

    Ok(())
}
