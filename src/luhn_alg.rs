pub fn luhn(cc_number: &str) -> bool {
    // remove spaces
    let number = cc_number.replace(" ", "");
    if number.len() < 2 {
        return false;
    }
    let mut sum = 0;

    // double every second digit from right to left
    for (i, chr) in number.chars().rev().enumerate() {
        if chr.is_digit(10) {
            let mut y: u32;
            y = chr.to_digit(10).unwrap();
            if i % 2 == 0 {
                sum += y;
            } else {
                y = y * 2;
                sum += y / 10 + y % 10;
            }
        } else {
            return false;
        }
    }
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

pub fn exercise8() {
    let cc_number = "4223 9826 4026 9299";
    let result = luhn(cc_number);
    println!("Result of lune: {}", result);
}
