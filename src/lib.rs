use clap::{App, Arg};
use std::process;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    text: Vec<String>,
    omit_newline: bool,
    read_slash: bool,
    ascii_artify: bool,
    color_output:bool,
    no_shrug: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("ego")
    .version("0.1.0")
    .author("Lab-Brat")
    .about("Rust implementation of echo command")
    .arg(
        Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .min_values(1),
    )
    .arg(
        Arg::with_name("omit_newline")
            .short("n")
            .long("drop-newline")
            .help("Skip the newline in the end")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("read_slash")
            .short("e")
            .help("Interpret backslash escapes")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("ascii-artify")
            .short("a")
            .long("ascii")
            .help("Print words in ASCII art")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("color-output")
            .short("c")
            .long("color")
            .help("color echo output")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("no-shrug")
            .short("o")
            .help("print nothing instead of shurg")
            .takes_value(false),
    )
    .get_matches();
    
    Ok(Config {
        text: matches.values_of_lossy("text").unwrap_or(vec![]),
        omit_newline: matches.is_present("omit_newline"),
        read_slash: matches.is_present("read_slash"),
        ascii_artify: matches.is_present("ascii-artify"),
        color_output: matches.is_present("color_output"),
        no_shrug: matches.is_present("no-shrug"),
    })
}

// ------------------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut empty_output = String::new();
    let shrug: &str = "¯\\_(ツ)_/¯";

    let omit_newline = config.omit_newline ;
    let rs = config.read_slash ;
    let ascii = config.ascii_artify ;
    let color_out = config.color_output ;

    if config.text.join(" ") == "" {
        if config.no_shrug == false { empty_output.push_str(shrug) }
        println!("{}", empty_output);
    }
    
    Ok(())

    // if ascii {
    //     ascii_artify(text)
    // } else if rs { 
    //     slash_parser(text)
    // } else {
    //     print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    // }
}
    
// ------------------------------------------------------------
pub fn ascii_artify(text: Vec<String>) {
    let mut aa: [String; 11] = Default::default();

    for letter in text.join(" ").chars() {
        if letter.is_alphabetic() { 
            let letvec = read_a_file(letter)
                .unwrap_or_else( |error| {
                    println!("ERROR: {}", error);
                    println!("One of the symbols doesn't exist \
                                in the ASCII art DB");
                    process::exit(1);
                }); 
            for (i, lv) in letvec.iter().enumerate() {
                aa[i].push_str(lv)
            }
        }
    }
    for line in &aa { println!("{}", line) }
}

// ------------------------------------------------------------
pub fn slash_parser(text: Vec<String>) {
    let mut text = text.join(" ");
    
    let mut count = 0;
    loop {
        count += 1;
        if text.contains("\\n") {
            text = text.replace("\\n", "\n");
        } else if text.contains("\\t") {
            text = text.replace("\\t", "\t");
        } else if text.contains("\\r") {
            text = text.replace("\\r", "\r");
        } else { () }
        if count == text.len() { break; }
    }
    
    println!("{}", text);
}

// ------------------------------------------------------------
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_a_file(letter: char) -> Result<Vec<String>, 
                            Box<dyn std::error::Error>> {
    let mut filename = String::from("letters/");
    filename.push(letter);
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    Ok(lines)
}
