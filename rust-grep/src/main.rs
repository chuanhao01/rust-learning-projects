use std::{env, process};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    pub fn build_from_env_args(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        // Skips program path
        args.next();

        // This is bad code
        let query = match args.next() {
            Some(query) => query,
            None => {
                return Err(String::from(
                    "Not enough arguments provided, expected 2, got 0",
                ))
            }
        };
        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => {
                return Err(String::from(
                    "Not enough arguments provided, expected 2, got 1",
                ))
            }
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
mod minigrep {
    use std::fs;

    use crate::Config;

    pub fn run(config: Config) -> Result<Vec<String>, String> {
        let file_contents = fs::read_to_string(config.file_path)
            .map_err(|err| format!("Problem reading file: {}", err))?;
        Ok(search(config.query, file_contents, config.ignore_case))
    }
    fn search(query: String, contents: String, ignore_case: bool) -> Vec<String> {
        let mut query = query;
        if ignore_case {
            query = query.to_lowercase();
        }
        contents
            .lines()
            .filter(|line| {
                if ignore_case {
                    line.to_lowercase().contains(&query)
                } else {
                    line.contains(&query)
                }
            })
            .map(|line| line.to_string())
            .collect()
    }
}

fn main() {
    let config = Config::build_from_env_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let result = minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Problem running minigrep: {}", err);
        process::exit(1);
    });
    println!("{}", result.join("\n"));
}
