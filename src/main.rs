use clap::{App, Arg};
use ego::flags;

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
    
    let mut shrug = Vec::new();
    if matches.is_present("no-shrug") { shrug.push(String::from("")) } else { shrug.push(String::from("¯\\_(ツ)_/¯")) }
    
    let text = matches.values_of_lossy("text").unwrap_or(shrug);
    let omit_newline = matches.is_present("omit_newline");
    let ascii = matches.is_present("ascii-artify");
    let rs = matches.is_present("read_slash");

    if ascii {
        flags::ascii_artify(text)
    } else if rs { 
        flags::slash_parser(text)
    } else {
        print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    }
}
