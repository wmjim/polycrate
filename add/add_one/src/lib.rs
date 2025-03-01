use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(-3), -2);
    }
}