use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();

    println!("output is: {}", number_of_bits(n));
}

fn number_of_bits(n : i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut i = 0;
    let mut ans = 0;
    while i<32 {
        if (n>>i) & 1 == 1{
            ans += 1;
        }
        i = i+1;
    } 
    return ans;
}