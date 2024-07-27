#[cfg(test)]
mod tests {
    #[test]
    fn test_fail() {
        assert_eq!(1, 2);
    }

    #[test]
    fn test_ok() {
        assert_eq!(1, 1);
    }
}
