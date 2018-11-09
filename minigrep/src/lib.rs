use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env::*;
pub struct Config {
    query: String,
    filename:String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
//        if args.len() < 3{
//            return Err("not enough arguments");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();

        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(),Box<Error>>{
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}