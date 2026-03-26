pub fn greeting() -> &'static str {
    "hello from the validated knope playground release flow!"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(
            greeting(),
            "hello from the validated knope playground release flow!"
        );
    }
}
