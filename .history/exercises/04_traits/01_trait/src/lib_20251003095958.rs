// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

/*************  ✨ Windsurf Command ⭐  *************/
/// Test that the `IsEven` trait is implemented correctly for `i32`.
///
/// Checks that even numbers (positive and zero) return `true` and odd numbers (positive and negative) return `false`.
/*******  0517cb91-9ffe-49c1-828f-26308b830dd9  *******/
    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
