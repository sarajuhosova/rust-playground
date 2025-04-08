fn fibonacci(num: u32) -> Vec<u32> {
    match num {
        0 => vec![0],
        1 => vec![0, 1],
        n => {
            let mut previous = fibonacci(n - 1).to_owned();
            let next = previous[previous.len() - 1] + previous[previous.len() - 2];
            previous.push(next);
            previous
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), vec![0]);
        assert_eq!(fibonacci(1), vec![0, 1]);
        assert_eq!(fibonacci(2), vec![0, 1, 1]);
        assert_eq!(fibonacci(3), vec![0, 1, 1, 2]);
        assert_eq!(fibonacci(4), vec![0, 1, 1, 2, 3]);
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3, 5]);
        assert_eq!(fibonacci(13), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]);
    }
}
