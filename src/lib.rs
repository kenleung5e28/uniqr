use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Leung <kenleung5e28@gmail.com")
        .about("Rust cat")
        .arg(
            Arg::with_name("in_file")
            .value_name("IN_FILE")
            .help("Input file")
            .default_value("-")
        )
        .arg(
            Arg::with_name("out_file")
            .value_name("OUT_FILE")
            .help("Output file")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("count")
            .short("c")
            .long("count")
            .help("Show counts")
            .takes_value(false)
        )
        .get_matches();
    
    Ok(Config {
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        out_file: matches.value_of_lossy("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut in_file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file = create(config.out_file.as_ref().map(|s| s.as_str()))?;
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count: u64 = 0;
    let mut print_line = |count: u64, s: &str| -> MyResult<()> {
        if count == 0 {
            return Ok(());
        }
        let output = if config.count {
            format!("{:>4} {}", count, s)
        } else {
            format!("{}", s)
        };
        out_file.write(output.as_bytes())?;
        Ok(())
    };
    loop {
        let bytes = in_file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if prev_line.trim_end() == line.trim_end() {
            count += 1;
        } else {
            print_line(count, &prev_line)?;
            prev_line = line.clone();
            count = 1;
        }
        line.clear();
    }
    print_line(count, &prev_line)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn create(filename: Option<&str>) -> MyResult<Box<dyn Write>> {
    match filename {
        Some(name) => Ok(Box::new(File::create(name)?)),
        _ => Ok(Box::new(io::stdout())),
    }
}