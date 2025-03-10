fn get_hello() -> String {
    "Hello World!".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_hello() {
        let result = get_hello();
        assert_eq!(result, "Hello World!");
    }
}
