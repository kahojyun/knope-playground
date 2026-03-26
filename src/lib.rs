pub fn greeting() -> &'static str {
    "hello from the knope playground!"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(greeting(), "hello from the knope playground!");
    }
}
