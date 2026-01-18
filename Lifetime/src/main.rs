// fn main() {
//     let r ;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r);  // error because x does not live long enough borrow checker will remmove him.


//     // Correct will be 
//     let r;
//     let x = 5;
//     r = &x;
//     println!("r: {}", r);  // ok because x lives long enough
// }

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// 'a does not create a lifetime but it is a generic lifetime parameter to define realitonship between input and output references. 
// time for str will be smallest form either of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
