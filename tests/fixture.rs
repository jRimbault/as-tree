use std::path::Path;

/// Load fixture input and expected output from text files in tests/fixture/.
pub fn load(name: &str) -> (String, String) {
    let dir = Path::new("tests/fixture");
    let input = std::fs::read_to_string(dir.join(format!("input/{name}.txt")))
        .unwrap_or_else(|e| panic!("input/{name}.txt: {e}"));
    let expected = std::fs::read_to_string(dir.join(format!("expected/{name}.txt")))
        .unwrap_or_else(|e| panic!("expected/{name}.txt: {e}"));
    (input, expected)
}
