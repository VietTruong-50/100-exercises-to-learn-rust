fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to lear!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about futures!");
    }
}
