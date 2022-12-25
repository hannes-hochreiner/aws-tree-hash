pub fn calculate_tree_hash(data: &[u8]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_data() {
        assert_eq!(
            calculate_tree_hash(&[0; 7680000]),
            String::from("7a43777ddc7a0326d36b15bc482e6c7736e1c2e9d80a647e8c301646f6a4785c")
        );
    }
}
