fn main () {
    let a = Some(5);
    let b = plus_one(a);
    let none = plus_one(None);
    
    if let Some(3) = some_value {
        println!("three");
    }


}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1)
//     }
// }

// if let syntax for small expressions 

if let some