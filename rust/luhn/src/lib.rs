/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");

    if code.len() < 2 {
        return false;
    }

    if !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = code
        .chars()
        .rev() // Process from right to left
        .enumerate()
        .map(|(i, c)| {
            let mut digit = c.to_digit(10).unwrap();

            // Double every second digit (0-indexed, so odd positions)
            if i % 2 == 1 {
                digit *= 2;
                // If result is > 9, subtract 9
                if digit > 9 {
                    digit -= 9;
                }
            }

            digit
        })
        .sum();

    sum % 10 == 0
}
