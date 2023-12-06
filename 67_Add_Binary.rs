pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();
    loop {
        match (a.next(), b.next()) {
            (Some(x), Some(y)) => {
                let sum = x.to_digit(10).unwrap() + y.to_digit(10).unwrap() + carry;
                result.push_str(&(sum % 2).to_string());
                carry = sum / 2;
            }
            (Some(x), None) => {
                let sum = x.to_digit(10).unwrap() + carry;
                result.push_str(&(sum % 2).to_string());
                carry = sum / 2;
            }
            (None, Some(y)) => {
                let sum = y.to_digit(10).unwrap() + carry;
                result.push_str(&(sum % 2).to_string());
                carry = sum / 2;
            }
            (None, None) => {
                if carry == 1 {
                    result.push_str(&carry.to_string());
                }
                break;
            }
        }
    }
    result.chars().rev().collect::<String>()
}

fn main() {
    let a = String::from("111");
    let b = String::from("1");
    let result = add_binary(a, b);
    println!("{:?}", result);
}
