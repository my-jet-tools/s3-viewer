/// The last segment of an object key: `reports/2026/q3.csv` -> `q3.csv`.
pub fn file_name_of(key: &str) -> &str {
    match key.rfind('/') {
        Some(index) => &key[index + 1..],
        None => key,
    }
}
