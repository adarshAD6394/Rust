use std::io;
use std::collections::*;

fn main () {

   let mut s = String::new();
   let mut t = String::new();

   io::stdin()
      .read_line(&mut s)
      .expect("failure input 1");
   io::stdin()
      .read_line(&mut t)
      .expect("failure input 2");

   println!("output is :{}", isomorphic_strings(&s,&t));
 
}

fn isomorphic_strings (s :&str, t:&str) -> bool{
 if s.len() != t.len() {
    return false
 }

 let mut map: HashMap<char, char> = HashMap::new();
 let mut map2: HashMap<char, char> = HashMap::new();
 for (i, j) in s.chars().zip(t.chars()) {
    if map.contains_key(&i){
        if map.get(&i) != Some(&j) {
         return false;
        }
    }else{
      map.insert(i, j);
    }
    if map2.contains_key(&j){
        if map2.get(&j) != Some(&i) {
         return false;
        }
    }else{
      map2.insert(j, i);
    }
 }
 return true;
}