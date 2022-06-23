pub mod flags {
    use std::process;

    pub fn ascii_artify(text: Vec<String>) {
        let mut aa: [String; 11] = Default::default();

        for letter in text.join(" ").chars() {
            if letter.is_alphabetic() { 
                let letvec = utilities::read_a_file(letter)
                    .unwrap_or_else( |error| {
                        println!("ERROR: {}", error);
                        println!("One of the symbols doesn't exist in the ASCII art DB");
                        process::exit(1);
                    }); 
                for (i, lv) in letvec.iter().enumerate() {
                    aa[i].push_str(lv)
                }
            }
        }
        for line in &aa { println!("{}", line) }
    }

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

    pub mod utilities {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        
        pub fn read_a_file(letter: char) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    }
}
