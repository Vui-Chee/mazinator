use rand::Rng;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub rows: usize,
    pub cols: usize,
    pub algorithm: String,
}

pub struct Options {}

impl Options {
    pub fn parse(args: &Vec<String>) -> Config {
        let mut rows = None;
        let mut cols = None;
        let mut algorithm = "dfs"; // default is dfs

        let mut args_excl = args.iter();
        args_excl.next();

        for arg in args_excl {
            match arg.to_string().parse::<usize>() {
                Ok(num) => {
                    if num <= 1 {
                        eprintln!("rows, cols must be integer value >= 1.");
                        process::exit(1);
                    }

                    if rows == None {
                        rows = Some(num);
                    } else if cols == None {
                        cols = Some(num);
                    } else {
                        eprintln!("Too many args.");
                        process::exit(1);
                    }
                }
                Err(_err) => {
                    if arg == "dfs" || arg == "bfs" {
                        algorithm = arg;
                    } else {
                        eprintln!("Unknown option : {}", arg);
                        process::exit(1);
                    }
                }
            };
        }

        // Set random default for rows and cols.
        if rows == None {
            rows = Some(rand::thread_rng().gen_range(5, 10));
        }
        if cols == None {
            cols = Some(rand::thread_rng().gen_range(5, 10));
        }

        Config {
            rows: rows.unwrap(),
            cols: cols.unwrap(),
            algorithm: String::from(algorithm),
        }
    }
}
