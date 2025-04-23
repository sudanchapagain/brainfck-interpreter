use brainfck::Interpreter;
use std::io::{self, Cursor};

fn run_bf(program: &str, input: &str) -> io::Result<String> {
    let input_cursor = Cursor::new(input.as_bytes());
    let mut output = Vec::new();

    let mut interpreter = Interpreter::new(program.bytes().collect(), input_cursor, &mut output);
    interpreter.run()?;

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[test]
fn test_hello_world() {
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    assert_eq!(run_bf(code, "").unwrap(), "Hello World!\n");
}
