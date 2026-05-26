use std::io;
use std::collections::HashMap;

fn main() {
    
    let mut s = String::new();
    let mut t = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("check first input error");
    
    io::stdin()
        .read_line(&mut t)
        .expect("check second input error");
    
    println!("result : {}", is_anagram(&s , &t));
}

// fn is_anagram (s : &str, t: &str) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }

//     let mut h1: HashMap<char, i32> = HashMap::new();
//     let mut h2: HashMap<char, i32> = HashMap::new();

//     for c1 in s.chars() {
//         let count = h1.entry(c1).or_insert(0);
//         *count += 1;
//     }

//     for c2 in t.chars(){
//         let count = h2.entry(c2).or_insert(0);
//         *count += 1;
//     }

//     return h1 == h2;
// }

fn is_anagram (s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map : HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t.chars() {
        let count = map.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    true
}