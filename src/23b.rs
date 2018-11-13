// The assembly amounts to counting the composite numbers
// between b and c, stepping by 17.
fn main() {
    let mut h = 0;
    let mut b = 106700;
    let c = b + 17000;
    while b <= c {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }

        b += 17;
    }

    println!("{}", h);
}
