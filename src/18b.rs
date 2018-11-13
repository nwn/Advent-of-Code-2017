use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
	let stdin = io::stdin();
    let commands = parse_commands(stdin.lock());

    let mut cpu0 = Cpu::new(0, &commands);
    let mut cpu1 = Cpu::new(1, &commands);
    let mut count = 0;
    loop {
        let (blocked0, msg_sent) = cpu0.advance();
        if let Some(msg) = msg_sent {
            cpu1.msg_queue.push_back(msg);
        }

        let (blocked1, msg_sent) = cpu1.advance();
        if let Some(msg) = msg_sent {
            cpu0.msg_queue.push_back(msg);
            count += 1;
        }

        if blocked0 && blocked1 {
            break;
        }
    }

    println!("{}", count);
}

type Word = i64;

#[derive(Debug, Copy, Clone)]
enum Command {
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Snd(Value),
    Rcv(usize),
    Jgz(Value, Value),
}
use Command::*;

#[derive(Debug, Copy, Clone)]
enum Value {
    Register(usize),
    Number(Word),
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
                let reg = parse_reg(words[1]);
                Rcv(reg)
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

struct Cpu<'a> {
    registers: [Word; 26],
    pc: usize,
    msg_queue: VecDeque<Word>,
    commands: &'a [Command],
}
impl<'a> Cpu<'a> {
    fn new(id: Word, commands: &'a [Command]) -> Self {
        let mut new = Self {
            registers: [0; 26],
            pc: 0,
            msg_queue: VecDeque::new(),
            commands,
        };
        new.registers[(b'p' - b'a') as usize] = id;
        new
    }

    fn advance(self: &mut Self) -> (bool, Option<Word>) {
        let mut blocked = false;
        let mut msg_out = None;
        let mut pc_step = 1;

        match self.commands[self.pc] {
            Set(dst, Register(src)) => self.registers[dst] =  self.registers[src],
            Add(dst, Register(src)) => self.registers[dst] += self.registers[src],
            Mul(dst, Register(src)) => self.registers[dst] *= self.registers[src],
            Mod(dst, Register(src)) => self.registers[dst] %= self.registers[src],
            Set(dst, Number(num))   => self.registers[dst] =  num,
            Add(dst, Number(num))   => self.registers[dst] += num,
            Mul(dst, Number(num))   => self.registers[dst] *= num,
            Mod(dst, Number(num))   => self.registers[dst] %= num,
            Snd(Register(src)) => msg_out = Some(self.registers[src]),
            Snd(Number(num))   => msg_out = Some(num),
            Rcv(dst) => {
                match self.msg_queue.pop_front() {
                    Some(msg) => self.registers[dst] = msg,
                    None => {
                        blocked = true;
                        pc_step = 0;
                    },
                }
            },
            Jgz(val, offset) => {
                let val = match val {
                    Register(reg) => self.registers[reg],
                    Number(num) => num,
                };
                let offset = match offset {
                    Register(reg) => self.registers[reg],
                    Number(num) => num,
                };

                if val > 0 {
                    pc_step = offset;
                }
            },
        }
        self.pc = (self.pc as Word + pc_step) as usize;

        (blocked, msg_out)
    }
}
