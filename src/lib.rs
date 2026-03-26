pub fn greeting() -> &'static str {
    "hello from knope-playground!"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(greeting(), "hello from knope-playground!");
    }
}
