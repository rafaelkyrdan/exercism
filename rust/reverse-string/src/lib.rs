use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    let mut reversed = String::new();
    for grapheme in input.graphemes(true).rev() {
        reversed.push_str(grapheme);
    }

    return reversed;
}
