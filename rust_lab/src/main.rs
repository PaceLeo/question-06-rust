fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    println!("Hello, Rust! 2 + 3 = {}", add(2, 3));

    for round in 1..=5 {
        let dice: u8 = rand::random_range(1..=6);
        println!("第 {round} 次掷骰子：{dice}");
    }
}
#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adding_zero_keeps_value() {
        assert_eq!(add(7, 0), 7);
    }
}
