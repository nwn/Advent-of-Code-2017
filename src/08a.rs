use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut registers: HashMap<String, i32> = HashMap::new();
    lines.for_each(|line| {
        let line = line.unwrap();
        let mut words = line.split_whitespace();

        // Read fields
        let reg = String::from(words.next().unwrap());
        let op = words.next().unwrap();
        let amnt: i32 = words.next().unwrap().parse().expect("Failed to read integer");
        words.next(); // Skip "if"
        let cond_l: i32 = {
            let operand = String::from(words.next().unwrap());
            match operand.parse() {
                Ok(int) => int,
                Err(_) => *registers.entry(operand).or_insert(0),
            }
        };
        let cond_op = words.next().unwrap();
        let cond_r: i32 = {
            let operand = String::from(words.next().unwrap());
            match operand.parse() {
                Ok(int) => int,
                Err(_) => *registers.entry(operand).or_insert(0),
            }
        };

        // Check condition
        let reg = registers.entry(reg).or_insert(0);
        let sat = match cond_op {
            "==" if cond_l == cond_r => true,
            "!=" if cond_l != cond_r => true,
            "<"  if cond_l <  cond_r => true,
            "<=" if cond_l <= cond_r => true,
            ">"  if cond_l >  cond_r => true,
            ">=" if cond_l >= cond_r => true,
            _ => false,
        };
        if sat {
            // Execute command
            match op {
                "inc" => *reg += amnt,
                "dec" => *reg -= amnt,
                _ => (),
            }
        }
    });

    println!("{}", registers.values().max().unwrap());
}
