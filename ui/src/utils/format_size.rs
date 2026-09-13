const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
const STEP: f64 = 1024.0;

/// Human readable size with binary steps: `532 B`, `4.21 KB`, `18.3 MB`, `512 GB`.
pub fn format_size(bytes: u64) -> String {
    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= STEP && unit < UNITS.len() - 1 {
        value /= STEP;
        unit += 1;
    }

    if unit == 0 {
        return format!("{bytes} B");
    }

    if value >= 100.0 {
        format!("{value:.0} {}", UNITS[unit])
    } else if value >= 10.0 {
        format!("{value:.1} {}", UNITS[unit])
    } else {
        format!("{value:.2} {}", UNITS[unit])
    }
}

/// The exact byte count with thousands separators: `1,234,567`.
pub fn format_bytes_exact(bytes: u64) -> String {
    let digits = bytes.to_string();
    let mut result = String::with_capacity(digits.len() + digits.len() / 3);

    for (index, digit) in digits.chars().enumerate() {
        let remaining = digits.len() - index;
        if index > 0 && remaining.is_multiple_of(3) {
            result.push(',');
        }
        result.push(digit);
    }

    result
}
