use brainfck::Interpreter;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: brainfck <file>");
        process::exit(1);
    }

    let filename = &args[1];
    let program = match read_file(filename) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error reading '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut interpreter = Interpreter::new(program, stdin.lock(), stdout.lock());

    if let Err(e) = interpreter.run() {
        eprintln!("runtime error: {}", e);
        process::exit(1);
    }
}

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
