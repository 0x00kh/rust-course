pub fn print_chars() {
    for ch in b'A'..=b'z' {
        print!("{} ", ch as char);
    }
}
