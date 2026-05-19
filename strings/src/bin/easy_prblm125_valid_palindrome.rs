use std::io;

fn main() {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    println!("Entered value: {}",input);
    println!("{}", prblm125_valid_palindrome(&input));

}

fn prblm125_valid_palindrome(s: &str) -> bool {

    let clean : String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    
    let chars: Vec<char> = clean.chars().collect();

    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
} 