#[cfg(test)]
mod tests {
    extern crate calc;

    #[test]
    fn sum_test() {
        assert_eq!(4, calc::sum(2, 2));
        assert_eq!(10, calc::sum(8, 2));
    }

    #[test]
    fn sub_test() {
        assert_eq!(0, calc::sub(2, 2));
        assert_eq!(6, calc::sub(8, 2));
    }

    #[test]
    fn div_test() {
        assert_eq!(1, calc::div(2, 2));
        assert_eq!(4, calc::div(8, 2));
    }

    #[test]
    fn mul_test() {
        assert_eq!(4, calc::mul(2, 2));
        assert_eq!(16, calc::mul(8, 2));
    }
}
