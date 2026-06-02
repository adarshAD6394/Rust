use std::io;

fn main(){

    let mut s = String::new();
    io::stdin()
    .read_line(&mut s)
    .expect("input error");
    let s = s.trim();
    println!("output is : {}", largest_odd_number_in_string(&s));
}

fn largest_odd_number_in_string(s: &str) -> String {
    for (i,ch) in s.char_indices().rev() {
        if ch.to_digit(10).unwrap()%2 != 0 {
            return s.chars().take(i+1).collect();
        }
    }
    return "".to_string();
}