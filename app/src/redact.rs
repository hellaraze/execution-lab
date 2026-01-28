pub fn redact(s: &str) -> String {
    if s.is_empty() {
        return "<empty>".to_string();
    }
    // Keep only length info; never leak raw content.
    format!("<redacted:{}>", s.len())
}
