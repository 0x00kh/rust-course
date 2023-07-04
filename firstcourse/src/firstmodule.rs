pub fn print_chars() {
    for ch in (b'Z'..=b'a').rev() {
        print!("{} ", ch as char);
    }
}