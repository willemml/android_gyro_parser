fn main() {
    let mut input = String::new();
    let mut is_next = false;
    while std::io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        if is_next { 
            println!("{}", trimmed.trim_end_matches(','));
            is_next = false;
        } else if trimmed.chars().last() == Some('[') {
            is_next = true
        }
        input.clear();
    }
}
