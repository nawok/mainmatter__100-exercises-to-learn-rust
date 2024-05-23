trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
        assert!(0.is_even());
        assert!(!(-1).is_even());
    }
}
