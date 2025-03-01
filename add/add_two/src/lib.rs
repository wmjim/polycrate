use rand;
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(1), 3);
        assert_eq!(add_two(5), 7);
        assert_eq!(add_two(-3), -1);
    }
}