#[allow(dead_code)]
pub fn center_text(text: &str, width: usize, pad_with: char) -> String {
    if text.len() > width {
        panic!("Text to be centered cannot be longer than the width.");
    }

    let total_padding = width - text.len();
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    format!(
        "{}{}{}",
        pad_with.to_string().repeat(left_padding),
        text,
        pad_with.to_string().repeat(right_padding)
    )
}
