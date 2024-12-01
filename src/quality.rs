// Calculate average quality of a sequence; supports Phred+33 and
// Phred+64 encodings.

pub fn average_quality(quality: &[u8], quality_encoding: &str) -> f32 {
    let offset = match quality_encoding {
        "Phred+33" => 33,
        "Phred+64" => 64,
        _ => 33, // Default to Phred+33 if unknown encoding
    };

    let sum: i32 = quality.iter().map(|&q| (q as i32 - offset)).sum();
    let count = quality.len() as i32;

    if count > 0 {
        sum as f32 / count as f32
    } else {
        0.0
    }
}
