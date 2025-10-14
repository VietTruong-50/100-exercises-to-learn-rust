fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn !"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about futures!");
    }
}
