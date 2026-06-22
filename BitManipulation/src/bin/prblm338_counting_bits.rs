use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    println!("output is : {:?}", couting_bits(n))
}

fn couting_bits(n:i32)->Vec<i32> {
    let mut arr:Vec<i32> = Vec::new();
    for i in 0..n{
        let mut count = 0;
        for j in 0..32 {
            if (i>>j) & 1 == 1 {
                count += 1;
            }
        }
        arr.push(count);
    }
    return arr;
}