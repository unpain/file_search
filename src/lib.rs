use std::{ error::Error, fs };
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters");
        } else {
            Ok(Config {
                query: &args[1],
                file_path: &args[2],
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
