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
    .get_matches();
    
    let mut shrug = Vec::new();
    shrug.push(String::from("¯\\_(ツ)_/¯"));
    let text = matches.values_of_lossy("text").unwrap_or(shrug);
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}