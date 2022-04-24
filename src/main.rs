use std::collections::VecDeque;
use std::io::{self};

// Enumerator for the different tokens in Brainfuck
#[derive(PartialEq, Eq)]
enum Token {
    Inc,
    Dec,
    PtrR,
    PtrL,
    In,
    Out,
    SLoop,
    ELoop,
    Ignore,
}

#[derive(PartialEq, Eq)]
enum Command {
    Inc,
    Dec,
    PtrR,
    PtrL,
    In,
    Out,
    Loop(Vec<Command>),
}

// Class representing the program and its tape

struct Program {
    tape: [u8; 4096],
    pointer: usize,
    output: String,
    source: String,
}
impl Program {
    fn new(src: &str) -> Program {
        Program {
            tape: [0u8; 4096],
            pointer: 0,
            output: String::new(),
            source: src.to_owned(),
        }
    }

    fn tokenize(&mut self) -> Vec<Command> {
        let mut p = Vec::new();
        let parsed = self
            .source
            .chars()
            .map(|tok| match tok {
                '>' => Token::PtrR,
                '<' => Token::PtrL,
                '+' => Token::Inc,
                '-' => Token::Dec,
                '.' => Token::Out,
                ',' => Token::In,
                '[' => Token::SLoop,
                ']' => Token::ELoop,
                _ => Token::Ignore,
            })
            .filter(|op| op.ne(&Token::Ignore));

        let mut tokenized_program: VecDeque<Vec<Command>> = VecDeque::new();
        tokenized_program.push_back(Vec::new());
        for i in parsed {
            let section = tokenized_program.back_mut();
            match i {
                Token::PtrR => {}
                Token::PtrL => {}
                Token::Inc => {}
                Token::Dec => {}
                Token::Out => {}
                Token::In => {}
                Token::SLoop => {}
                Token::ELoop => {}
            }
        }

        return p;
    }

    fn execute(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Inc => self.tape[self.pointer] += 1,
                Command::Dec => self.tape[self.pointer] -= 1,
                Command::PtrR => self.pointer += 1,
                Command::PtrL => self.pointer -= 1,
                Command::In => {
                    let mut input = String::new();
                    print!("The program requires your input: ");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Could not read from stdin");
                    self.tape[self.pointer] = input.chars().next().unwrap() as u8;
                }
                Command::Out => {
                    self.output.push(self.tape[self.pointer] as char);
                    println!("{}", self.output);
                }
                Command::Loop(commands) => {
                    while self.tape[self.pointer] != 0 {
                        self.execute(commands);
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        let cmds = self.tokenize();
        self.execute(&cmds);
    }
}

fn main() {
    let mut source = String::new();
    // loop {
    //     let mut inp = String::new();
    //     io::stdin()
    //         .read_line(&mut inp)
    //         .expect("Error reading from stdin");
    //     inp = inp.trim().to_string();
    //     if inp == "" {
    //         break;
    //     }
    //     source.push_str(&inp)
    // }
    let mut program = Program::new(&source);
    program.run();
}
