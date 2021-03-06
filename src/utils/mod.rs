use rand::Rng;
use std::env::Args;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub rows: usize,
    pub cols: usize,
    pub algorithm: String,
}

pub struct Options {}

impl Options {
    pub fn parse(mut args: Args) -> Config {
        let mut rows = None;
        let mut cols = None;
        let mut algorithm = String::from("dfs"); // default is dfs

        // Skip the first arg (path of program)
        args.next();

        for arg in args {
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
            algorithm: algorithm,
        }
    }
}
