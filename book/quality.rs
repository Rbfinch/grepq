// Calculate average quality of a sequence; supports Phred+33 and
// Phred+64 encodings.

pub fn average_quality(quality: &[u8], quality_encoding: &str) -> f32 {
    // Determine the offset based on the quality encoding
    let offset = match quality_encoding {
        "Phred+33" => 33,
        "Phred+64" => 64,
        _ => 33, // Default to Phred+33 if unknown encoding
    };

    // Initialize sum and count
    let mut sum = 0;
    let count = quality.len() as i32;

    // Calculate the sum of quality scores adjusted by the offset
    for &q in quality {
        sum += q as i32 - offset;
    }

    // Calculate and return the average quality score
    if count > 0 {
        sum as f32 / count as f32
    } else {
        0.0
    }
}
