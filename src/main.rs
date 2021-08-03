const HELP_NO_ARGS: &str = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.\
";

fn main() -> std::io::Result<()> {
    println!("{}", HELP_NO_ARGS);
    Ok(())
}
