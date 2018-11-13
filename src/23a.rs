use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
    let commands = parse_commands(stdin.lock());

    let mut registers = [0; 8];

    let mut count = 0;
    let mut pc = 0;
    while pc < commands.len() {
        match commands[pc] {
            Set(reg, val) => {
                registers[reg] = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Sub(reg, val) => {
                registers[reg] -= match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Mul(reg, val) => {
                registers[reg] *= match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
                count += 1;
            },
            Jnz(val, offset) => {
                let val = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
                let offset = match offset {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };

                if val != 0 {
                    pc = (pc as i64 + offset) as usize;
                    continue;
                }
            },
        }
        pc += 1;
    }

    println!("{}", count);
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}
use Command::*;

#[derive(Debug, Copy, Clone)]
enum Value {
    Register(usize),
    Number(i64),
}
use Value::*;

fn parse_commands(input: impl BufRead) -> Vec<Command> {
    let mut commands = vec![];
    for line in input.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_whitespace().collect();

        fn parse_val(input: &str) -> Value {
            match input.parse() {
                Ok(n) => Number(n),
                Err(_) => Register((input.bytes().next().unwrap() - b'a') as usize),
            }
        }
        fn parse_reg(input: &str) -> usize {
            if let Register(reg) = parse_val(input) {
                reg
            } else {
                panic!()
            }
        }

        commands.push(match words[0] {
            "set" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Set(reg, val)
            },
            "sub" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Sub(reg, val)
            },
            "mul" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Mul(reg, val)
            },
            "jnz" => {
                let val1 = parse_val(words[1]);
                let val2 = parse_val(words[2]);
                Jnz(val1, val2)
            },
            _ => panic!(),
        });
    }

    commands
}
