use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
    let commands = parse_commands(stdin.lock());

    let mut registers = [0; 26];
    let mut snd_reg = 0;
    let mut rcv_reg = 0;

    let mut pc = 0;
    while pc < commands.len() {
        match commands[pc] {
            Set(reg, val) => {
                registers[reg] = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Add(reg, val) => {
                registers[reg] += match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Mul(reg, val) => {
                registers[reg] *= match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Mod(reg, val) => {
                registers[reg] %= match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Snd(val) => {
                snd_reg = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
            },
            Rcv(val) => {
                let val = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };

                if val != 0 {
                    rcv_reg = snd_reg;
                    break;
                }
            },
            Jgz(val, offset) => {
                let val = match val {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };
                let offset = match offset {
                    Register(reg) => registers[reg],
                    Number(num) => num,
                };

                if val > 0 {
                    pc = (pc as i64 + offset) as usize;
                    continue;
                }
            },
        }
        pc += 1;
    }

    println!("{}", rcv_reg);
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Snd(Value),
    Rcv(Value),
    Jgz(Value, Value),
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
            "add" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Add(reg, val)
            },
            "mul" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Mul(reg, val)
            },
            "mod" => {
                let reg = parse_reg(words[1]);
                let val = parse_val(words[2]);
                Mod(reg, val)
            },
            "snd" => {
                let val = parse_val(words[1]);
                Snd(val)
            },
            "rcv" => {
                let val = parse_val(words[1]);
                Rcv(val)
            },
            "jgz" => {
                let val1 = parse_val(words[1]);
                let val2 = parse_val(words[2]);
                Jgz(val1, val2)
            },
            _ => panic!(),
        });
    }

    commands
}
