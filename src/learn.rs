use std::{ env, error::Error, fs, process };

fn main() {
    let args: Vec<String> = env::args().collect();
    // // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    // // println!("Searching for {}", query);
    // // println!("In file {}", file_path)
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}")
    // let (query, file_path) = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path)
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}
impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
