use std::io;
use std::collections::*;
use std::cmp::*;

fn main(){
    let mut s = String::new();

    io::stdin()
    .read_line(&mut s)
    .expect("Error at input");

    println!("output is : {}", max_length_between_equal_characters(&s))
}

fn max_length_between_equal_characters(s :&str) -> i16 {
    let mut ans = -1;
    let mut map: HashMap<char, usize> = HashMap::new();
    for (i, ch) in s.chars().enumerate() {
        if let Some(&val) = map.get(&ch){
            ans = max(ans, (i-val) as i16);
        }else{
            map.insert(ch,i+1);
        }
    }
    return ans;
}