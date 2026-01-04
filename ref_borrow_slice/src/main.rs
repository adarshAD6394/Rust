fn main() {
    let string: String = String::from("Hello World!");

    let slice: &str = borrow_slice_example(&string);
    println!("Slice: {}", slice);

}

fn borrow_slice_example(s: &String) -> &str {
    let byte: &[u8] = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b'o' {
            return &s[0..i];
        }
    }

    &s[0..s.len()]
}