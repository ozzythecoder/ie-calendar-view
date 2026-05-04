use std::collections::HashMap;

pub fn require_field(
    map: &HashMap<String, String>,
    candidates: &[&str],
) -> Result<String, String> {
    for &name in candidates {
        if let Some(val) = map.get(name) {
            return Ok(val.clone());
        }
    }
    println!("[csv::require_field]: no valid field found for candidates: {:?}", candidates);
    Err(From::from("No valid field found"))
}

pub fn optional_field(map: &HashMap<String, String>, candidates: &[&str]) -> Option<String> {
    candidates.iter().find_map(|&name| map.get(name).cloned())
}
