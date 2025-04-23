use std::io::{self, Read, Write};

pub struct Interpreter<R: Read, W: Write> {
    cells: Vec<u8>,
    program: Vec<u8>,
    loop_stack: Vec<usize>,
    prog_ptr: usize,
    data_ptr: usize,
    input: R,
    output: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    const SIZE: usize = 30_000;

    pub fn new(program: Vec<u8>, input: R, output: W) -> Self {
        let filtered = program
            .into_iter()
            .filter(|c| b"><+-.,[]".contains(c))
            .collect();

        Self {
            cells: vec![0; Self::SIZE],
            program: filtered,
            loop_stack: Vec::new(),
            prog_ptr: 0,
            data_ptr: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.prog_ptr < self.program.len() {
            match self.program[self.prog_ptr] as char {
                '>' => self.data_ptr += 1,
                '<' => {
                    if self.data_ptr == 0 {
                        return Err(io::Error::other("data pointer moved left out of bounds"));
                    }
                    self.data_ptr -= 1;
                }
                '+' => self.cells[self.data_ptr] = self.cells[self.data_ptr].wrapping_add(1),
                '-' => self.cells[self.data_ptr] = self.cells[self.data_ptr].wrapping_sub(1),
                '.' => {
                    self.output.write_all(&[self.cells[self.data_ptr]])?;
                    self.output.flush()?;
                }
                ',' => {
                    let mut buffer = [0];
                    if self.input.read_exact(&mut buffer).is_ok() {
                        self.cells[self.data_ptr] = buffer[0];
                    }
                }
                '[' => {
                    if self.cells[self.data_ptr] == 0 {
                        let mut balance = 1;
                        while balance > 0 {
                            self.prog_ptr += 1;
                            if self.prog_ptr >= self.program.len() {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Unmatched '['",
                                ));
                            }
                            match self.program[self.prog_ptr] as char {
                                '[' => balance += 1,
                                ']' => balance -= 1,
                                _ => {}
                            }
                        }
                    } else {
                        self.loop_stack.push(self.prog_ptr);
                    }
                }
                ']' => {
                    if self.cells[self.data_ptr] != 0 {
                        if let Some(pos) = self.loop_stack.last() {
                            self.prog_ptr = *pos;
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "unmatched ']'",
                            ));
                        }
                    } else {
                        self.loop_stack.pop();
                    }
                }
                _ => {}
            }
            self.prog_ptr += 1;
        }
        Ok(())
    }
}
