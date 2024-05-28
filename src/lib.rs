use std::{ error::Error, fs };
#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}
impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters");
        } else {
            let ignore_case = match args.get(3) {
                Some(s) => if s.contains("-i") || s.contains("1") { true } else { false }
                None => false,
            };
            let query = match args.get(1) {
                Some(s) => if s.contains("-q") {
                    let str: Vec<&str> = s.split("-q").collect();
                    str[0]
                } else {
                    s
                }
                _ => "",
            };
            let file_path = match args.get(2) {
                Some(s) => if s.contains("-p") {
                    let str: Vec<&str> = s.split("-p").collect();
                    str[0]
                } else {
                    s
                }
                _ => "",
            };
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        // &String可以自动转换为&str
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    //     #[test]
    //     fn one_result() {
    //         let query = "duct";
    //         let contents = "\
    // Rust:
    // safe, fast, productive.
    // Pick three.";
    //         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}
