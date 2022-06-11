# ego
Modern updated echo command in Rust  
Supports printing a user input in ASCII art format  

## Table of Contents
- [prerequisites](#prerequisites)
- [run](#run)
- [usage](#usage)

## Prerequisites
Rust version -> ```rustc 1.59.0```  
Cargo version -> ```cargo 1.59.0```  
Crates -> ```clap = 2```

## Run
To run the program, download the repository first:
```
git clone https://github.com/Lab-Brat/ego.git
```
Change into the repo, and run using cargo:
```
cd ego
cargo run
```

## Usage
```-h``` or ```--help``` - display help message  
```-n``` or ```--drop-newline``` - do not add newline (\n) in the end of output  
```-a``` or ```--ascii``` - display user input in ascii format
```-o``` - print empty user input without shrug emoji
```-e``` - interpret escape symbols
