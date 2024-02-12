use std::{
    env, fs,
    io::{self, BufRead},
    process::exit,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let compiler = Lox::new();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        compiler.run_file(&args[1]);
        exit(64);
    } else {
        compiler.run_prompt();
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&self, input: &String) {
        let output = fs::read(input).unwrap();
        let as_string = String::from_utf8(output).unwrap();
        println!("{}", as_string);
    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            print!("> ");
            let current_line = line.unwrap();
            self.run(&current_line);
        }
    }

    fn run(&self, str: &String) {
        let scanner = Scanner::new();
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:#?}", token);
        }
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, String::from(""), message)
    }

    fn report(&mut self, line: u32, wh: String, message: String) {
        eprintln!("[line {} ] Error {} : {}", line, wh, message);
        self.had_error = true;
    }
}

struct Scanner {}

impl Scanner {
    fn new() -> Scanner {
        Scanner {}
    }

    fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}

#[derive(Debug)]
struct Token {}
