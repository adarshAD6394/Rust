use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let arr: Vec<i32> = line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    for num in &arr {
        print!("{} ", num);
    }

    println!("output is : {}",single_number(&arr))
}

fn single_number(arr:&Vec<i32>)->i32{
    let mut ans:i32 = 0;
    for num in arr{
        ans = ans^num;
    }
    return ans;
}