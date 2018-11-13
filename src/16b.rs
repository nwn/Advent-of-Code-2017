use std::io;

/*
 * Consider this problem in terms of group theory.
 *
 * The reorder (spin, exchange) operations are orthogonal to the
 * rename (partner) operations; they can be applied sequentially,
 * or interwoven.
 *
 * Moreover, all operations of a certain symmetry can be combined
 * into a single operation. We can therefore reduce all reorder
 * operations into a single reordering and all rename operations
 * into a signle renaming. Therefore all commands can be reduced
 * to two steps per round.
 *
 * A further optimization would be to determine the period p of
 * the reordering and renaming operations. From this, we can get
 * within p rounds of the target number in one round;
 */

fn parse_commands(input: impl io::BufRead) -> ([usize; 16], [u8; 16]) {
    let mut reorder = [0; 16];
    let mut rename = [b'a'; 16];
    for i in 0..16 {
        reorder[i] += i;
        rename[i] += i as u8;
    }

    let cmds = input.split(b',');
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
                reorder[..].rotate_right(x);
            },
            b'x' => {
                let split = cmd[..].iter().position(|ch| *ch == b'/').unwrap();
                let x = std::str::from_utf8(&cmd[1..split]).unwrap().parse().unwrap();
                let y = std::str::from_utf8(&cmd[split+1..]).unwrap().parse().unwrap();
                reorder[..].swap(x, y);
            },
            b'p' => {
                let split = cmd[..].iter().position(|ch| *ch == b'/').unwrap();
                let x = rename.iter().position(|ch| *ch == cmd[split - 1]).unwrap();
                let y = rename.iter().position(|ch| *ch == cmd[split + 1]).unwrap();
                rename[..].swap(x, y);
            },
            _ => panic!(),
        }
    }

    (reorder, rename)
}

fn main() {
	let stdin = io::stdin();

    let (reorder, rename) = parse_commands(stdin.lock());

    let mut new_dancers = [b'a'; 16];
    for i in 0..16 {
        new_dancers[i] += i as u8;
    }

    for _ in 0..1_000_000_000 {
        let old_dancers = new_dancers;
        for i in 0..16 {
            new_dancers[i] = rename[(old_dancers[reorder[i]] - b'a') as usize];
        }
    }

    for dancer in &new_dancers {
        print!("{}", *dancer as char);
    }
    println!("");
}
