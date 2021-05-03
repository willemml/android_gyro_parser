// 0, 1 or 2
const VARIABLE_NUM: u8 = 2;

fn main() {
    let mut input = String::new();
    let mut is_next = false;
    let mut count: u8 = 0;
    while std::io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        if is_next && count == VARIABLE_NUM {
            println!("{}", trimmed.trim_end_matches(','));
            count = 0;
            is_next = false;
        } else if trimmed.chars().last() == Some('[') {
            is_next = true
        } else {
            count = count + 1;
        }
        input.clear();
    }
}
