use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();

    println!("output is : {}", reverse_bit(n));
}
fn reverse_bit(n: i32)->i32 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut ans = 0;
    for i in 0..32 {
        let bit = (n>>i) & 1;
            ans = (ans<<1) | bit;
    }
    return ans;
}