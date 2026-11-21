pub fn spaced(s: &str) -> String {
    s.chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\u{200A}")
}
