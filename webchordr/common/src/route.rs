pub fn route<S: AsRef<str>>(route: S) -> String {
    format!("/#/{}", route.as_ref().trim_start_matches('/'))
}
