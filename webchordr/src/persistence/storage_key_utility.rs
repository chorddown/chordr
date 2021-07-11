pub fn build_combined_key<N: AsRef<str>, K: AsRef<str>>(namespace: &N, key: &K) -> String {
    if namespace.as_ref().is_empty() {
        key.as_ref().to_string()
    } else {
        format!("{}.{}", namespace.as_ref(), key.as_ref())
    }
}
