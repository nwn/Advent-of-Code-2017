use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();

    let mut dancers = [b'a'; 16];
    for i in 0..16 {
        dancers[i] += i as u8;
    }

    let cmds = stdin.lock().split(b',');
    for cmd in cmds {
        let cmd = cmd.unwrap();
        let cmd = {
            // Drop trailing newline if present
            match cmd[..].iter().position(|ch| *ch == b'\n') {
                Some(n) => &cmd[..n],
                None => &cmd[..],
            }
        };

        match cmd[0] {
            b's' => {
                let x = std::str::from_utf8(&cmd[1..]).unwrap().parse().unwrap();
                dancers[..].rotate_right(x);
            },
            b'x' => {
                let split = cmd[..].iter().position(|ch| *ch == b'/').unwrap();
                let x = std::str::from_utf8(&cmd[1..split]).unwrap().parse().unwrap();
                let y = std::str::from_utf8(&cmd[split+1..]).unwrap().parse().unwrap();
                dancers[..].swap(x, y);
            },
            b'p' => {
                let split = cmd[..].iter().position(|ch| *ch == b'/').unwrap();
                let x = dancers.iter().position(|ch| *ch == cmd[split - 1]).unwrap();
                let y = dancers.iter().position(|ch| *ch == cmd[split + 1]).unwrap();

                dancers[..].swap(x, y);
            },
            _ => panic!(),
        }
    }

    for dancer in &dancers {
        print!("{}", *dancer as char);
    }
    println!("");
}
