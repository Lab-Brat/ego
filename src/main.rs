use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use clap::{App, Arg};

fn main() {
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
            Arg::with_name("ascii-artify")
                .short("a")
                .long("ascii")
                .help("Print words in ASCII art")
                .takes_value(false),
        )
    .get_matches();
    
    let mut shrug = Vec::new();
    shrug.push(String::from("¯\\_(ツ)_/¯"));
    let text = matches.values_of_lossy("text").unwrap_or(shrug);
    let omit_newline = matches.is_present("omit_newline");

    let ascii = matches.is_present("ascii-artify");
    if ascii {
        ascii_artify(text)
    } else {
        print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    }
}

fn ascii_artify(text: Vec<String>) {
    let mut aa: [String; 11] = Default::default();

    for letter in text.join(" ").chars() {
        if letter.is_alphabetic() { 
            let letvec = read_a_file(letter);
            for (i, lv) in letvec.iter().enumerate() {
                aa[i].push_str(lv)
            }
        }
    }
    for line in &aa { println!("{}", line) }
}

fn read_a_file(letter: char) -> Vec<String> {
    let mut filename = String::from("letters/");
    filename.push(letter);

    let file = File::open(filename).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    return lines
}
